use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub deadline: String,
    pub progress: f32,
    pub completed: bool,
}

fn get_milestones() -> Vec<Milestone> {
    vec![
        Milestone {
            id: "1".to_string(),
            title: "First milestone".to_string(),
            description: "This is the first milestone".to_string(),
            deadline: "2023-05-01".to_string(),
            progress: 1.0,
            completed: true,
        },
        Milestone {
            id: "2".to_string(),
            title: "Second milestone".to_string(),
            description: "This is the second milestone".to_string(),
            deadline: "2023-06-01".to_string(),
            progress: 0.8,
            completed: false,
        },
        Milestone {
            id: "3".to_string(),
            title: "Third milestone".to_string(),
            description: "This is the third milestone".to_string(),
            deadline: "2023-07-01".to_string(),
            progress: 0.2,
            completed: false,
        },
    ]
}

pub fn milestone_controller() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            let milestones = get_milestones();
            axum::response::Json(json!({ "milestones": milestones }))
        }),
    )
}
