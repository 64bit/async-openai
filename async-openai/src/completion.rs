use std::pin::Pin;

use futures::{stream::StreamExt, Stream};
use reqwest_eventsource::{Event, RequestBuilderExt};

use crate::{
    client::Client,
    error::OpenAIError,
    types::{CreateCompletionRequest, CreateCompletionResponse},
};

/// Given a prompt, the model will return one or more predicted
/// completions, and can also return the probabilities of alternative
/// tokens at each position.
pub struct Completion;

/// Parsed server side events stream until an [DONE] is received from server.
pub type CompletionResponseStream =
    Pin<Box<dyn Stream<Item = Result<CreateCompletionResponse, OpenAIError>>>>;

impl Completion {
    /// Creates a completion for the provided prompt and parameters
    pub async fn create(
        client: &Client,
        request: CreateCompletionRequest,
    ) -> Result<CreateCompletionResponse, OpenAIError> {
        if request.stream.is_some() && request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is true, use Completion::create_stream".into(),
            ));
        }
        client.post("/completions", request).await
    }

    /// Creates a completion request for the provided prompt and parameters
    ///
    /// Stream back partial progress. Tokens will be sent as data-only
    /// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)
    /// as they become available, with the stream terminated by a data: [DONE] message.
    ///
    /// [CompletionResponseStream] is a parsed SSE stream until a [DONE] is received from server.
    pub async fn create_stream(
        client: &Client,
        mut request: CreateCompletionRequest,
    ) -> Result<CompletionResponseStream, OpenAIError> {
        if request.stream.is_some() && !request.stream.unwrap() {
            return Err(OpenAIError::InvalidArgument(
                "When stream is false, use Completion::create".into(),
            ));
        }

        request.stream = Some(true);

        let mut event_source = reqwest::Client::new()
            .post(format!("{}/completions", client.api_base()))
            .bearer_auth(client.api_key())
            .json(&request)
            .eventsource()
            .unwrap();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(ev) = event_source.next().await {
                match ev {
                    Err(e) => {
                        if let Err(_e) = tx.send(Err(OpenAIError::StreamError(e.to_string()))) {
                            // rx dropped
                            break;
                        }
                    }
                    Ok(event) => match event {
                        Event::Message(message) => {
                            if message.data == "[DONE]" {
                                break;
                            }

                            let response = match serde_json::from_str::<CreateCompletionResponse>(
                                &message.data,
                            ) {
                                Err(e) => Err(OpenAIError::JSONDeserialize(e)),
                                Ok(ccr) => Ok(ccr),
                            };

                            if let Err(_e) = tx.send(response) {
                                // rx dropped
                                break;
                            }
                        }
                        Event::Open => continue,
                    },
                }
            }

            event_source.close();
        });

        Ok(
            Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
                as CompletionResponseStream,
        )
    }
}
