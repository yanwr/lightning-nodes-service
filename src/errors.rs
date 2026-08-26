use axum::{Json, response::{IntoResponse, Response}};
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Mempool Service failed to fetch data")]
    MempoolGatewayError {
        #[source]
        source: reqwest::Error,
    },
    #[error("Mempool Service returned unexpected HTTP status: {status}")]
    MempoolGatewayErrorResponse { 
        status: StatusCode 
    },
}

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("missing environment variable: {0}")]
    MissingEnvironment(String),
    #[error("invalid environment variable {name}: {reason}")]
    InvalidEnvironment { name: String, reason: String },
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: AppErrorData,
}

#[derive(Debug, Serialize)]
pub struct AppErrorData {
    pub code: String,
    pub message: String,
    pub request_id: String,
}
impl AppErrorData {
    pub fn new(code: &str, message: &str, request_id: &str) -> Self {
        Self { code: code.to_string(), message: message.to_string(), request_id: request_id.to_string() }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let request_id = Uuid::now_v7().to_string();
        let (status, code, message) = match &self {
            Self::MempoolGatewayError { source } => (
                source.status().unwrap_or(StatusCode::BAD_GATEWAY),
                "MEMPOOL_GATEWAY_ERROR",
                "The Mempool Gateway is unavailable.",
            ),
            Self::MempoolGatewayErrorResponse { status } => (
                status.to_owned(),
                "MEMPOOL_GATEWAY_ERROR",
                "The Mempool Gateway returned an unexpected response.",
            ),
        };
        tracing::error!(error = ?self, request_id = %request_id, code, "request failed");
        (
            status,
            Json(ErrorResponse {
                error: AppErrorData::new(code, message, &request_id)
            }),
        ).into_response()
    }
}