use tokio::sync::broadcast;

pub struct AppState {
    pub tx: broadcast::Sender<String>,
}
