mod commands;
mod hooks;
mod handler;
mod adapters;

use std::{collections::{HashSet, HashMap}, env, sync::{Arc, atomic::{AtomicUsize}}};
use adapters::db::ShotSaver;
use aws_config::meta::region::RegionProviderChain;
use commands::tags::TAGS_GROUP;
use hooks::{counter, counter::{MessageCount, CommandCounter}};
use handler::Handler;
use serenity::{
    client::{bridge::gateway::ShardManager, Cache},
    framework::{StandardFramework, standard::{Args, HelpOptions, CommandResult, help_commands, CommandGroup, macros::help}},
    prelude::*, model::{channel::Message, id::UserId}, cache::Settings,
};
use tracing::{error};

pub struct AdapterContainer;

impl TypeMapKey for AdapterContainer {
    type Value = ShotSaver;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
     let token = env::var("LOST_BOT_TOKEN").expect("Expected a token in the environment");

    tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .before(counter::before)
        .group(&TAGS_GROUP)
        .help(&MY_HELP);

    let intents = GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_PRESENCES;

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let ddb_client = aws_sdk_dynamodb::Client::new(&config);
    let shot_saver = ShotSaver::new(ddb_client);

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<CommandCounter>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
        data.insert::<AdapterContainer>(shot_saver);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Error setting CTRL-C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}