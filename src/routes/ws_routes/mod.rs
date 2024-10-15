use rocket::tokio::sync::{mpsc::Sender, Mutex, MutexGuard};
use std::{
    collections::HashMap,
    sync::{Arc, PoisonError},
};
use ws::Message;

pub type PeersMap = Arc<Mutex<HashMap<String, Sender<Message>>>>;
#[derive(Clone, Debug)]
pub struct WSPeers(PeersMap);

impl WSPeers {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
    pub async fn inner(&self) -> MutexGuard<HashMap<String, Sender<Message>>> {
        self.0.lock().await
    }
}
