use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, Application, utils::constants::prod,
};

#[tokio::main]
async fn main() {

    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
    .await
    .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
    
}
