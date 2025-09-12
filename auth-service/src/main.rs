use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working

    let user_store = Arc::new(RwLock::new(HashmapUserStore::default())) as Arc<RwLock<dyn auth_service::domain::UserStore + Send + Sync>>;
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

// async fn hello_handler() -> Html<&'static str> {
//// DONE: Update this to a custom message!
//   Html("<h1>🦀 Hello, Rustaceans! 🦀</h1>")
// }
