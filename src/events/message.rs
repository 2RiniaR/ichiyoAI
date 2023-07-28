use anyhow::Context as _;
use chatgpt::config::ChatGPTEngine;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::api::chatgpt::chat_completion;
use crate::api::discord::{edit_response, edit_response_with_file, reply};
use crate::utils::create_temp_file;

pub async fn chat_ai(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    let waiting_message = reply(ctx, msg, "思考中... 🤔").await;

    let response = chat_completion(&msg.content, Some(ChatGPTEngine::Gpt4))
        .await
        .context("ChatGPT APIとのやり取りに失敗しました。")?;
    let response_content = &response.message().content;

    match response_content.chars().count() {
        count if count > 2000 => {
            create_temp_file(response_content.to_string());
            edit_response_with_file(ctx, waiting_message, "temp/temp.txt").await;
        }
        _ => edit_response(ctx, waiting_message, response_content).await,
    }

    Ok(())
}
