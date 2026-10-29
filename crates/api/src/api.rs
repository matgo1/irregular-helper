use axum::{Router, routing::get};

fn create_router() -> Router {
    Router::new()
        .route("/verbs/{base}", get(handlers::get_verbs))
        .with_state(verbs::IrregularVerbs::new())
}

pub async fn start_polling() {
    let app: Router = create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
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
