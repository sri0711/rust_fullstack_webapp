use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing)]
    pub status: Option<i32>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = match self.status {
            Some(code) => StatusCode::from_u16(code as u16).unwrap_or(StatusCode::BAD_REQUEST),
            None => {
                if self.success {
                    StatusCode::OK
                } else {
                    StatusCode::BAD_REQUEST
                }
            }
        };

        // Create the JSON response and set the status
        (status, Json(self)).into_response()
    }
}
