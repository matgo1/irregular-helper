use axum::{Router, routing::get};

fn create_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/verbs/{base}", get(handlers::get_verbs))
        .with_state(verbs::IrregularVerbs::new())
}

async fn shutdown_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut term = signal(SignalKind::terminate()).expect("can't listen from SIGTERM");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = term.recv() => {},
    }
    tracing::info!("Shutting down");
}

pub async fn serve() -> std::io::Result<()> {
    let addr = std::env::var("API_ADDR").unwrap_or_else(|_| "127.0.0.1:5080".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("API listening on {addr}");
    axum::serve(listener, create_router())
        .with_graceful_shutdown(shutdown_signal())
        .await
}

mod handlers {
    use axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
    };

    use verbs::{Forms, IrregularVerbs};

    pub async fn get_verbs(
        State(st): State<IrregularVerbs>,
        Path(base): Path<String>,
    ) -> Result<Json<Forms>, StatusCode> {
        st.get(&base)
            .cloned()
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    }
}
