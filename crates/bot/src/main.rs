use teloxide::prelude::*;

mod ir_verbs;

fn init_logging() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        // On the server: machine readable
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .init();
    } else {
        // On the working machine: human-readable
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_logging();
    tracing::info!("Starting bot...");

    let bot = Bot::from_env(); // Reads TELOXIDE TOKEN

    // Handler
    let handler = Update::filter_message().endpoint(ir_verbs::get_forms);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
