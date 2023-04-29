use axum::{routing::get, Json, Router};
use local_ip_address::local_ip;
use serde_json::json;

use crate::post::controllers::create_post_router;

use super::{
    layers::{cors::create_cors_layer, trace::create_trace_layer},
    servers::{local::run_local_server, loopback::run_loopback_server},
    state::create_app_state,
};

pub async fn actual_main() {
    // create the app router
    let app = create_app();

    // get the local IP address of the system
    match local_ip() {
        Ok(ip) => {
            // run both loopback and local servers
            let loopback_server = run_loopback_server(app.clone());
            let local_server = run_local_server(app, ip);
            // await both servers concurrently
            let (_, _) = tokio::join!(loopback_server, local_server);
        }
        Err(e) => {
            // log the error as info and run only the loopback server
            tracing::info!("Running only on the loopback address: {}", e);
            let loopback_server = run_loopback_server(app);
            // await the loopback server
            loopback_server.await;
        }
    }
}

// create and configure the app router
fn create_app() -> Router {
    let app_state = create_app_state();

    let app = Router::new();
    let app = app.layer(create_trace_layer()).layer(create_cors_layer());
    let app = app
        .nest("/posts", create_post_router())
        .route(
            "/",
            get(|| async {
                Json(json!({
                    "app": { "version": env!("CARGO_PKG_VERSION") },
                    "repository": { "url": env!("CARGO_PKG_REPOSITORY") }
                }))
            }),
        )
        .with_state(app_state);
    app
}
