mod api;
mod logic;

#[tokio::main]
async fn main() {
    api::start_polling().await;
}
