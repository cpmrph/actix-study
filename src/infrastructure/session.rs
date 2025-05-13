use crate::application::session::AppState;
use crate::domain::session::User;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RoomResponse {
    room_id: Uuid,
}

#[derive(Deserialize, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::session::AppState;
    use actix_web::{App, body::to_bytes, dev::Service, http::StatusCode, test, web};

    #[actix_rt::test]
    async fn test_create_room_handler() {
        let app_state = web::Data::new(AppState::new());
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/rooms", web::post().to(create_room_handler)),
        )
        .await;

        let req = test::TestRequest::post().uri("/rooms").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        let room_response: RoomResponse = serde_json::from_slice(&body).unwrap();

        assert!(
            app_state
                .rooms
                .lock()
                .unwrap()
                .contains_key(&room_response.room_id)
        );
    }

    #[actix_rt::test]
    async fn test_join_room_handler() {
        let app_state = web::Data::new(AppState::new());
        let room_id = app_state.create_room();
        let user = User::new();

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/rooms/{room_id}/join", web::post().to(join_room_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&format!("/rooms/{}/join", room_id))
            .set_json(&ModifyUserRequest { user_id: user.id })
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert!(
            app_state
                .rooms
                .lock()
                .unwrap()
                .get(&room_id)
                .unwrap()
                .users
                .contains(&user.id)
        );
    }

    #[actix_rt::test]
    async fn test_leave_room_handler() {
        let app_state = web::Data::new(AppState::new());
        let room_id = app_state.create_room();
        let user = User::new();
        app_state.add_user_to_room(room_id, user.clone());

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/rooms/{room_id}/leave", web::post().to(leave_room_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(&format!("/rooms/{}/leave", room_id))
            .set_json(&ModifyUserRequest { user_id: user.id })
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert!(
            !app_state
                .rooms
                .lock()
                .unwrap()
                .get(&room_id)
                .unwrap()
                .users
                .contains(&user.id)
        );
    }
}
