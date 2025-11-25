use serde::Deserialize;

/// Response structure for organization usage endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageResponse {
    /// The object type, which is always `page`.
    pub object: String,
    /// List of time buckets containing usage data.
    pub data: Vec<UsageTimeBucket>,
    /// Whether there are more pages available.
    pub has_more: bool,
    /// Cursor for the next page.
    pub next_page: Option<String>,
}

/// A time bucket containing usage results.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageTimeBucket {
    /// The object type, which is always `bucket`.
    pub object: String,
    /// Start time of the bucket (Unix seconds).
    pub start_time: u64,
    /// End time of the bucket (Unix seconds).
    pub end_time: u64,
    /// Start time of the bucket in ISO 8601 format.
    #[serde(default)]
    pub start_time_iso: Option<String>,
    /// End time of the bucket in ISO 8601 format.
    #[serde(default)]
    pub end_time_iso: Option<String>,
    /// Usage results for this time bucket.
    pub results: Vec<UsageResult>,
}

/// Discriminated union of all possible usage result types.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum UsageResult {
    AudioSpeeches(UsageAudioSpeechesResult),
    AudioTranscriptions(UsageAudioTranscriptionsResult),
    CodeInterpreterSessions(UsageCodeInterpreterSessionsResult),
    Completions(UsageCompletionsResult),
    Embeddings(UsageEmbeddingsResult),
    Images(UsageImagesResult),
    Moderations(UsageModerationsResult),
    VectorStores(UsageVectorStoresResult),
    Costs(CostsResult),
}

/// The aggregated audio speeches usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageAudioSpeechesResult {
    /// The object type, which is always `organization.usage.audio_speeches.result`.
    pub object: String,
    /// The number of characters processed.
    pub characters: u64,
    /// The count of requests made to the model.
    pub num_model_requests: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
}

/// The aggregated audio transcriptions usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageAudioTranscriptionsResult {
    /// The object type, which is always `organization.usage.audio_transcriptions.result`.
    pub object: String,
    /// The number of seconds processed.
    pub seconds: u64,
    /// The count of requests made to the model.
    pub num_model_requests: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
}

/// The aggregated code interpreter sessions usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageCodeInterpreterSessionsResult {
    /// The object type, which is always `organization.usage.code_interpreter_sessions.result`.
    pub object: String,
    /// The number of code interpreter sessions.
    pub num_sessions: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
}

/// The aggregated completions usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageCompletionsResult {
    /// The object type, which is always `organization.usage.completions.result`.
    pub object: String,
    /// The aggregated number of text input tokens used, including cached tokens. For customers subscribe to scale tier, this includes scale tier tokens.
    pub input_tokens: u64,
    /// The aggregated number of text output tokens used. For customers subscribe to scale tier, this includes scale tier tokens.
    pub output_tokens: u64,
    /// The aggregated number of text input tokens that has been cached from previous requests. For customers subscribe to scale tier, this includes scale tier tokens.
    #[serde(default)]
    pub input_cached_tokens: Option<u64>,
    /// The aggregated number of uncached input tokens.
    #[serde(default)]
    pub input_uncached_tokens: Option<u64>,
    /// The aggregated number of text input tokens used.
    #[serde(default)]
    pub input_text_tokens: Option<u64>,
    /// The aggregated number of text output tokens used.
    #[serde(default)]
    pub output_text_tokens: Option<u64>,
    /// The aggregated number of cached text input tokens.
    #[serde(default)]
    pub input_cached_text_tokens: Option<u64>,
    /// The aggregated number of audio input tokens used, including cached tokens.
    #[serde(default)]
    pub input_audio_tokens: Option<u64>,
    /// The aggregated number of cached audio input tokens.
    #[serde(default)]
    pub input_cached_audio_tokens: Option<u64>,
    /// The aggregated number of audio output tokens used.
    #[serde(default)]
    pub output_audio_tokens: Option<u64>,
    /// The aggregated number of image input tokens used.
    #[serde(default)]
    pub input_image_tokens: Option<u64>,
    /// The aggregated number of cached image input tokens.
    #[serde(default)]
    pub input_cached_image_tokens: Option<u64>,
    /// The aggregated number of image output tokens used.
    #[serde(default)]
    pub output_image_tokens: Option<u64>,
    /// The count of requests made to the model.
    pub num_model_requests: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
    /// When `group_by=batch`, this field tells whether the grouped usage result is batch or not.
    pub batch: Option<bool>,
    /// When `group_by=service_tier`, this field provides the service tier of the grouped usage result.
    pub service_tier: Option<String>,
}

/// The aggregated embeddings usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageEmbeddingsResult {
    /// The object type, which is always `organization.usage.embeddings.result`.
    pub object: String,
    /// The aggregated number of input tokens used.
    pub input_tokens: u64,
    /// The count of requests made to the model.
    pub num_model_requests: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
}

/// The aggregated images usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageImagesResult {
    /// The object type, which is always `organization.usage.images.result`.
    pub object: String,
    /// The number of images processed.
    pub images: u64,
    /// The count of requests made to the model.
    pub num_model_requests: u64,
    /// When `group_by=source`, this field provides the source of the grouped usage result, possible values are `image.generation`, `image.edit`, `image.variation`.
    pub source: Option<String>,
    /// When `group_by=size`, this field provides the image size of the grouped usage result.
    pub size: Option<String>,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
}

/// The aggregated moderations usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageModerationsResult {
    /// The object type, which is always `organization.usage.moderations.result`.
    pub object: String,
    /// The aggregated number of input tokens used.
    pub input_tokens: u64,
    /// The count of requests made to the model.
    pub num_model_requests: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
    /// When `group_by=user_id`, this field provides the user ID of the grouped usage result.
    pub user_id: Option<String>,
    /// When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result.
    pub api_key_id: Option<String>,
    /// When `group_by=model`, this field provides the model name of the grouped usage result.
    pub model: Option<String>,
}

/// The aggregated vector stores usage details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct UsageVectorStoresResult {
    /// The object type, which is always `organization.usage.vector_stores.result`.
    pub object: String,
    /// The vector stores usage in bytes.
    pub usage_bytes: u64,
    /// When `group_by=project_id`, this field provides the project ID of the grouped usage result.
    pub project_id: Option<String>,
}

/// The aggregated costs details of the specific time bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct CostsResult {
    /// The object type, which is always `organization.costs.result`.
    pub object: String,
    /// The monetary value in its associated currency.
    pub amount: CostsAmount,
    /// When `group_by=line_item`, this field provides the line item of the grouped costs result.
    pub line_item: Option<String>,
    /// When `group_by=project_id`, this field provides the project ID of the grouped costs result.
    pub project_id: Option<String>,
    /// The organization ID.
    #[serde(default)]
    pub organization_id: Option<String>,
}

/// The monetary value in its associated currency.
#[derive(Debug, Clone, Deserialize)]
pub struct CostsAmount {
    /// The numeric value of the cost.
    pub value: f64,
    /// Lowercase ISO-4217 currency e.g. "usd"
    pub currency: String,
}
