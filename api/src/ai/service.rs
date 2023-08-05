use serde::Deserialize;
use std::sync::Arc;

use crate::{
  _utils::{
    error::AIError,
    string::{escape_double_quote, escape_new_line},
  },
  config::service::ConfigService,
};

#[derive(Deserialize)]
pub struct PostToSuggestTagsFor {
  title: String,
  description: String,
}

#[derive(Deserialize)]
pub struct AIResponseChoiceMessage {
  content: String,
}

#[derive(Deserialize)]
pub struct AIResponseChoice {
  message: AIResponseChoiceMessage,
}

#[derive(Deserialize)]
pub struct AIResponse {
  choices: Vec<AIResponseChoice>,
}

pub struct AIService {
  config_service: Arc<ConfigService>,
}

impl AIService {
  pub fn new(config_service: Arc<ConfigService>) -> Self {
    Self { config_service }
  }

  pub async fn suggest_tags_for_post(
    self,
    post: PostToSuggestTagsFor,
  ) -> Result<Vec<String>, AIError> {
    let client = reqwest::Client::new();

    let ai_response_string = client
      .post("https://api.openai.com/v1/chat/completions")
      .header("content-type", "application/json")
      .header("accept", "application/json")
      .header(
        "Authorization",
        self.config_service.get_config().ai_service_auth_token,
      )
      .body(format!(
        r#"{{
          "model": "gpt-3.5-turbo",
          "messages": [
            {{
              "role": "system",
              "content": "You will be provided with an Algerian job post description, and your task is to generate 10 relevant keywords in English about skills needed, in this format: [keyword1|keyword2|...|keyword10]"
            }},
            {{
              "role": "user",
              "content": "job title: {}"
            }},
            {{
              "role": "user",
              "content": "job description: {}"
            }}
          ],
          "temperature": 0.3,
          "max_tokens": 256,
          "top_p": 1,
          "frequency_penalty": 0.25,
          "presence_penalty": 0
        }}"#,
        escape_new_line(&escape_double_quote(&post.title)),
        escape_new_line(&escape_double_quote(&post.description)),
      ))
      .send()
      .await;
    if ai_response_string.is_err() {
      tracing::error!(
        "Failed to talk to AI: {}",
        ai_response_string.err().unwrap()
      );
      return Err(AIError::InternalError);
    }
    let ai_response_string = ai_response_string.unwrap();
    let ai_response = serde_json::from_str::<AIResponse>(&ai_response_string.text().await.unwrap());
    if ai_response.is_err() {
      // @TODO-ZM: log the response
      return Err(AIError::InternalError);
    }
    let ai_response = ai_response.unwrap();

    let string_result = ai_response.choices.first();
    if string_result.is_none() {
      return Ok(vec![]);
    }
    let string_result = &string_result.unwrap().message.content;

    // extract all keywords where string_result is: [keyword1|keyword2|keyword3|...]
    let keywords = string_result
      .split('[')
      .nth(1)
      .unwrap()
      .split(']')
      .nth(0)
      .unwrap()
      .split('|')
      .map(|s| s.trim().to_string())
      .collect::<Vec<String>>();

    Ok(keywords)
  }
}
