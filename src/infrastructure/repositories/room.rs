use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;
use uuid::Uuid;

use crate::domain::models::room::{CreateRoom, Room, User};
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::room::RoomRepository;
use crate::infrastructure::error::InMemoryRepositoryError;

pub struct RoomInMemoryRepository {
    pub rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
}

impl RoomInMemoryRepository {
    pub fn new() -> Self {
        RoomInMemoryRepository {
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for RoomInMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RoomRepository for RoomInMemoryRepository {
    async fn create(&self, new_room: &CreateRoom) -> RepositoryResult<Room> {
        let room = Room::new(new_room.user_id);

        self.rooms.lock().unwrap().insert(room.id, room.clone());

        info!("Room created: {:?}", room);

        Ok(room)
    }

    async fn get(&self, room_id: Uuid) -> RepositoryResult<Room> {
        let binding = self.rooms.lock().unwrap();
        if let Some(room) = binding.get(&room_id) {
            Ok(room.clone())
        } else {
            Err(
                InMemoryRepositoryError::from(format!("Room with id {} not found", room_id))
                    .into_inner(),
            )
        }
    }

    async fn join(&self, room_id: Uuid, user_id: Uuid) -> RepositoryResult<()> {
        let mut binding = self.rooms.lock().unwrap();
        if let Some(room) = binding.get_mut(&room_id) {
            room.members.insert(
                user_id,
                User {
                    id: user_id,
                    updated_at: Utc::now(),
                },
            );
            info!("User {} joined room {}", user_id, room_id);
        } else {
            return Err(InMemoryRepositoryError::from(format!(
                "Room with id {} not found",
                room_id
            ))
            .into_inner());
        }
        Ok(())
    }

    async fn leave(&self, room_id: Uuid, user_id: Uuid) -> RepositoryResult<()> {
        let mut binding = self.rooms.lock().unwrap();
        if let Some(room) = binding.get_mut(&room_id) {
            room.members.remove(&user_id);
            info!("User {} left room {}", user_id, room_id);
        } else {
            return Err(InMemoryRepositoryError::from(format!(
                "Room with id {} not found",
                room_id
            ))
            .into_inner());
        }
        Ok(())
    }
}
