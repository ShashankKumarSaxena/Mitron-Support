use serenity::{async_trait, model::prelude::*, prelude::*};
use tracing::instrument;

use crate::events::{ready_event::ready, guild_create_event::guild_create};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Ready Event
    #[instrument(skip(self, _ctx))]
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        ready(_ctx, _ready).await;
    }

    // Guild Member Join Event

    // Guild Create Event
    #[instrument(skip(self, _ctx, _guild))]
    async fn guild_create(&self, _ctx: Context, _guild: Guild, _is_new: bool) {
        guild_create(_ctx, _guild, _is_new).await;
    }
}
