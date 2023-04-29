use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer() -> CorsLayer {
    let layer = CorsLayer::new()
        // @TODO-ZM: what's best for cors? dynamic value or static list?
        .allow_origin(Any)
        .allow_methods([Method::GET]);
    tracing::info!("created cors layer");
    layer
}
