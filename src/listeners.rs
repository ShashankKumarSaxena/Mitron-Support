use serenity::{async_trait, model::prelude::*, prelude::*};
use tracing::instrument;

use crate::commands::status::task::reset_bot_status;
use crate::events::{
    guild_create_event::guild_create, guild_member_addition_event::guild_member_addition,
    interaction_create_event::interaction_create, reaction_add_event::reaction_add,
    ready_event::ready,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct Handler {
    pub is_loop_running: AtomicBool,
}

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

    // Interaction Handling
    #[instrument(skip(self, _ctx))]
    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {
        interaction_create(_ctx, _interaction).await;
    }

    // Reaction Add Event
    async fn reaction_add(&self, _ctx: Context, _reaction: Reaction) {
        reaction_add(&_ctx, _reaction).await;
    }

    // Cache Ready Event
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            tokio::spawn(async move {
                loop {
                    reset_bot_status(Arc::clone(&ctx)).await;
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                }
            });

            self.is_loop_running.store(true, Ordering::Relaxed);
        }
    }
}
