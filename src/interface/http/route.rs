use std::sync::Arc;

use axum::{Router, routing::post};

use crate::interface::http::{
    handler::order_handler::{
        create_order,
        // find_order_by_id,
    },
    state::AppState,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/orders", post(create_order))
    // .route("/orders/{id}", get(find_order))
}
