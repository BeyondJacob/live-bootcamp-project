mod helpers;
mod login;
mod logout;
mod root;
mod signup;
mod verify_2fa;
mod verify_token;

use auth_service::{Application, app_state::AppState, services::hashmap_user_store::HashmapUserStore};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // Create user store and app state
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default())) as Arc<RwLock<dyn auth_service::domain::UserStore + Send + Sync>>;
    let app_state = AppState::new(user_store);

    // Start the application
    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build application");

    // Run the application
    app.run().await.expect("Failed to run application");
}
