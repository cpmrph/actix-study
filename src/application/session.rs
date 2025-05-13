use crate::domain::session::{Room, User};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct AppState {
    pub rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_room(&self) -> Uuid {
        let room = Room::new();
        let room_id = room.id;
        self.rooms.lock().unwrap().insert(room_id, room);
        room_id
    }

    pub fn add_user_to_room(&self, room_id: Uuid, user: User) -> bool {
        if let Some(room) = self.rooms.lock().unwrap().get_mut(&room_id) {
            room.add_user(user.id);
            true
        } else {
            false
        }
    }

    pub fn remove_user_from_room(&self, room_id: Uuid, user_id: Uuid) -> bool {
        if let Some(room) = self.rooms.lock().unwrap().get_mut(&room_id) {
            room.remove_user(&user_id);
            true
        } else {
            false
        }
    }
}
