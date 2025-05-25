use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::error::CommonError;
use crate::domain::models::room::{CreateRoom, Room};

#[async_trait]
pub trait RoomService: 'static + Sync + Send {
    async fn create(&self, room: CreateRoom) -> Result<Room, CommonError>;
    async fn get(&self, room_id: Uuid) -> Result<Room, CommonError>;
    async fn join(&self, room_id: Uuid, user_id: Uuid) -> Result<(), CommonError>;
    async fn leave(&self, room_id: Uuid, user_id: Uuid) -> Result<(), CommonError>;
}
