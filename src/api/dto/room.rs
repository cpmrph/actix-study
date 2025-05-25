use crate::domain::models::room::{CreateRoom, Event, Room, User};
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

#[derive(Deserialize, Serialize)]
pub struct SubscribeRoomDTO {
    pub version: u32,
}

#[derive(Debug, Serialize)]
pub struct RoomDTO {
    id: Uuid,
    owner_id: Uuid,
    version: u32,
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
            version: room.version(),
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

#[derive(Debug, Serialize)]
pub enum EventDTO {
    UserJoined {
        user_id: Uuid,
        timestamp: String, // ISO 8601 format
    },
    UserLeft {
        user_id: Uuid,
        timestamp: String, // ISO 8601 format
    },
}

impl From<Event> for EventDTO {
    fn from(event: Event) -> Self {
        match event {
            Event::UserJoined { user_id, timestamp } => EventDTO::UserJoined {
                user_id,
                timestamp: timestamp.to_rfc3339(),
            },
            crate::domain::models::room::Event::UserLeft { user_id, timestamp } => {
                EventDTO::UserLeft {
                    user_id,
                    timestamp: timestamp.to_rfc3339(),
                }
            }
        }
    }
}
