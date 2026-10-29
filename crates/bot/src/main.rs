use std::sync::Arc;

use teloxide::prelude::*;

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
    let verbs = Arc::new(verbs::IrregularVerbs::new());

    // HACK: Move into separated function later
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let verbs = Arc::clone(&verbs);
        async move {
            let word = msg.text().unwrap_or_default().trim().to_lowercase();
            let reply = match verbs.get(&word) {
                Some([base, past, participle]) => format!("{base} -> {past} -> {participle}"),
                None => "Regular verb or I don't have it in my list".to_string(),
            };
            bot.send_message(msg.chat.id, reply).await?;
            Ok(())
        }
    })
    .await
}
