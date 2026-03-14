use chrono::{DateTime, Utc};

use crate::domain::error::Result;

pub trait LocalNotificationService: Send + Sync {
    fn request_permission(&self) -> Result<()>;
    fn show_now(&self, id: &str, title: &str, body: &str) -> Result<()>;
    fn schedule_at(&self, id: &str, title: &str, body: &str, at: DateTime<Utc>) -> Result<()>;
}

#[derive(Clone, Default)]
pub struct StubLocalNotificationService;

impl LocalNotificationService for StubLocalNotificationService {
    fn request_permission(&self) -> Result<()> {
        Ok(())
    }

    fn show_now(&self, _id: &str, _title: &str, _body: &str) -> Result<()> {
        Ok(())
    }

    fn schedule_at(&self, _id: &str, _title: &str, _body: &str, _at: DateTime<Utc>) -> Result<()> {
        Ok(())
    }
}
