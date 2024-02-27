use chrono::{DateTime, Utc};

pub struct Save {
    timestamp: DateTime<Utc>,
    digest: String,
    content: Vec<u8>,
}
