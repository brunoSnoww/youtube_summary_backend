use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use axum::{routing::post, Router};
use std::net::SocketAddr;
use yt_summarizer_backend::process_youtube_video;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SummarizeRequest {
    youtube_url: String,
}

#[derive(Serialize)]
struct SummarizeResponse {
    summary: String,
}

async fn summarize_handler(
    Json(payload): Json<SummarizeRequest>,
) -> Result<Json<SummarizeResponse>, (StatusCode, String)> {
    // Extract the YouTube URL from the request
    let youtube_url = payload.youtube_url;
    // Call your existing functions
    match process_youtube_video::process_youtube_video(&youtube_url).await {
        Ok(summary) => Ok(Json(SummarizeResponse { summary })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/summarize", post(summarize_handler));

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
