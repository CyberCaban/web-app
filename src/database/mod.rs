use std::sync::{Mutex, MutexGuard};

use diesel::PgConnection;

use crate::services;


pub struct Connection(Mutex<PgConnection>);

impl Connection {
    pub fn new() -> Self {
        Self(Mutex::new(services::connect_pg()))
    }

    pub fn get(&self) -> MutexGuard<PgConnection> {
        self.0.lock().expect("Connection error")
    }
}