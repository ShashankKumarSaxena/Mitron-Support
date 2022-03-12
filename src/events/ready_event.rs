use serenity::{model::prelude::*, prelude::*, model::gateway::Activity};
use tracing::{error, info};

pub async fn ready(ctx: Context, ready: Ready) {
    ctx.set_activity(Activity::watching("Miत्रों Support Server")).await;

    info!("[CORE] {} is connected!", ready.user.name);
}