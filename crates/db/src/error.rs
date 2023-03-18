use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NewClientError(#[from] prisma_client_rust::NewClientError),

    #[error(transparent)]
    QueryError(#[from] prisma_client_rust::QueryError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("Error: {}", self).into_response()
    }
}
