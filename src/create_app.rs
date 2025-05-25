use crate::api::controllers::room_handler::{
    create_room_handler, get_room_handler, join_room_handler, leave_room_handler,
    subscribe_room_handler,
};
use crate::container::Container;
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{App, web};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub fn create_app(
    container: Arc<Container>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let room_service = container.room_service.clone();

    App::new()
        .app_data(web::Data::from(room_service.clone()))
        .wrap(TracingLogger::default())
        .service(
            web::scope("/room")
                .route("", web::post().to(create_room_handler))
                .route("/{id}", web::get().to(get_room_handler))
                .route("/{id}/join", web::post().to(join_room_handler))
                .route("/{id}/leave", web::post().to(leave_room_handler))
                .route("/{id}/subscribe", web::post().to(subscribe_room_handler)),
        )
}
