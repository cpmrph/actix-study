use crate::domain::models::room::{CreateRoom, Room, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateRoomDTO {
    pub user_id: Uuid,
}

impl From<CreateRoomDTO> for CreateRoom {
    fn from(dto: CreateRoomDTO) -> Self {
        CreateRoom {
            user_id: dto.user_id,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct JoinRoomDTO {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct LeaveRoomDTO {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct RoomDTO {
    id: Uuid,
    owner_id: Uuid,
    members: Vec<UserDTO>,
}

#[derive(Debug, Serialize)]
pub struct UserDTO {
    id: Uuid,
}
impl From<Room> for RoomDTO {
    fn from(room: Room) -> Self {
        RoomDTO {
            id: room.id,
            owner_id: room.owner_id,
            members: room
                .members
                .values()
                .map(|user| UserDTO::from(user.clone()))
                .collect(),
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO { id: user.id }
    }
}
