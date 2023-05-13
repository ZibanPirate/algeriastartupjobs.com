use axum::{extract::State, http::header::HeaderMap, response::IntoResponse, Json, Router};
use fake::Fake;
use hyper::StatusCode;
use serde_json::json;

use crate::{
  _entry::state::AppState,
  _utils::{is_admin::is_admin, string::slugify},
  account::model::{AccountType, DBAccount},
  category::model::DBCategory,
  post::model::DBPost,
  tag::model::DBTag,
  task::model::{DBTask, TaskName, TaskStatus, TaskType},
};

pub async fn seed_the_database_with_mocks(
  State(app_state): State<AppState>,
  headers: HeaderMap,
) -> impl IntoResponse {
  if is_admin(&app_state, headers).is_none() {
    return StatusCode::UNAUTHORIZED.into_response();
  }

  let mut account_ids: Vec<u32> = [].to_vec();
  for index in 0..9 {
    let company_name = fake::faker::company::en::CompanyName().fake::<String>();
    let slug = slugify(&company_name);
    let account_id = app_state
      .account_repository
      .create_one_account(DBAccount {
        email: format!("test+{}.{}@algeriastartupjobs.com", slug, index),
        slug,
        r#type: AccountType::Company { company_name },
      })
      .await;
    match account_id {
      Ok(account_id) => {
        account_ids.push(account_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  for index in 10..19 {
    let first_name = fake::faker::name::en::FirstName().fake::<String>();
    let last_name = fake::faker::name::en::LastName().fake::<String>();
    let slug = slugify(&format!("{}_{}", first_name, last_name));
    let account_id = app_state
      .account_repository
      .create_one_account(DBAccount {
        email: format!("test+{}.{}@algeriastartupjobs.com", slug, index),
        slug,
        r#type: AccountType::Individual {
          first_name,
          last_name,
        },
      })
      .await;
    match account_id {
      Ok(account_id) => {
        account_ids.push(account_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  let mut category_ids: Vec<u32> = [].to_vec();
  for _ in 0..10 {
    let name = fake::faker::lorem::en::Sentence(1..3).fake::<String>();
    let slug = slugify(&name);
    let description = fake::faker::lorem::en::Paragraph(2..10).fake::<String>();
    let category_id = app_state
      .category_repository
      .create_one_category(DBCategory {
        slug,
        name,
        description,
      })
      .await;
    match category_id {
      Ok(category_id) => {
        category_ids.push(category_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  let mut tag_ids: Vec<u32> = [].to_vec();
  for _ in 0..50 {
    let name = fake::faker::lorem::en::Sentence(3..5).fake::<String>();
    let slug = slugify(&name);
    let tag_id = app_state
      .tag_repository
      .create_one_tag(DBTag { slug, name })
      .await;
    match tag_id {
      Ok(tag_id) => {
        tag_ids.push(tag_id);
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  let job_titles = vec![
    "Software Developer",
    "Data Analyst",
    "Marketing Manager",
    "Human Resources Specialist",
    "Financial Analyst",
    "Project Manager",
    "Sales Representative",
    "Graphic Designer",
    "Customer Service Representative",
    "Registered Nurse",
    "Web Developer",
    "Accountant",
    "Mechanical Engineer",
    "Social Media Manager",
    "Physical Therapist",
    "Executive Assistant",
    "Operations Manager",
    "Business Analyst",
    "Occupational Therapist",
    "Software Engineer",
    "Account Manager",
    "Electrical Engineer",
    "Technical Writer",
    "Business Development Manager",
    "Web Designer",
    "Marketing Coordinator",
    "Financial Manager",
    "Human Resources Manager",
    "Sales Manager",
    "Physical Education Teacher",
    "Mechanical Designer",
    "Quality Assurance Analyst",
    "Technical Support Specialist",
    "Customer Service Manager",
    "Software Quality Assurance Engineer",
    "Marketing Director",
    "Business Intelligence Analyst",
    "Network Administrator",
    "Public Relations Specialist",
    "Supply Chain Manager",
  ];

  let job_short_descriptions = vec![
    "Develops software programs and applications for computers and mobile devices.",
    "Collects, analyzes, and interprets large datasets to identify trends and patterns.",
    "Develops marketing strategies and campaigns to promote products or services.",
    "Recruits, interviews, and hires employees while also managing employee benefits and training programs.",
    "Analyzes financial data to identify trends and make recommendations for investment decisions.",
    "Plans, executes, and manages projects from start to finish.",
    "Sells products or services to customers by identifying their needs and demonstrating how the product or service meets those needs.",
    "Creates visual concepts using computer software or by hand to communicate ideas that inspire, inform, or captivate consumers.",
    "Provides customer service by answering questions and resolving complaints.",
    "Provides patient care in hospitals, clinics, or other healthcare settings.",
    "Designs and develops websites using programming languages such as HTML, CSS, and JavaScript.",
    "Prepares financial records for individuals or businesses by analyzing financial data and making recommendations for improvement.",
    "Designs mechanical systems such as engines, machines, and tools.",
    "Manages social media accounts for businesses or individuals by creating content and engaging with followers.",
    "Helps patients recover from injuries or illnesses by developing treatment plans that include exercise, stretching, and other therapies.",
    "Provides administrative support to high-level executives by managing schedules, arranging meetings, and handling correspondence.",
    "Oversees the day-to-day operations of a business or organization to ensure efficiency and productivity.",
    "Analyzes business processes to identify areas for improvement and makes recommendations for change.",
    "Helps patients recover from injuries or illnesses by developing treatment plans that include activities of daily living.",
    "Designs software programs using programming languages such as Java, Python, or C++.",
    "Manages relationships with clients by providing customer service and ensuring their needs are met.",
    "Designs electrical systems such as power generation equipment, lighting systems, and communication systems.",
    "Creates technical documentation such as user manuals or instruction guides for products or services.",
    "Identifies new business opportunities and develops strategies to increase revenue for a company or organization.",
    "Creates visual designs for websites using computer software such as Adobe Photoshop or Sketch.",
    "Assists marketing managers with developing marketing campaigns by conducting research and analyzing data.",
    "Oversees financial operations of a company or organization by managing budgets, investments, and financial reports.",
    "Oversees human resources operations of a company or organization by managing employee benefits programs and training initiatives.",
    "Manages sales teams by setting sales goals and developing strategies to meet those goals.",
    "Teaches physical education classes in schools.",
    "Designs mechanical systems such as engines, machines, and tools using computer-aided design (CAD) software.",
    "Tests software programs or applications to ensure they meet quality standards before release to the public.",
    "Provides technical support to customers who are experiencing issues with products or services over the phone or online.",
    "Manages customer service teams by setting goals and developing strategies to meet those goals while also ensuring customer satisfaction.",
    "Tests software programs or applications to ensure they meet quality standards before release to the public while also developing automated testing scripts using programming languages such as Python or Java.",
    "Develops marketing strategies for businesses or organizations while also managing marketing teams that execute those strategies.",
    "Analyzes business data to identify trends and make recommendations for improvement while also developing reports using data visualization tools such as Tableau or Power BI.",
    "Manages computer networks for businesses or organizations by installing hardware and software components while also monitoring network performance for issues that need troubleshooting.",
    "Develops public relations campaigns for businesses or organizations while also managing relationships with media outlets such as newspapers or television stations.",
    "Oversees supply chain operations of a company or organization by managing inventory levels, purchasing decisions, and logistics.",
  ];

  let mut post_ids: Vec<u32> = [].to_vec();
  let mut task_ids: Vec<u32> = [].to_vec();
  for index in 0..200 {
    let title = format!("{} #{}", job_titles[index % job_titles.len()], index);
    let short_description = format!(
      "{} #{}",
      job_short_descriptions[index % job_short_descriptions.len()],
      index
    );
    let slug = slugify(&title);
    let post_id = app_state
      .post_repository
      .create_one_post(DBPost {
        slug,
        title,
        category_id: category_ids[index % category_ids.len()],
        poster_id: account_ids[index % account_ids.len()],
        description: fake::faker::lorem::en::Paragraph(20..30).fake::<String>(),
        short_description,
        tag_ids: tag_ids
          .iter()
          .skip(index % tag_ids.len())
          .take(3)
          .map(|tag_id| *tag_id)
          .collect::<Vec<u32>>(),
      })
      .await;
    match post_id {
      Ok(post_id) => {
        let task_id = app_state
          .task_repository
          .create_one_task(DBTask {
            name: TaskName::Indexing {
              model_name: "post".to_string(),
              model_id: post_id,
            },
            status: TaskStatus::Pending,
            r#type: TaskType::Automated,
          })
          .await;
        match task_id {
          Ok(task_id) => {
            post_ids.push(post_id);
            task_ids.push(task_id);
          }
          Err(e) => {
            tracing::error!("error {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
          }
        }
      }
      Err(e) => {
        tracing::error!("error {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
      }
    }
  }

  Json(json!({
    "account_ids": account_ids,
    "category_ids": category_ids,
    "tag_ids": tag_ids,
    "post_ids": post_ids,
    "task_ids": task_ids,
  }))
  .into_response()
}

pub async fn clean_the_database_from_mocks(
  State(app_state): State<AppState>,
  headers: HeaderMap,
) -> impl IntoResponse {
  // @TODO-ZM: move this to a middleware with access to the app_state (cloned)
  if is_admin(&app_state, headers).is_none() {
    return StatusCode::UNAUTHORIZED.into_response();
  }

  let main_db_query = format!(
    r#"
    DELETE account;
    DELETE post;
    DELETE tag;
    DELETE category;
    DELETE task;
    "#,
  );
  let search_db_query = format!(
    r#"
    DELETE word;
    "#,
  );

  let main_db_query_result = app_state.main_db.query(main_db_query.as_str()).await;
  let search_db_query_result = app_state.search_db.query(search_db_query.as_str()).await;

  if main_db_query_result.is_err() || search_db_query_result.is_err() {
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  StatusCode::OK.into_response()
}

pub fn create_test_router() -> Router<AppState> {
  Router::new()
    .route("/seed", axum::routing::post(seed_the_database_with_mocks))
    .route("/clean", axum::routing::post(clean_the_database_from_mocks))
}
