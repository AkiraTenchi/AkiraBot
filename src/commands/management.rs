use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::Context;

#[command]
async fn delete_msg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel_id = msg.channel_id;
    let num;

    if args.is_empty() {
        num = 2
    } else {
        num = args.parse::<u64>().unwrap() + 1;
    }

    if num > 100 {
        channel_id
            .say(ctx, "Maximum number of deletable messages is 99")
            .await?;
        return Ok(());
    }

    let messages = ctx
        .http
        .get_messages(*channel_id.as_u64(), &format!("?limit={:?}", num))
        .await
        .expect("Failed to get Messages in channel");

    for msg in messages {
        ctx.http
            .delete_message(*msg.channel_id.as_u64(), *msg.id.as_u64())
            .await
            .unwrap_or_else(|e| println!("Could not delete msg {:?}", e));
    }

    channel_id
        .say(ctx, format!("Deleted {:?} messages", num - 1))
        .await?;
    Ok(())
}

#[command]
async fn nick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args.clone();

    let author_perms = get_member_permissions_from_msg(&ctx, &msg).await;

    if author_perms.contains(Permissions::KICK_MEMBERS)
        || author_perms.contains(Permissions::ADMINISTRATOR)
    {
        let user_id = args.single::<UserId>().expect("userId");
        let new_nick = args.single::<String>().expect("new nick");

        let member = get_member_from_user_id(&ctx, &msg, user_id).await;

        member
            .edit(&ctx.http, |e| e.nickname(&new_nick))
            .await
            .expect("edit member");
        msg.reply(
            &ctx.http,
            format!(
                "{} changed {}s username from {} to {}",
                msg.author.name,
                member.user.name,
                member.display_name(),
                new_nick
            ),
        )
        .await?;
    } else {
        msg.reply(
            &ctx.http,
            "You do not have the permissions required to use this command!",
        )
        .await?;
    }

    Ok(())
}

#[command]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mem_perms = get_member_permissions_from_msg(&ctx, &msg).await;

    if mem_perms.contains(Permissions::KICK_MEMBERS)
        || mem_perms.contains(Permissions::ADMINISTRATOR)
    {
        let target_id = args
            .single::<UserId>()
            .expect("Failed to get target from msg");

        let target = get_member_from_user_id(&ctx, &msg, target_id).await;

        target.kick(ctx).await.expect("failed to kick member");
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "{} has been kicked by {}",
                    target.user.name,
                    msg.author.name
                ),
            )
            .await?;
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "You do not have the Permissions required to use this command",
            )
            .await?;
    }

    Ok(())
}

#[command]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mem_perms = get_member_permissions_from_msg(&ctx, &msg).await;

    if mem_perms.contains(Permissions::BAN_MEMBERS)
        || mem_perms.contains(Permissions::ADMINISTRATOR)
    {
        let target_id = args
            .single::<UserId>()
            .expect("Failed to get target from msg");

        let target = get_member_from_user_id(&ctx, &msg, target_id).await;

        target
            .ban(&ctx.http, 7)
            .await
            .expect("failed to ban member");

        msg.channel_id
            .say(
                &ctx.http,
                format!("{} has banned {}", msg.author.name, target.user.name),
            )
            .await?;
    } else {
        msg.channel_id
            .say(&ctx.http, "You do not have Permission to use this Command")
            .await?;
    }
    Ok(())
}

async fn get_member_permissions_from_msg(ctx: &Context, msg: &Message) -> Permissions {
    let member = msg.member(&ctx).await.expect("Failed to retrieve Member");
    let member_role_id = member.highest_role_info(&ctx.cache).await;

    match member_role_id {
        Some((ri, _num)) => {
            ri.to_role_cached(&ctx.cache)
                .await
                .expect("Role")
                .permissions
        }
        None => Permissions::empty(),
    }
}

async fn get_member_from_user_id(ctx: &Context, msg: &Message, user_id: UserId) -> Member {
    ctx.http
        .get_member(*msg.guild_id.expect("guild id").as_u64(), *user_id.as_u64())
        .await
        .expect("User")
}
