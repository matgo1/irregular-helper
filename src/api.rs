use axum::{Router, routing::get};

fn create_router() -> Router {
    Router::new().route("/", get(|| async { "Hello world!" }))
}

pub async fn start_polling() {
    let app: Router = create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
