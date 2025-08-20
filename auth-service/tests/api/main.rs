mod helpers;
mod routes;

use auth_service::Application;

#[tokio::main]
async fn main() {
    // Start the application
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build application");

    // Run the application
    app.run().await.expect("Failed to run application");
}
