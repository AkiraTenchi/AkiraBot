use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::futures::TryFutureExt;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::static_assertions::_core::convert::TryFrom;

#[command]
async fn delete_msg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_id = msg.channel_id;
    let mut num;
    if args.is_empty() {
        num = 2
    } else {
        num = args.parse::<u64>().unwrap() + 1;
    }

    let res = match ctx
        .http
        .get_messages(u64::from(channel_id), &format!("?limit={:?}", num))
        .await
    {
        Ok(msgs) => msgs,
        Err(e) => {
            println!("failed do to {:?}", e);
            return Err(CommandError::try_from(e).unwrap());
        }
    };

    for msg in res {
        ctx.http
            .delete_message(u64::from(msg.channel_id), u64::from(msg.id))
            .await
            .unwrap_or_else(|e| println!("Could not delete msg {:?}", e));
    }

    channel_id.say(ctx, format!("Deleted {:?} messages", num))
        .await?;
    Ok(())
}
