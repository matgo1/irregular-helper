mod api;

#[tokio::main]
async fn main() {
    api::start_polling().await;
}
