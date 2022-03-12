use serenity::{model::gateway::Activity, model::prelude::*, prelude::*};
use tracing::{error, info};

pub async fn ready(ctx: Context, ready: Ready) {
    ctx.set_activity(Activity::watching("Miत्रों Support Server"))
        .await;

    info!("[CORE] {} is connected!", ready.user.name);
}
