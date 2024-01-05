use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(untagged)]
pub enum NEpochs {
    NEpochs(u8),
    #[default]
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Hyperparameters {
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    pub n_epochs: NEpochs,
}

#[derive(Debug, Serialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateFineTuningJobRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFineTuningJobRequest {
    /// The name of the model to fine-tune. You can select one of the
    /// [supported models](https://platform.openai.com/docs/guides/fine-tuning/what-models-can-be-fine-tuned).
    pub model: String,

    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/upload) for how to upload a file.
    ///
    /// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    pub training_file: String,

    /// The hyperparameters used for the fine-tuning job.
    pub hyperparameters: Option<Hyperparameters>,

    /// A string of up to 18 characters that will be added to your fine-tuned model name.
    ///
    /// For example, a `suffix` of "custom-model-name" would produce a model name
    /// like `ft:gpt-3.5-turbo:openai:custom-model-name:7p4lURel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>, // default: null, minLength:1, maxLength:40

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation
    /// metrics periodically during fine-tuning. These metrics can be viewed in
    /// the fine-tuning results file.
    /// The same data should not be present in both train and validation files.
    ///
    /// Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
}

/// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneJobError {
    ///  A machine-readable error code.
    pub code: String,
    ///  A human-readable error message.
    pub message: String,
    /// The parameter that was invalid, usually `training_file` or `validation_file`.
    /// This field will be null if the failure was not parameter-specific.
    pub param: Option<String>, // nullable true
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FineTuningJobStatus {
    ValidatingFiles,
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: u32,
    /// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
    pub error: Option<FineTuneJobError>,
    /// The name of the fine-tuned model that is being created.
    /// The value will be null if the fine-tuning job is still running.
    pub fine_tuned_model: Option<String>, // nullable: true
    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished.
    /// The value will be null if the fine-tuning job is still running.
    pub finished_at: Option<u32>, // nullable true

    /// The hyperparameters used for the fine-tuning job.
    /// See the [fine-tuning guide](/docs/guides/fine-tuning) for more details.
    pub hyperparameters: Hyperparameters,

    ///  The base model that is being fine-tuned.
    pub model: String,

    /// The object type, which is always "fine_tuning.job".
    pub object: String,
    /// The organization that owns the fine-tuning job.
    pub organization_id: String,

    /// The compiled results file ID(s) for the fine-tuning job.
    /// You can retrieve the results with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub result_files: Vec<String>,

    /// The current status of the fine-tuning job, which can be either
    /// `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`.
    pub status: FineTuningJobStatus,

    /// The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running.
    pub trained_tokens: Option<u32>,

    /// The file ID used for training. You can retrieve the training data with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub training_file: String,

    ///  The file ID used for validation. You can retrieve the validation results with the [Files API](https://platform.openai.com/docs/api-reference/files/retrieve-contents).
    pub validation_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListPaginatedFineTuningJobsResponse {
    pub data: Vec<FineTuningJob>,
    pub has_more: bool,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFineTuningJobEventsResponse {
    pub data: Vec<FineTuningJobEvent>,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Info,
    Warn,
    Error,
}

///Fine-tuning job event object
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningJobEvent {
    pub id: String,
    pub created_at: u32,
    pub level: Level,
    pub message: String,
    pub object: String,
}
