use chrono::DateTime;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    title: String,
    desc: String,
    time: DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct EventDB {
    title: String,
    desc: String,
    time: bson::DateTime,
}
impl From<Event> for EventDB {
    fn from(p: Event) -> Self {
        Self {
            title: p.title,
            desc: p.desc,
            time: bson::DateTime::from_chrono(p.time),
        }
    }
}
impl From<EventDB> for Event {
    fn from(p: EventDB) -> Self {
        Self {
            title: p.title,
            desc: p.desc,
            time: p.time.to_chrono(),
        }
    }
}
