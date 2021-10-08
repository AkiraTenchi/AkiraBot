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
#[commands(delete_msg, nick, kick)]
struct Management;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, mut new_member: Member) {
    //     println!("new member");
    //     let roles = guild_id.roles(&ctx.http).await.expect("failed to get guild roles");
    //     let lowest_role = roles.into_iter().last().expect("lowest role");
    //     new_member.add_role(&ctx.http, lowest_role.0).await.expect("member role addition");
    // }

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
