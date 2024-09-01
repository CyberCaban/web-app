use std::sync::{Mutex, MutexGuard};

use diesel::PgConnection;

use crate::services;

pub struct Connection(Mutex<PgConnection>);

impl Connection {
    pub fn new() -> Self {
        Self(Mutex::new(services::connect_pg()))
    }

    pub fn get(
        &self,
    ) -> Result<MutexGuard<'_, PgConnection>, std::sync::TryLockError<MutexGuard<'_, PgConnection>>>
    {
        self.0.try_lock()
    }
}
