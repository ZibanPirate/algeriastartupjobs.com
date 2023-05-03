use std::net::{SocketAddr, TcpListener};

use axum::Router;

use super::config::SERVER_PORT;

// create and run the loopback server
pub async fn run_loopback_server(app: Router) {
  // create a TcpListener with the loopback address and port
  match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], SERVER_PORT))) {
    Ok(listener) => {
      // get the local address of the listener
      let addr = listener.local_addr().unwrap();
      // create and run the server
      let server = axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());
      tracing::debug!("\nlistening on loopback address http://{}", addr);
      server.await.unwrap();
    }
    Err(e) => {
      // log the error and exit
      tracing::error!("Failed to bind loopback listener: {}", e);
      std::process::exit(1);
    }
  }
}
