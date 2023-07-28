use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, event::ResumedEvent, prelude::Ready},
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        info!("{}", message.author);
        info!("{}", message.content);
        info!("{}", message.id);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
