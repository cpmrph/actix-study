use std::sync::Arc;

use crate::{
    domain::{repositories::room::RoomRepository, services::room::RoomService},
    infrastructure::repositories::room::RoomInMemoryRepository,
    services::room::RoomServiceImpl,
};

pub struct Container {
    pub room_service: Arc<dyn RoomService>,
}

impl Container {
    pub fn new() -> Self {
        let room_repository: Arc<dyn RoomRepository> = Arc::new(RoomInMemoryRepository::new());
        let room_service = Arc::new(RoomServiceImpl {
            repository: room_repository,
        });
        Container { room_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
