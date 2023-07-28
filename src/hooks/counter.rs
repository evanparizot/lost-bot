use serenity::{framework::standard::macros::hook, prelude::TypeMapKey, client::Context, model::channel::Message};
use tracing::info;
use std::{collections::{HashMap}, sync::{Arc, atomic::AtomicUsize}};
use tokio::sync::RwLock;

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = Arc<RwLock<HashMap<String, u64>>>;
}

pub struct MessageCount;

impl TypeMapKey for MessageCount {
    type Value = Arc<AtomicUsize>;
}

#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    let counter_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap").clone()
    };

    {
        let mut counter = counter_lock.write().await;
        let entry = counter.entry(command_name.to_string()).or_insert(0);
        *entry += 1;
    }

    true
}