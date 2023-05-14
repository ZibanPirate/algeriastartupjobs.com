use super::model::SearchRecord;
use crate::{
  _utils::{error::SearchError, string::escape_double_quote},
  config::service::ConfigService,
  post::model::Post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Serialize, Deserialize)]
struct WordIn {
  model_id: u32,
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
        appear_in,
      });
      return;
    }
  }

  word_indexes.push(WordIndex {
    word: word.to_string(),
    r#in: vec![WordIn {
      model_id,
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

  // @TODO-ZM: add pagination
  pub async fn search_posts(&self, query: &String) -> Result<Vec<u32>, SearchError> {
    let query = format!(
      r#"
      SELECT math::sum(score) as score, in.model_id as id FROM (
        SELECT in, word, (
            IF in.appear_in="post_title" THEN
              100
            ELSE IF in.appear_in="post_short_description" THEN
              25
            ELSE IF in.appear_in="post_tags" THEN
              5
            ELSE
              1
            END
          ) as score FROM (
          SELECT count() as count, in.model_id, in.appear_in, word FROM (
            SELECT in, id.word as word FROM word WHERE id.word IN [{}] SPLIT in
          ) GROUP BY in.model_id, in.appear_in, word
        )
      ) GROUP BY id ORDER BY score NUMERIC DESC LIMIT 100 START 0;
      "#,
      query
        .split(" ")
        .map(|word| format!(r#""{}""#, escape_double_quote(&word.to_lowercase())))
        .collect::<Vec<String>>()
        .join(", ")
    );

    let query_result = self.search_db.query(&query).await;

    match query_result {
      Ok(mut query_result) => {
        let record: Result<Vec<SearchRecord>, _> = query_result.take(0);

        match record {
          Ok(record) => Ok(record.iter().map(|r| r.id).collect()),
          Err(err) => {
            tracing::error!("Failed to parse the query result, {}", err);
            Err(SearchError::InternalError)
          }
        }
      }
      Err(err) => {
        tracing::error!("Failed to search the query, {}", err);
        Err(SearchError::InternalError)
      }
    }
  }
}
