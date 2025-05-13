use actix_study::application::session::AppState;
use actix_study::infrastructure::session::{
    create_room_handler, join_room_handler, leave_room_handler,
};
use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/rooms", web::post().to(create_room_handler))
            .route("/rooms/{room_id}/join", web::post().to(join_room_handler))
            .route("/rooms/{room_id}/leave", web::post().to(leave_room_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
