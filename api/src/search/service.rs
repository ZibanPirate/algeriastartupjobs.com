use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
  _utils::{error::SearchError, string::escape_double_quote},
  config::service::ConfigService,
  post::model::Post,
};

use super::model::SearchResult;

#[derive(Debug, Serialize, Deserialize)]
struct WordIn {
  model_id: u32,
  model_name: String,
  appear_in: String,
}

#[derive(Debug)]
struct WordIndex {
  word: String,
  r#in: Vec<WordIn>,
}

fn add_word_appearance_to_word_indexes(
  word: &String,
  word_indexes: &mut Vec<WordIndex>,
  model_id: u32,
  appear_in: String,
) {
  for word_index in word_indexes.iter_mut() {
    if word_index.word == *word {
      word_index.r#in.push(WordIn {
        model_id,
        model_name: "post".to_string(),
        appear_in,
      });
      return;
    }
  }

  word_indexes.push(WordIndex {
    word: word.to_string(),
    r#in: vec![WordIn {
      model_id,
      model_name: "post".to_string(),
      appear_in,
    }],
  });
}

pub struct SearchService {
  search_db: Arc<Surreal<Client>>,
  config_service: Arc<ConfigService>,
}

impl SearchService {
  pub fn new(config_service: Arc<ConfigService>, search_db: Arc<Surreal<Client>>) -> Self {
    Self {
      config_service,
      search_db,
    }
  }

  pub async fn index_posts(&self, posts: Vec<Post>) -> Result<(), SearchError> {
    let mut word_indexes: Vec<WordIndex> = vec![];
    for post in posts {
      // @TODO-ZM: use regex \d to split the string
      post.title.split(" ").for_each(|word| {
        let word = word.to_lowercase();

        add_word_appearance_to_word_indexes(
          &word,
          &mut word_indexes,
          post.id,
          "post_title".to_string(),
        );
      });

      post.short_description.split(" ").for_each(|word| {
        let word = word.to_lowercase();

        add_word_appearance_to_word_indexes(
          &word,
          &mut word_indexes,
          post.id,
          "post_short_description".to_string(),
        );
      });

      // @TODO-ZM: add post description to the index
      // @TODO-ZM: populate tags by tag_ids and index them.
      // @TODO-ZM: populate poster by poster_id and index it.
      // @TODO-ZM: populate category by category_id and index it.
    }

    let word_indexes_length = word_indexes.len();
    for index in 0..word_indexes_length {
      let word_index = &word_indexes[index];
      let word = &word_index.word;
      let appear_in = serde_json::to_string(&word_index.r#in).unwrap();

      let query = format!(
        r#"
        INSERT INTO word [ {{ id: {{ word: "{}" }}, in: {} }} ]
        ON DUPLICATE KEY UPDATE in+={}
        "#,
        escape_double_quote(&word),
        appear_in,
        appear_in
      );

      let res = self.search_db.query(&query).await;

      tracing::info!(
        "Indexing {} words... {:.2}%",
        word_indexes_length,
        ((index + 1) as f32 / word_indexes_length as f32) * 100.0,
      );

      if res.is_err() {
        tracing::error!("Failed to index the word: {}", res.err().unwrap());
        return Err(SearchError::InternalError);
      }
    }

    Ok(())
  }

  pub async fn search_posts(&self, query: &String) -> Result<Vec<u32>, SearchError> {
    let client = reqwest::Client::new();
    // TODO-ZM: return only ids
    let res = client
      .post(format!(
        "{}/api/v1/posts/search",
        self.config_service.get_config().search_url
      ))
      .header("content-type", "application/json")
      .body(format!(
        r#"{{
          "query": "{}"
        }}"#,
        escape_double_quote(&query)
      ))
      .send()
      .await;

    if res.is_err() {
      tracing::error!("Failed to perform the search: {}", res.err().unwrap());
      return Err(SearchError::InternalError);
    }
    let res = res.unwrap();

    if !res.status().is_success() {
      tracing::error!(
        "Failed to perform the search: {}, body: {}",
        res.status(),
        res.text().await.unwrap()
      );
      return Err(SearchError::InternalError);
    }
    let res = res.json::<SearchResult>().await;

    if res.is_err() {
      tracing::error!("Failed to parse the search result: {}", res.err().unwrap());
      return Err(SearchError::InternalError);
    }
    let res = res.unwrap();

    let record_ids = res.hits.iter().map(|hit| hit.id).collect::<Vec<u32>>();

    // deduplicate ids
    let record_ids = record_ids
      .iter()
      .cloned()
      .collect::<std::collections::HashSet<u32>>()
      .into_iter()
      .collect::<Vec<u32>>();

    Ok(record_ids)
  }
}
