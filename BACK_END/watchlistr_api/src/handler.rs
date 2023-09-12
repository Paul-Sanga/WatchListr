use std::sync::Arc;

// IMPLEMENT CRUD FUNCTIONALITY

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::json;

use crate::{
    model::{ShowModel, ShowModelResponse},
    schema::{CreateShowSchema, FilterOptions, UpdateShowSchema},
    AppState,
};

fn filter_db_record(show: &ShowModel) -> ShowModelResponse {
    ShowModelResponse {
        id: show.id.to_owned(),
        title: show.title.to_owned(),
        content: show.content.to_owned(),
        category: show.category.to_owned().unwrap(),
        published: show.published != 0,
        createdAt: show.created_at.unwrap(),
        updatedAt: show.updated_at.unwrap(),
    }
}

// GET ALL SHOWS | Route Function, fn
pub async fn show_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    // SET PAGINATION VALUES
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // DEFINE SQL QUERY TO GET ALL SHOWS
    let shows = sqlx::query_as!(
        ShowModel,
        r#"SELECT * FROM shows ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        // DEFINE ERROR RESPONSE
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let show_responses = shows
        .iter()
        .map(|show| filter_db_record(&show))
        .collect::<Vec<ShowModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": show_responses.len(),
        "shows": show_responses
    });

    Ok(Json(json_response))
}

// ADD SHOW | Route Function, fn
pub async fn create_show_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateShowSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = uuid::Uuid::new_v4().to_string();
    let query_result =
        sqlx::query(r#"INSERT INTO shows (id,title,content,category) VALUES (?, ?, ?, ?)"#)
            .bind(user_id.clone())
            .bind(body.title.to_string())
            .bind(body.content.to_string())
            .bind(body.category.to_owned().unwrap_or_default())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Show with that title already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", err)})),
        ));
    }

    let show = sqlx::query_as!(ShowModel, r#"SELECT * FROM shows WHERE id = ?"#, user_id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    let show_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "show": filter_db_record(&show)
    })});

    Ok(Json(show_response))
}
