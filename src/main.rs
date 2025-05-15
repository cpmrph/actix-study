use actix_study::api::{
    CreateSessionRequest, CreateSessionSuccessData, CustomError, JoinSessionErrorData,
    JoinSessionRequest, JoinSessionSuccessData, Request, SessionResponse, SuccessData,
};
use actix_web::{App, Error as ActixError, HttpResponse, HttpServer, web};

// Actix-web handler function
async fn session_handler(
    create: actix_web::Result<web::Json<Request>>,
) -> Result<HttpResponse, ActixError> {
    let query = create.map_err(|_err| CustomError::ActixWebError)?;
    let result = process_session(query.into_inner());
    let response = convert_to_session_response(result)?;

    Ok(HttpResponse::Ok().json(response))
}

fn handle_create_session(create: CreateSessionRequest) -> Result<SuccessData, CustomError> {
    if create.user_name.is_empty() {
        return Err(CustomError::SessionCreationError);
    }
    if create.user_name == "John Doe" {
        return Err(CustomError::SessionCreationError2);
    }
    Ok(SuccessData::Create(CreateSessionSuccessData {
        user_name: create.user_name,
        session_id: "1".into(),
    }))
}

fn handle_join_session(join: JoinSessionRequest) -> Result<SuccessData, CustomError> {
    if join.user_name.is_empty() {
        return Err(CustomError::SessionJoinError(JoinSessionErrorData {
            detail: "name required".into(),
        }));
    }
    // Simulate session joining logic
    Ok(SuccessData::Join(JoinSessionSuccessData {
        user_name: join.user_name,
        session_id: "1".into(),
        joined_at: "2023-10-05T12:34:56Z".into(),
    }))
}

// Simulating a process that returns a Result
fn process_session(request: Request) -> Result<SuccessData, CustomError> {
    match request {
        Request::CreateSession(create) => handle_create_session(create),
        Request::JoinSession(join) => handle_join_session(join),
    }
}

// Convert Result to SessionResponse
fn convert_to_session_response(
    result: Result<SuccessData, CustomError>,
) -> Result<SessionResponse, ActixError> {
    match result {
        Ok(success_data) => Ok(SessionResponse::SUCCESS(success_data)),
        Err(err) => Ok(SessionResponse::FAILURE(err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/session", web::post().to(session_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
