use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Room {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub members: HashMap<Uuid, User>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl Room {
    pub fn new(owner_id: Uuid) -> Self {
        let mut members = HashMap::new();
        members.insert(
            owner_id,
            User {
                id: owner_id,
                updated_at: Utc::now(),
            },
        );
        Room {
            id: Uuid::new_v4(),
            owner_id,
            members,
            closed_at: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct CreateRoom {
    pub user_id: Uuid,
}
