use std::{collections::HashMap, vec};

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Room {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub members: HashMap<Uuid, User>,
    pub events: Vec<Event>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl Room {
    pub fn new(owner_id: Uuid) -> Self {
        let now = Utc::now();
        let mut members = HashMap::new();
        members.insert(
            owner_id,
            User {
                id: owner_id,
                updated_at: now,
            },
        );
        Room {
            id: Uuid::new_v4(),
            owner_id,
            members,
            events: vec![Event::UserJoined {
                user_id: owner_id,
                timestamp: now,
            }],
            closed_at: None,
        }
    }

    pub fn version(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn get_events_since(&self, version: u32) -> Vec<Event> {
        self.events.iter().skip(version as usize).cloned().collect()
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub enum Event {
    UserJoined {
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserLeft {
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Clone)]
pub struct CreateRoom {
    pub user_id: Uuid,
}
