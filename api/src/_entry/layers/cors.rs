use tower_http::cors::CorsLayer;

pub fn create_cors_layer() -> CorsLayer {
  // @TODO-ZM: what's best for cors? dynamic value or static list?
  let layer = CorsLayer::permissive();

  tracing::info!("created cors layer");
  layer
}
