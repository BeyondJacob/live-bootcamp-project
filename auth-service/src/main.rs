// use axum::{response::Html, routing::get, routing::post, Router, http::StatusCode, response::IntoResponse};
// use tower_http::services::ServeDir;
use auth_service::Application;

#[tokio::main]
async fn main() {
    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // println!("listening on {}", listener.local_addr().unwrap());

    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    // axum::serve(listener, app).await.unwrap();
    app.run().await.expect("Failed to run app");
}

// async fn hello_handler() -> Html<&'static str> {
//// DONE: Update this to a custom message!
//   Html("<h1>🦀 Hello, Rustaceans! 🦀</h1>")
// }
