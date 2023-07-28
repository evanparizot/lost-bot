use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use tracing::info;

#[group]
#[commands(take, give)]
pub struct Tags;

#[command]
async fn take(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let author_id = msg.author.id.clone();
    info!("Author Id: {:?}", author_id);

    Ok(())
}

#[command]
async fn give(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn leaderboard(ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}