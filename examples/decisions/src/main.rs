use async_openai::{
    config::OpenAIConfig,
    types::decisions::{
        ChoiceOptionParam, ChoiceValueParam, DecisionRequestArgs, QuestionParam,
        QuestionParamChoice, QuestionParamInstructions, QuestionParamPredicate, QuestionParamScore,
        ScoreLevelParam,
    },
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    predicate(&client).await?;
    choice(&client).await?;
    score(&client).await?;

    Ok(())
}

async fn predicate(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    // A predicate estimates the probability that a statement about the input is true.
    let request = DecisionRequestArgs::default()
        .model("gpt-6-luna")
        .input(
            "I was charged twice for my subscription this month. Please refund the extra charge.",
        )
        .questions(vec![QuestionParam::Predicate(QuestionParamPredicate {
            name: Some("refund_requested".into()),
            instructions: QuestionParamInstructions::Text(
                "The customer is requesting a refund.".into(),
            ),
        })])
        .build()?;

    let response = client.decisions().create(request).await?;
    println!("Predicate response: {response:#?}");

    Ok(())
}

async fn choice(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    // A choice selects one of the supplied values and includes their probabilities.
    let request = DecisionRequestArgs::default()
        .model("gpt-6-luna")
        .input("I cannot sign in after resetting my password.")
        .questions(vec![QuestionParam::Choice(QuestionParamChoice {
            name: Some("support_team".into()),
            instructions: QuestionParamInstructions::Text(
                "Choose the support team that should handle this customer message.".into(),
            ),
            choices: vec![
                ChoiceOptionParam {
                    value: ChoiceValueParam::Text("billing".into()),
                    description: Some("Payments, invoices, and refunds.".into()),
                },
                ChoiceOptionParam {
                    value: ChoiceValueParam::Text("account".into()),
                    description: Some("Sign-in, passwords, and account access.".into()),
                },
                ChoiceOptionParam {
                    value: ChoiceValueParam::Text("technical".into()),
                    description: Some("Product bugs and service errors.".into()),
                },
            ],
        })])
        .build()?;

    let response = client.decisions().create(request).await?;
    println!("Choice response: {response:#?}");

    Ok(())
}

async fn score(client: &Client<OpenAIConfig>) -> Result<(), Box<dyn Error>> {
    // A score rates the input against ordered levels, from lowest to highest urgency.
    let request = DecisionRequestArgs::default()
        .model("gpt-6-luna")
        .input("Our production checkout is down and no customers can complete their purchases.")
        .questions(vec![QuestionParam::Score(QuestionParamScore {
            name: Some("urgency".into()),
            instructions: QuestionParamInstructions::Text(
                "Rate how urgently this customer issue needs attention.".into(),
            ),
            levels: vec![
                ScoreLevelParam {
                    label: "low".into(),
                    description: Some("A general question with no immediate impact.".into()),
                },
                ScoreLevelParam {
                    label: "medium".into(),
                    description: Some("A problem with an available workaround.".into()),
                },
                ScoreLevelParam {
                    label: "high".into(),
                    description: Some("A critical outage blocking business operations.".into()),
                },
            ],
        })])
        .build()?;

    let response = client.decisions().create(request).await?;
    println!("Score response: {response:#?}");

    Ok(())
}
