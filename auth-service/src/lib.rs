use axum::{
    routing::post, routing::Router, serve::Serve,
};
use std::error::Error;
use tower_http::services::ServeDir;

use app_state::AppState;

// Struct encapsulates our application related logic
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from 'main.rs' to here.
        // Also, remove the 'hello' route.
        // We don't need it at this point?
        let router = Router::new()
            .nest_service(
                "/",
                ServeDir::new("assets").append_index_html_on_directories(true),
            )
            // TODO: Add authentication routes
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", self.address);
        self.server.await
    }
}
