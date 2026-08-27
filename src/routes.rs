use std::sync::Arc;

use axum::{Router, http::HeaderName, routing::get};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::{nodes::apis::list, state::AppState};

pub fn build_routes(app_state: Arc<AppState>) -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");
    Router::new()
        .route("/nodes", get(list::list))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::new(
            request_id_header.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(request_id_header))
}
