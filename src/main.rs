use teloxide::{requests::Requester, types::Message, Bot};

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    println!("Starting bot...");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, "🎯").await?;
        Ok(())
    })
    .await;
}
