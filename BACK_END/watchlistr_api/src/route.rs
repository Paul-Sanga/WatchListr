use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_show_handler, delete_show_handler, edit_show_handler, get_show_handler,
        health_checker_handler, show_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/shows/", post(create_show_handler))
        .route("/api/shows", get(show_list_handler))
        .route(
            "/api/shows/:id",
            get(get_show_handler)
                .patch(edit_show_handler)
                .delete(delete_show_handler),
        )
        .with_state(app_state)
}
