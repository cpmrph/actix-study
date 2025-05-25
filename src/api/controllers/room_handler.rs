use crate::api::dto::room::{CreateRoomDTO, JoinRoomDTO, LeaveRoomDTO, RoomDTO};
use crate::domain::error::ApiError;
use crate::domain::services::room::RoomService;
use actix_web::{HttpResponse, Result, web};
use uuid::Uuid;

pub async fn create_room_handler(
    room_service: web::Data<dyn RoomService>,
    post_data: web::Json<CreateRoomDTO>,
) -> Result<web::Json<RoomDTO>, ApiError> {
    let room = room_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(room.into()))
}

pub async fn get_room_handler(
    room_service: web::Data<dyn RoomService>,
    params: web::Path<Uuid>,
) -> Result<web::Json<RoomDTO>, ApiError> {
    let room = room_service.get(params.into_inner()).await?;
    Ok(web::Json(room.into()))
}

pub async fn join_room_handler(
    room_service: web::Data<dyn RoomService>,
    params: web::Path<Uuid>,
    post_data: web::Json<JoinRoomDTO>,
) -> Result<HttpResponse, ApiError> {
    room_service
        .join(params.into_inner(), post_data.user_id)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn leave_room_handler(
    room_service: web::Data<dyn RoomService>,
    params: web::Path<Uuid>,
    post_data: web::Json<LeaveRoomDTO>,
) -> Result<HttpResponse, ApiError> {
    room_service
        .leave(params.into_inner(), post_data.user_id)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
