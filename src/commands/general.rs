use serenity::utils::MessageBuilder;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let content = MessageBuilder::new()
        .push_bold_line("Help")
        .push_underline_line("commands")
        .push_line("ping: replies with pong if connection can be established")
        .push_line("delete_msg {amount of msgs}: deletes the specified amount of messages if no amount is specified it will delete one msg")
        .push_line("ban @{username}: bans mentioned user from the server")
        .push_line("kick @{username}: kicks mentioned user from the server")
        .push_line("nick @{username} {nick name}: changes nickname of mentioned user to specified string")
        .build();

    msg.channel_id.say(&ctx.http, content).await?;

    Ok(())
}
