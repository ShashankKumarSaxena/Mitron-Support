// A custom paginator implementation with buttons

use serenity::{
    builder::CreateMessage,
    builder::{CreateActionRow, CreateButton},
    model::interactions::message_component::ButtonStyle,
    model::{
        channel::{Message, ReactionType},
        interactions::InteractionResponseType,
    },
    prelude::*,
};
use std::time::Duration;

struct PaginatorButtons {
    guild_id: u64,
}

impl PaginatorButtons {
    fn get_buttons(&self) -> Vec<CreateButton> {
        let mut buttons: Vec<CreateButton> = vec![];

        let mut backward_button = CreateButton::default();
        backward_button.emoji(ReactionType::from('⏪'));
        backward_button.style(ButtonStyle::Primary);
        backward_button.custom_id(format!("backward_btn:{}", self.guild_id));
        buttons.push(backward_button);

        let mut back = CreateButton::default();
        back.emoji(ReactionType::from('◀'));
        back.style(ButtonStyle::Primary);
        back.custom_id(format!("back_btn:{}", self.guild_id));
        buttons.push(back);

        let mut stop = CreateButton::default();
        stop.emoji(ReactionType::from('⏹'));
        stop.style(ButtonStyle::Primary);
        stop.custom_id(format!("stop_btn:{}", self.guild_id));
        buttons.push(stop);

        let mut next = CreateButton::default();
        next.emoji(ReactionType::from('▶'));
        next.custom_id(format!("next_btn:{}", self.guild_id));
        next.style(ButtonStyle::Primary);
        buttons.push(next);

        let mut forward_button = CreateButton::default();
        forward_button.emoji(ReactionType::from('⏩'));
        forward_button.style(ButtonStyle::Primary);
        forward_button.custom_id(format!("forward_btn:{}", self.guild_id));
        buttons.push(forward_button);

        buttons
    }
}

pub async fn paginate(ctx: &Context, msg: &Message, pages: Vec<CreateMessage<'_>>) {
    let buttons = PaginatorButtons {
        guild_id: msg.guild_id.unwrap().0,
    };
    let mut paginate_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.clone_from(&pages[0]);
            m.components(|c| {
                let mut action_row = CreateActionRow::default();
                for button in buttons.get_buttons().iter() {
                    action_row.add_button(button.clone());
                }
                c.add_action_row(action_row);
                c
            });
            m
        })
        .await
        .unwrap();

    let mut current_page_idx: usize = 0;
    // Main paginator logic
    loop {
        if let Some(control) = &paginate_msg
            .await_component_interaction(&ctx)
            .timeout(Duration::from_secs(50))
            .await
        {
            let custom_id = &control.data.custom_id;

            if custom_id.as_ref() == format!("backward_btn:{}", control.guild_id.unwrap()) {
                control
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::UpdateMessage);
                        r.interaction_response_data(|d| {
                            d.0.clone_from(&pages[0].0);
                            d
                        });
                        r
                    })
                    .await
                    .unwrap();
                current_page_idx = 0;
            } else if custom_id.as_ref() == format!("back_btn:{}", control.guild_id.unwrap()) {
                if (current_page_idx - 1) < 0 {
                } else {
                    control
                        .create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseType::UpdateMessage);
                            r.interaction_response_data(|d| {
                                d.0.clone_from(&pages[current_page_idx - 1].0);
                                d
                            });
                            r
                        })
                        .await
                        .unwrap();
                    current_page_idx -= 1;
                }
            } else if custom_id.as_ref() == format!("stop_btn:{}", control.guild_id.unwrap()) {
                paginate_msg.delete(&ctx.http).await.unwrap();
                break;
            } else if custom_id.as_ref() == format!("next_btn:{}", control.guild_id.unwrap()) {
                if (current_page_idx + 1) >= pages.len() {
                } else {
                    control
                        .create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseType::UpdateMessage);
                            r.interaction_response_data(|d| {
                                d.0.clone_from(&pages[current_page_idx + 1].0);
                                d
                            });
                            r
                        })
                        .await
                        .unwrap();
                    current_page_idx += 1;
                }
            } else if custom_id.as_ref()
                == format!("{}", format!("forward_btn:{}", control.guild_id.unwrap()))
            {
                control
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::UpdateMessage);
                        r.interaction_response_data(|d| {
                            d.0.clone_from(&pages[pages.len() - 1].0);
                            d
                        });
                        r
                    })
                    .await
                    .unwrap();
                current_page_idx = pages.len() - 1;
            } else {
                paginate_msg.delete(&ctx.http).await.unwrap();
                break;
            }
        }
    }
}
