use serenity::{async_trait, model::prelude::*, prelude::*};
use tracing::instrument;

use crate::events::ready_event::ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip(self, _ctx))]
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        ready(_ctx, _ready).await;
    }
}