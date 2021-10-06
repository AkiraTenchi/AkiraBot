mod commands;
use commands::general::*;
use commands::management::*;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{macros::group, StandardFramework},
    model::gateway::Ready,
};
use std::fs;

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(delete_msg, nick)]
struct Management;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("*")) // set prefix to '*'
        .group(&MANAGEMENT_GROUP)
        .group(&GENERAL_GROUP);

    // login via token from file
    let token = fs::read_to_string("token.txt").expect("token from file");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error while creating client");

    // start listening to event
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
