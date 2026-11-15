use std::error::Error;

use async_openai::{
    types::chat::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use llm_budget::AgentBudget;

// Ask gpt-4o-mini five questions against a $0.0001 budget: each call costs
// about $0.00002-0.00003 at $0.15/M input + $0.60/M output, so the last
// questions are refused before any money is spent.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let budget = AgentBudget::builder("chat", 0.0001).build()?;
    let model = "gpt-4o-mini";

    for question in [
        "What is the capital of France?",
        "And of Japan?",
        "And of Brazil?",
        "And of Canada?",
        "And of Australia?",
    ] {
        let request = CreateChatCompletionRequestArgs::default()
            .max_completion_tokens(64u32)
            .model(model)
            .messages([
                ChatCompletionRequestSystemMessage::from("Answer in one word.").into(),
                ChatCompletionRequestUserMessage::from(question).into(),
            ])
            .build()?;

        // Refuse before spending when the cap is gone.
        if budget.is_exhausted() {
            println!("{question} -> skipped: ${:.4} of ${:.4} spent", budget.spent_usd(), budget.limit_usd());
            continue;
        }

        let response = client.chat().create(request).await?;

        // Book the real usage at the model's real price.
        if let Some(usage) = response.usage {
            let receipt = budget
                .charge_model(
                    model,
                    usage.prompt_tokens,
                    usage.completion_tokens,
                    None,
                    None,
                )
                .await?;
            let answer = response
                .choices
                .first()
                .and_then(|c| c.message.content.clone())
                .unwrap_or_default();
            println!(
                "{question} -> {answer} (${:.6} this call, ${:.6} total)",
                receipt.cost_usd, receipt.total_spent_usd
            );
        }
    }

    Ok(())
}
