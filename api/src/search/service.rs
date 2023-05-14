use super::model::SearchRecord;
use crate::{
  _utils::{
    error::SearchError,
    string::{escape_double_quote, get_words},
  },
  post::model::Post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Serialize, Deserialize)]
struct WordIndex {
  word: String,
  model_id: u32,
  appear_in: String,
}

pub struct SearchService {
  search_db: Arc<Surreal<Client>>,
}

impl SearchService {
  pub fn new(search_db: Arc<Surreal<Client>>) -> Self {
    Self { search_db }
  }

  pub async fn index_posts(&self, posts: Vec<Post>) -> Result<(), SearchError> {
    let mut word_indexes: Vec<WordIndex> = vec![];
    for post in posts {
      // @TODO-ZM: use regex \d to split the string
      get_words(&post.title).for_each(|word| {
        let word = word.to_lowercase();

        word_indexes.push(WordIndex {
          word,
          model_id: post.id,
          appear_in: "post_title".to_string(),
        });
      });

      get_words(&post.short_description).for_each(|word| {
        let word = word.to_lowercase();

        word_indexes.push(WordIndex {
          word,
          model_id: post.id,
          appear_in: "post_short_description".to_string(),
        });
      });

      // @TODO-ZM: add post description to the index
      // @TODO-ZM: populate tags by tag_ids and index them.
      // @TODO-ZM: populate poster by poster_id and index it.
      // @TODO-ZM: populate category by category_id and index it.
    }

    let word_indexes_length = word_indexes.len();
    for index in 0..word_indexes_length {
      let word_index = &word_indexes[index];

      let query = format!(
        r#"CREATE word CONTENT {{
          {}
        }}"#,
        serde_json::to_string(&word_index).unwrap()
      );

      let res = self.search_db.query(&query).await;

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
      SELECT math::sum(score) as score, model_id as id FROM (
        SELECT model_id, ((
            IF appear_in="post_title" THEN
              100
            ELSE IF appear_in="post_short_description" THEN
              25
            ELSE IF appear_in="post_tags" THEN
              5
            ELSE
              1
            END
          ) * count) as score FROM (
          SELECT count() as count, word, model_id, appear_in FROM word WHERE word IN [{}] GROUP BY word, model_id, appear_in
        )
      ) GROUP BY id ORDER BY score NUMERIC DESC LIMIT 10 START 0;
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
