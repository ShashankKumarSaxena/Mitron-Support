use serenity::{async_trait, model::prelude::*, prelude::*};
use tracing::instrument;

use crate::events::{
    guild_create_event::guild_create, guild_member_addition_event::guild_member_addition,
    ready_event::ready,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Ready Event
    #[instrument(skip(self, _ctx))]
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        ready(_ctx, _ready).await;
    }

    // Guild Member Join Event
    #[instrument(skip(self, _ctx))]
    async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {
        guild_member_addition(_ctx, _guild_id, _new_member).await;
    }

    // Guild Create Event
    #[instrument(skip(self, _ctx, _guild))]
    async fn guild_create(&self, _ctx: Context, _guild: Guild, _is_new: bool) {
        guild_create(_ctx, _guild, _is_new).await;
    }
}
