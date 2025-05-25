use crate::domain::models::room::{CreateRoom, Room};
use async_trait::async_trait;
use uuid::Uuid;

use super::repository::RepositoryResult;

#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn create(&self, new_room: &CreateRoom) -> RepositoryResult<Room>;
    async fn get(&self, room_id: Uuid) -> RepositoryResult<Room>;
    async fn join(&self, room_id: Uuid, user_id: Uuid) -> RepositoryResult<()>;
    async fn leave(&self, room_id: Uuid, user_id: Uuid) -> RepositoryResult<()>;
}
