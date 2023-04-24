// Importing the necessary modules
use axum::{extract::Query, http::StatusCode, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Defining the job post struct
#[derive(Serialize, Deserialize, Clone)]
struct JobPost {
    id: u32,
    title: String,
    short_description: String,
    poster: Poster,
}

// Defining the poster struct
#[derive(Serialize, Deserialize, Clone)]
struct Poster {
    name: String,
    avatar_url: String,
}

// Defining the category enum
#[derive(Deserialize)]
enum Category {
    IT,
    Marketing,
    Design,
    Operations,
    Sales,
    Product,
    HR,
    Finance,
    Internships,
    Freelance,
    CoFounders,
    Other,
}

// Defining the query struct
#[derive(Deserialize)]
struct JobQuery {
    category: Category,
}

// Defining the handler function
async fn get_job_posts(
    Query(query): Query<JobQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Mocking some job posts for each category
    let it_posts = vec![
        JobPost {
            id: 1,
            title: "Full-stack developer".to_string(),
            short_description:
                "Looking for a talented full-stack developer with experience in React and Rust."
                    .to_string(),
            poster: Poster {
                name: "Alice".to_string(),
                avatar_url: "https://example.com/alice.jpg".to_string(),
            },
        },
        JobPost {
            id: 2,
            title: "DevOps engineer".to_string(),
            short_description: "Seeking a DevOps engineer with expertise in Kubernetes and AWS."
                .to_string(),
            poster: Poster {
                name: "Bob".to_string(),
                avatar_url: "https://example.com/bob.jpg".to_string(),
            },
        },
        JobPost {
            id: 7,
            title: "Front-end developer".to_string(),
            short_description: "Seeking a Front-end developer with expertise in React and Vue."
                .to_string(),
            poster: Poster {
                name: "Bob".to_string(),
                avatar_url: "https://example.com/bob.jpg".to_string(),
            },
        },
        JobPost {
            id: 8,
            title: "Back-end developer".to_string(),
            short_description: "Seeking a Back-end developer with expertise in Node and Python."
                .to_string(),
            poster: Poster {
                name: "Bob".to_string(),
                avatar_url: "https://example.com/bob.jpg".to_string(),
            },
        },
        JobPost {
            id: 9,
            title: "Mobile developer".to_string(),
            short_description:
                "Seeking a Mobile developer with expertise in Flutter and React Native.".to_string(),
            poster: Poster {
                name: "Bob".to_string(),
                avatar_url: "https://example.com/bob.jpg".to_string(),
            },
        },
        JobPost {
            id: 10,
            title: "Data scientist".to_string(),
            short_description: "Seeking a Data scientist with expertise in Python and R."
                .to_string(),
            poster: Poster {
                name: "Bob".to_string(),
                avatar_url: "https://example.com/bob.jpg".to_string(),
            },
        },
        JobPost {
            id: 11,
            title: "Data engineer".to_string(),
            short_description: "Seeking a Data engineer with expertise in Python and R."
                .to_string(),
            poster: Poster {
                name: "Bob".to_string(),
                avatar_url: "https://example.com/bob.jpg".to_string(),
            },
        },
    ];

    let marketing_posts = vec![
        JobPost {
            id: 3,
            title: "Social media manager".to_string(),
            short_description: "Looking for a creative social media manager who can grow our online presence and engagement.".to_string(),
            poster: Poster {
                name: "Charlie".to_string(),
                avatar_url: "https://example.com/charlie.jpg".to_string(),
            },
        },
        JobPost {
            id: 4,
            title: "Content writer".to_string(),
            short_description: "Seeking a content writer who can produce high-quality blog posts and newsletters.".to_string(),
            poster: Poster {
                name: "Diana".to_string(),
                avatar_url: "https://example.com/diana.jpg".to_string(),
            },
        },
    ];

    let design_posts = vec![
        JobPost {
            id: 5,
            title: "UI/UX designer".to_string(),
            short_description:
                "Looking for a UI/UX designer who can create beautiful and intuitive user interfaces.".to_string(),
            poster: Poster {
                name: "Evan".to_string(),
                avatar_url: "https://example.com/evan.jpg".to_string(),
            },
        },
        JobPost {
            id: 6,
            title: "Graphic designer".to_string(),
            short_description:
                "Seeking a graphic designer who can create stunning logos and branding materials.".to_string(),
            poster: Poster {
                name: "Fiona".to_string(),
                avatar_url: "https://example.com/fiona.jpg".to_string(),
            },
        },
    ];

    // Creating an empty vector to store the matching job posts
    let mut job_posts = Vec::new();

    // Looping through the query categories and adding the corresponding job posts to the vector
    // for category in query.category.iter() {
    match query.category {
        Category::IT => job_posts.extend(it_posts.clone()),
        Category::Marketing => job_posts.extend(marketing_posts.clone()),
        Category::Design => job_posts.extend(design_posts.clone()),
        // TODO: Add more categories and job posts as needed
        _ => return Err(StatusCode::NOT_IMPLEMENTED),
    }
    // }

    // Returning the vector of job posts as JSON
    Ok(Json(
        json!({ "category":{"name":"IT"},  "job_posts": job_posts }),
    ))
}

pub fn job_post_controller() -> Router {
    Router::new().route("/", get(get_job_posts))
}
