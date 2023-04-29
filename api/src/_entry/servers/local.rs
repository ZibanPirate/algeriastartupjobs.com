use std::net::{SocketAddr, TcpListener};

use axum::Router;

use super::config::SERVER_PORT;

// create and run the local server
pub async fn run_local_server(app: Router, ip: std::net::IpAddr) {
    // create a TcpListener with the local address and port
    match TcpListener::bind(SocketAddr::new(ip, SERVER_PORT)) {
        Ok(listener) => {
            // get the local address of the listener
            let addr = listener.local_addr().unwrap();
            // create and run the server
            let server = axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app.into_make_service_with_connect_info::<SocketAddr>());
            tracing::debug!("\nlistening on local address http://{}", addr);
            server.await.unwrap();
        }
        Err(e) => {
            // log the error and exit
            tracing::error!("Failed to bind local listener: {}", e);
            std::process::exit(1);
        }
    }
}
