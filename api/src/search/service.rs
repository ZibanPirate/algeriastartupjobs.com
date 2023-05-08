use std::sync::Arc;

use crate::{
  _utils::{error::SearchError, string::escape_double_quote},
  config::service::ConfigService,
  post::model::Post,
};

use super::model::SearchResult;

pub struct SearchService {
  pub config_service: Arc<ConfigService>,
}

impl SearchService {
  pub async fn setup_search(&self) {
    let client = reqwest::Client::new();
    let res = client
      .post(format!(
        "{}/api/v1/indexes",
        self.config_service.get_config().search_url
      ))
      .header("content-type", "application/yaml")
      .body(
        r#"
        version: 0.5

        index_id: posts

        doc_mapping:
          field_mappings:
            - name: id
              type: u64
            - name: title
              type: text
              tokenizer: default
              record: position
            - name: short_description
              type: text
              tokenizer: default
              record: position
            - name: description
              type: text
              tokenizer: default
              record: position
            - name: poster_id
              type: u64
            - name: category_id
              type: u64
              # add tag_ids
          tag_fields: [poster_id, category_id]

        search_settings:
          default_search_fields: [title, description]
        "#,
      )
      .send()
      .await;

    if res.is_err() {
      tracing::error!("Failed to setup the search: {}", res.err().unwrap());
      return;
    }
    let res = res.unwrap();

    if !res.status().is_success() {
      tracing::error!(
        "Failed to setup the search: {}, body: {}",
        res.status(),
        res.text().await.unwrap()
      );
    }
    tracing::info!("Post index created");
  }

  pub async fn index_posts(&self, posts: Vec<Post>) -> Result<(), SearchError> {
    let posts_in_ndjson_format = posts
      .iter()
      .map(|post| {
        serde_json::to_string(&post)
          .unwrap()
          .replace("\n", "")
          .replace("\r", "")
      })
      .collect::<Vec<String>>()
      .join("\n");

    let client = reqwest::Client::new();
    let res = client
      .post(format!(
        "{}/api/v1/posts/ingest",
        self.config_service.get_config().search_url
      ))
      .body(posts_in_ndjson_format)
      .send()
      .await;

    if res.is_err() {
      tracing::error!("Failed to index the posts: {}", res.err().unwrap());
      return Err(SearchError::InternalError);
    }
    let res = res.unwrap();

    if !res.status().is_success() {
      tracing::error!(
        "Failed to index the posts: {}, body: {}",
        res.status(),
        res.text().await.unwrap()
      );
      return Err(SearchError::InternalError);
    }

    tracing::debug!("result of indexing: {}", res.text().await.unwrap());

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
