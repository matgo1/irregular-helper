use teloxide::prelude::*;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn get_forms(bot: Bot, msg: Message) -> HandlerResult {
    let verbs = verbs::IrregularVerbs::new();
    let word = msg.text().unwrap_or_default().trim().to_lowercase();

    let reply = match verbs.get(&word) {
        Some([base, past, participle]) => format!("{base} -> {past} -> {participle}"),
        None => "Verb is regular or is not in the list.\n If the word should be in list adress your proposal to @starmatt_1".to_string(),
    };
    bot.send_message(msg.chat.id, reply).await?;

    Ok(())
}
