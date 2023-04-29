use axum::{routing::get, Json, Router};
use local_ip_address::local_ip;
use serde_json::json;
use servers::{local::run_local_server, loopback::run_loopback_server};

mod layers;
mod servers;

#[tokio::main]
async fn main() {
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
    let app = Router::new();
    let app = layers::trace::attach_trace_layer(app);
    let app = layers::cors::attach_cors_layer(app);
    let app = app.route(
        "/",
        get(|| async {
            Json(json!({
                "app": { "version": env!("CARGO_PKG_VERSION") },
                "repository": { "url": env!("CARGO_PKG_REPOSITORY") }
            }))
        }),
    );
    app
}
