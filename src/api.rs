use actix_web::{Error as ActixError, HttpResponse, ResponseError};
use core::fmt;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use thiserror::Error;

// #[derive(Debug, Error, Serialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "error_code")]
#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Session creation failed")]
    SessionCreationError,

    #[error("Session creation failed2")]
    SessionCreationError2,

    #[error("Session join failed: {0}")]
    SessionJoinError(JoinSessionErrorData),

    #[error("Actix-Web error: {0}")]
    ActixWebError(String),

    #[error("Logic error")]
    LogicError,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "action", content = "data")]
pub enum Request {
    CreateSession(CreateSessionRequest),
    JoinSession(JoinSessionRequest),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreateSessionRequest {
    pub user_name: String,
    pub flag: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JoinSessionRequest {
    pub user_name: String,
}

impl From<ActixError> for CustomError {
    fn from(err: ActixError) -> Self {
        CustomError::ActixWebError(format!("{:?}", err))
    }
}

// Implement ResponseError trait for CustomError
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct JoinSessionErrorData {
    pub detail: String,
}

impl fmt::Display for JoinSessionErrorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Detail: {}", self.detail)
    }
}

#[derive(Serialize)]
#[serde(tag = "result", content = "data")]
pub enum SessionResponse {
    SUCCESS(SuccessData),
    FAILURE(CustomError),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SuccessData {
    Create(CreateSessionSuccessData),
    Join(JoinSessionSuccessData),
}

#[derive(Serialize)]
pub struct CreateSessionSuccessData {
    pub user_name: String,
    pub session_id: String,
}

#[derive(Serialize)]
pub struct JoinSessionSuccessData {
    pub user_name: String,
    pub session_id: String,
    pub joined_at: String,
}

impl Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomError", 2)?;
        match self {
            CustomError::SessionCreationError => {
                state.serialize_field("error_code", "SESSION_CREATION_ERROR")?;
            }
            CustomError::SessionCreationError2 => {
                state.serialize_field("error_code", "SESSION_CREATION_ERROR2")?;
            }
            CustomError::SessionJoinError(data) => {
                state.serialize_field("error_code", "SESSION_JOIN_ERROR")?;
                state.serialize_field("detail", &data.detail)?;
            }
            CustomError::ActixWebError(detail) => {
                state.serialize_field("error_code", "ACTIX_WEB_ERROR")?;
                state.serialize_field("detail", detail)?;
            }
            CustomError::LogicError => {
                state.serialize_field("error_code", "LOGIC_ERROR")?;
            }
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_join_session_error_data_serialization() {
        let data = JoinSessionErrorData {
            detail: "Internal server error".to_string(),
        };
        let json = serde_json::to_string(&data).unwrap();
        let expected_json = r#"{"detail":"Internal server error"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_custom_error_serialization() {
        let error = CustomError::SessionCreationError;
        let json = serde_json::to_string(&error).unwrap();
        let expected_json = r#"{"error_code":"SESSION_CREATION_ERROR"}"#;
        assert_eq!(json, expected_json);

        let error2 = CustomError::SessionCreationError2;
        let json2 = serde_json::to_string(&error2).unwrap();
        let expected_json2 = r#"{"error_code":"SESSION_CREATION_ERROR2"}"#;
        assert_eq!(json2, expected_json2);

        let logic_error = CustomError::LogicError;
        let logic_json = serde_json::to_string(&logic_error).unwrap();
        let expected_logic_json = r#"{"error_code":"LOGIC_ERROR"}"#;
        assert_eq!(logic_json, expected_logic_json);

        let actix_error = CustomError::ActixWebError("Some Actix error".to_string());
        let actix_json = serde_json::to_string(&actix_error).unwrap();
        let expected_actix_json = r#"{"error_code":"ACTIX_WEB_ERROR","detail":"Some Actix error"}"#;
        assert_eq!(actix_json, expected_actix_json);
    }

    #[test]
    fn test_session_response_success_serialization() {
        let success_data = SuccessData::Create(CreateSessionSuccessData {
            user_name: "Alice".to_string(),
            session_id: "xyz123".to_string(),
        });
        let response = SessionResponse::SUCCESS(success_data);
        let json = serde_json::to_string(&response).unwrap();
        let expected_json =
            r#"{"result":"SUCCESS","data":{"user_name":"Alice","session_id":"xyz123"}}"#;
        assert_eq!(json, expected_json);

        let success_data_join = SuccessData::Join(JoinSessionSuccessData {
            user_name: "Bob".to_string(),
            session_id: "abc456".to_string(),
            joined_at: "2023-01-01T12:00:00Z".to_string(),
        });
        let response_join = SessionResponse::SUCCESS(success_data_join);
        let json_join = serde_json::to_string(&response_join).unwrap();
        let expected_json_join = r#"{"result":"SUCCESS","data":{"user_name":"Bob","session_id":"abc456","joined_at":"2023-01-01T12:00:00Z"}}"#;
        assert_eq!(json_join, expected_json_join);
    }

    #[test]
    fn test_session_response_failure_serialization() {
        let error = CustomError::SessionCreationError;
        let response = SessionResponse::FAILURE(error);
        let json = serde_json::to_string(&response).unwrap();
        let expected_json =
            r#"{"result":"FAILURE","data":{"error_code":"SESSION_CREATION_ERROR"}}"#;
        assert_eq!(json, expected_json);

        let join_error_data = JoinSessionErrorData {
            detail: "User not found".to_string(),
        };
        let join_error = CustomError::SessionJoinError(join_error_data);
        let join_response = SessionResponse::FAILURE(join_error);
        let join_json = serde_json::to_string(&join_response).unwrap();
        let expected_join_json = r#"{"result":"FAILURE","data":{"error_code":"SESSION_JOIN_ERROR","detail":"User not found"}}"#;
        assert_eq!(join_json, expected_join_json);

        let actix_error = CustomError::ActixWebError("Network failure".to_string());
        let actix_response = SessionResponse::FAILURE(actix_error);
        let actix_json = serde_json::to_string(&actix_response).unwrap();
        let expected_actix_json = r#"{"result":"FAILURE","data":{"error_code":"ACTIX_WEB_ERROR","detail":"Network failure"}}"#;
        assert_eq!(actix_json, expected_actix_json);
    }
}
