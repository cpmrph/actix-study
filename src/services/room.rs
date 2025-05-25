use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::room::{CreateRoom, Event, Room};
use crate::domain::repositories::room::RoomRepository;
use crate::domain::services::room::RoomService;

#[derive(Clone)]
pub struct RoomServiceImpl {
    pub repository: Arc<dyn RoomRepository>,
}

impl RoomServiceImpl {
    pub fn new(repository: Arc<dyn RoomRepository>) -> Self {
        RoomServiceImpl { repository }
    }
}

#[async_trait]
impl RoomService for RoomServiceImpl {
    async fn create(&self, room: CreateRoom) -> Result<Room, CommonError> {
        self.repository
            .create(&room)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, room_id: Uuid) -> Result<Room, CommonError> {
        self.repository
            .get(room_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn join(&self, room_id: Uuid, user_id: Uuid) -> Result<(), CommonError> {
        self.repository
            .join(room_id, user_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn leave(&self, room_id: Uuid, user_id: Uuid) -> Result<(), CommonError> {
        self.repository
            .leave(room_id, user_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn subscribe(&self, room_id: Uuid, version: u32) -> Result<Vec<Event>, CommonError> {
        self.repository
            .subscribe(room_id, version)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
