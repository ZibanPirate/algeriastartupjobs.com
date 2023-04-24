use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Method, Request},
    response::Response,
    routing::get,
    Router,
};
use local_ip_address::local_ip;
use std::{
    net::{SocketAddr, TcpListener},
    time::Duration,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod job_post;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "debug=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/job-posts", job_post::controller::job_post_controller())
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                // @TODO-ZM: what's best for cors? dynamic value or static list?
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        )
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    // get the local IP address of the system
    let ip = local_ip();

    // check if the IP address is local
    if let Ok(ip) = ip {
        // create two TcpListeners with different addresses and ports
        let loopback_listener =
            TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 9090))).unwrap();
        let loopback_addr = loopback_listener.local_addr().unwrap();
        let loopback_server = axum::Server::from_tcp(loopback_listener).unwrap().serve(
            app.clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        );

        let local_listener = TcpListener::bind(SocketAddr::new(ip, 9090)).unwrap();
        let local_addr = local_listener.local_addr().unwrap();
        let local_server = axum::Server::from_tcp(local_listener)
            .unwrap()
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());

        tracing::debug!(
            "\nlistening on both:\nhttp://{}\nhttp://{}",
            loopback_addr,
            local_addr
        );

        // run both servers concurrently
        let (_, _) = tokio::join!(local_server, loopback_server);
    } else {
        // create a TcpListener with the loopback address and port
        let loopback_listener =
            TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 9090))).unwrap();
        let loopback_addr = loopback_listener.local_addr().unwrap();
        let loopback_server = axum::Server::from_tcp(loopback_listener)
            .unwrap()
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());

        tracing::debug!("\nlistening on http://{}", loopback_addr);

        // run the server
        loopback_server.await.unwrap();
    }
}
