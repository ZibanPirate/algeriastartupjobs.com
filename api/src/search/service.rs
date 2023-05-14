use super::model::SearchRecord;
use crate::{
  _utils::{
    error::SearchError,
    string::{escape_double_quote, get_searchable_words, get_words},
  },
  post::model::Post,
};
use bk_tree::{metrics, BKTree};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use surrealdb::{engine::remote::ws::Client, Surreal};
#[derive(Debug, Serialize, Deserialize)]
struct WordIndex {
  word: String,
  model_id: u32,
  appear_in: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WordRecord {
  word: String,
}

pub struct SearchService {
  search_db: Arc<Surreal<Client>>,
  bk_tree: Arc<Mutex<BKTree<String>>>,
}

impl SearchService {
  pub fn new(search_db: Arc<Surreal<Client>>) -> Self {
    Self {
      search_db,
      bk_tree: Arc::new(Mutex::new(BKTree::new(metrics::Levenshtein))),
    }
  }

  pub async fn refresh_bk_tree(&self) -> Result<(), SearchError> {
    let query = r#"
      SELECT word from word WHERE appear_in IN ["post_title", "post_short_description"] GROUP BY word
    "#.to_string();

    let query_result = self.search_db.query(&query).await;

    match query_result {
      Ok(mut query_result) => {
        let records: Result<Vec<WordRecord>, _> = query_result.take(0);

        match records {
          Ok(records) => {
            let words_count = records.len();
            let mut bk_tree = self.bk_tree.lock().unwrap();
            // @TODO-ZM: clean the tree before adding new words

            for word_index in records {
              bk_tree.add(word_index.word);
            }

            tracing::info!("BK-Tree refreshed with {} words", words_count);
          }
          Err(err) => {
            tracing::error!("Failed to parse the query result, {}", err);
            return Err(SearchError::InternalError);
          }
        }
      }
      Err(err) => {
        tracing::error!("Failed to get words for bk-tree, {}", err);
        return Err(SearchError::InternalError);
      }
    }

    Ok(())
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

  fn get_corrected_queries(&self, query: &String, max_suggestions: u8) -> Vec<String> {
    let query_words = get_searchable_words(query);

    let bk_tree = self.bk_tree.lock().unwrap();
    let mut corrected_words_in_queries = vec![];
    for query_word in &query_words {
      let mut corrected_words_with_distance = vec![];

      let tolerance = query_word.len() / 2;
      bk_tree
        .find(query_word, tolerance as u32)
        .for_each(|(distance, corrected_word)| {
          corrected_words_with_distance.push((distance, corrected_word));
        });

      corrected_words_with_distance.sort_by_key(|k| k.0);

      corrected_words_in_queries.push(
        corrected_words_with_distance
          .iter()
          .map(|(_, corrected_word)| corrected_word)
          .map(|corrected_word| escape_double_quote(corrected_word))
          .collect::<Vec<String>>(),
      );
    }

    let mut corrected_queries = vec![];

    for index in 0..max_suggestions {
      let mut corrected_query_words = vec![];
      for query_word_index in 0..query_words.len() {
        let corrected_word = corrected_words_in_queries
          .get(query_word_index)
          .unwrap()
          .get(index as usize);

        match corrected_word {
          Some(corrected_word) => corrected_query_words.push(corrected_word.as_str()),
          None => corrected_query_words.push({
            match query_words.get(query_word_index) {
              Some(query_word) => query_word,
              None => "",
            }
          }),
        }
      }
      corrected_queries.push(corrected_query_words.join(" "));
    }

    // filter out the original query
    let corrected_queries = corrected_queries
      .iter()
      .filter(|corrected_query| *corrected_query != query)
      .map(|corrected_query| corrected_query.to_string())
      .collect::<Vec<String>>();

    corrected_queries
  }

  fn generate_search_query(&self, query: &String) -> String {
    let search_query = format!(
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

    search_query
  }

  // @TODO-ZM: add pagination
  pub async fn search_posts(&self, query: &String) -> Result<Vec<u32>, SearchError> {
    let corrected_queries = self.get_corrected_queries(query, 1);

    let mut search_queries = corrected_queries
      .iter()
      .map(|corrected_query| self.generate_search_query(corrected_query))
      .collect::<Vec<String>>();

    search_queries.insert(0, self.generate_search_query(query));

    let mut search_records: Vec<SearchRecord> = vec![];

    let search_queries_len = search_queries.len();
    for index in 0..search_queries_len {
      let search_query = &search_queries[index];

      let query_result = self.search_db.query(search_query).await;

      match query_result {
        Ok(mut query_result) => {
          let found_search_records: Result<Vec<SearchRecord>, _> = query_result.take(0);

          match found_search_records {
            Ok(found_search_records) => {
              search_records.extend(found_search_records.iter().map(|r| SearchRecord {
                id: r.id,
                score: r.score * (search_queries_len as u32 - index as u32),
              }));
            }
            Err(err) => {
              tracing::error!("Failed to parse the query result, {}", err);
              return Err(SearchError::InternalError);
            }
          }
        }
        Err(err) => {
          tracing::error!("Failed to search the query, {}", err);
          return Err(SearchError::InternalError);
        }
      }
    }

    let search_records = search_records
      .into_iter()
      .sorted_by_key(|record| record.id)
      .group_by(|record| record.id)
      .into_iter()
      .map(|(id, group)| {
        let score = group.fold(0, |acc, record| acc + record.score);
        SearchRecord { id, score }
      })
      .collect::<Vec<_>>();

    Ok(search_records.iter().map(|r| r.id).collect())
  }
}
