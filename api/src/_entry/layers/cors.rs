use axum::Router;
use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn attach_cors_layer(router: Router) -> Router {
    let router = router.layer(
        CorsLayer::new()
            // @TODO-ZM: what's best for cors? dynamic value or static list?
            .allow_origin(Any)
            .allow_methods([Method::GET]),
    );
    tracing::info!("attached cors layer");
    router
}
