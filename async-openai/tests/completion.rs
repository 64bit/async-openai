//! This test is primarily to make sure that macros_rules for From traits are correct.
use async_openai::{
    error::OpenAIError,
    types::{CreateCompletionRequest, CreateCompletionRequestArgs, Prompt},
};

fn request<T>(prompt: T) -> Result<CreateCompletionRequest, OpenAIError>
where
    Prompt: From<T>,
{
    CreateCompletionRequestArgs::default()
        .prompt(prompt)
        .max_tokens(10_u16)
        .temperature(1.0)
        .n(1)
        .stream(false)
        .stop("stop")
        .user("async-openai")
        .build()
}

#[test]
fn create_completion_request() {
    let prompt = "This is &str prompt";
    let _ = request(prompt);

    let prompt = "This is String".to_string();
    let _ = request(&prompt);
    let _ = request(prompt);

    let prompt = vec!["This is first", "This is second"];
    let _ = request(&prompt);
    let _ = request(prompt);

    let prompt = vec!["First string".to_string(), "Second string".to_string()];
    let _ = request(&prompt);
    let _ = request(prompt);

    let first = "First".to_string();
    let second = "Second".to_string();
    let prompt = vec![&first, &second];
    let _ = request(&prompt);
    let _ = request(prompt);

    let prompt = ["first", "second"];
    let _ = request(&prompt);
    let _ = request(prompt);

    let prompt = ["first".to_string(), "second".to_string()];
    let _ = request(&prompt);
    let _ = request(prompt);

    let first = "First".to_string();
    let second = "Second".to_string();
    let prompt = [&first, &second];
    let _ = request(&prompt);
    let _ = request(prompt);
}
