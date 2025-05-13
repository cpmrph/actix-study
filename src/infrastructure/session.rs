use crate::application::session::AppState;
use crate::domain::session::User;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RoomResponse {
    room_id: Uuid,
}

#[derive(Deserialize)]
pub struct ModifyUserRequest {
    user_id: Uuid,
}

pub async fn create_room_handler(data: web::Data<AppState>) -> impl Responder {
    let room_id = data.create_room();
    HttpResponse::Ok().json(RoomResponse { room_id })
}

pub async fn join_room_handler(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<ModifyUserRequest>,
) -> impl Responder {
    let room_id = path.into_inner();
    let user_id = req.into_inner().user_id;

    if data.add_user_to_room(room_id, User { id: user_id }) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn leave_room_handler(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    req: web::Json<ModifyUserRequest>,
) -> impl Responder {
    let room_id = path.into_inner();
    let user_id = req.into_inner().user_id;

    if data.remove_user_from_room(room_id, user_id) {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
