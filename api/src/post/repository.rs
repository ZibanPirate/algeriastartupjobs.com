use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::_utils::{
  database::{db_thing_to_id, DBRecord},
  error::DataAccessError,
  string::escape_single_quote,
};

use super::{
  mocks::generate_posts_seed,
  model::{DBPost, Post},
};

pub struct PostRepository {
  pub db: Arc<Surreal<Client>>,
}

impl PostRepository {
  pub fn get_all_posts(&self) -> Result<Vec<Post>, DataAccessError> {
    Ok(generate_posts_seed())
  }

  pub fn get_post_by_id(&self, id: i32) -> Result<Post, DataAccessError> {
    let posts = generate_posts_seed();
    for post in posts {
      if post.id == id {
        return Ok(post);
      }
    }
    Err(DataAccessError::NotFound)
  }

  pub fn get_many_similar_posts_by_id(&self, id: i32) -> Result<Vec<Post>, DataAccessError> {
    let posts = generate_posts_seed();
    let current_post = self.get_post_by_id(id).unwrap();
    let mut similar_posts = Vec::new();
    for post in posts {
      if post.id != id
        && post
          .tag_ids
          .iter()
          .any(|tag_id| current_post.tag_ids.contains(tag_id))
      {
        similar_posts.push(post);
      }
    }

    Ok(similar_posts)
  }

  pub async fn create_one_post(&self, post: DBPost) -> Result<u32, DataAccessError> {
    let query = format!(
      r#"
      BEGIN TRANSACTION;

      LET $count = (SELECT count() FROM post GROUP BY count)[0].count || 0;

      CREATE post:{{ id: $count }} CONTENT {{
        slug: '{}',
        title: '{}',
        poster_id: {},
        short_description:'{}',
        description:'{}',
        category_id:'{}',
        tag_ids:[{}],
      }};

      COMMIT TRANSACTION;
      "#,
      escape_single_quote(&post.slug),
      escape_single_quote(&post.title),
      escape_single_quote(&post.poster_id.to_string()),
      escape_single_quote(&post.short_description),
      escape_single_quote(&post.description),
      escape_single_quote(&post.category_id.to_string()),
      post
        .tag_ids
        .iter()
        .map(|tag_id| escape_single_quote(&tag_id.to_string()))
        .collect::<Vec<String>>()
        .join(", ")
    );

    let query_result = self.db.query(&query).await;
    match query_result {
      Ok(mut query_result) => {
        let record: Result<Option<DBRecord>, _> = query_result.take(1);

        match record {
          Ok(record) => match record {
            Some(record) => {
              let id = db_thing_to_id(&record.id);
              match id {
                Some(id) => return Ok(id),
                None => {
                  tracing::error!("failed to get created post id {:?}", record);
                  return Err(DataAccessError::InternalError);
                }
              }
            }
            None => {
              tracing::error!("failed to get created post record {:?}", record);
              return Err(DataAccessError::InternalError);
            }
          },
          Err(e) => {
            tracing::error!("failed to get created post record {:?}", e);
            return Err(DataAccessError::InternalError);
          }
        }
      }
      Err(e) => {
        tracing::error!("failed to create post {:?}, query {:?}", e, &query);
        return Err(DataAccessError::CreationError);
      }
    }
  }
}
