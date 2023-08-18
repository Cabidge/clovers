use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;

pub struct AppError {
    status: StatusCode,
    message: String,
}

impl From<(StatusCode, String)> for AppError {
    fn from((status, message): (StatusCode, String)) -> Self {
        Self { status, message }
    }
}

impl From<AppError> for (StatusCode, String) {
    fn from(AppError { status, message }: AppError) -> Self {
        (status, message)
    }
}

impl From<DbErr> for AppError {
    fn from(_err: DbErr) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Database Error"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        <(StatusCode, String)>::from(self).into_response()
    }
}
