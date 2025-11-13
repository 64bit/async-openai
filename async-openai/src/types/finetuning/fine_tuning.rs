use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    error::OpenAIError,
    types::{
        graders::{
            GraderMulti, GraderPython, GraderScoreModel, GraderStringCheck, GraderTextSimilarity,
        },
        Metadata,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum NEpochs {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    NEpochs(u8),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum BatchSize {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    BatchSize(u16),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum LearningRateMultiplier {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    LearningRateMultiplier(f32),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct FineTuneSupervisedHyperparameters {
    /// Number of examples in each batch. A larger batch size means that model parameters
    /// are updated less frequently, but with lower variance.
    pub batch_size: BatchSize,
    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
    /// overfitting.
    pub learning_rate_multiplier: LearningRateMultiplier,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    pub n_epochs: NEpochs,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum Beta {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    Beta(f32),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct FineTuneDPOHyperparameters {
    /// The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.
    pub beta: Beta,
    /// Number of examples in each batch. A larger batch size means that model parameters
    /// are updated less frequently, but with lower variance.
    pub batch_size: BatchSize,
    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
    /// overfitting.
    pub learning_rate_multiplier: LearningRateMultiplier,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    pub n_epochs: NEpochs,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateFineTuningJobRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFineTuningJobRequest {
    /// The name of the model to fine-tune. You can select one of the
    /// [supported models](https://platform.openai.com/docs/guides/fine-tuning#which-models-can-be-fine-tuned).
    pub model: String,

    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/create) for how to upload a file.
    ///
    /// Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// The contents of the file should differ depending on if the model uses the [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input),
    /// [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input)
    /// format, or if the fine-tuning method uses the
    /// [preference](https://platform.openai.com/docs/api-reference/fine-tuning/preference-input) format.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/model-optimization) for more
    /// details.
    pub training_file: String,

    /// A string of up to 64 characters that will be added to your fine-tuned model name.
    ///
    /// For example, a `suffix` of "custom-model-name" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.
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
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/model-optimization) for more
    /// details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,

    /// A list of integrations to enable for your fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<FineTuningIntegration>>,

    /// The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.
    /// If a seed is not specified, one will be generated for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>, // min:0, max: 2147483647

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<FineTuneMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// The method used for fine-tuning.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FineTuneMethod {
    Supervised {
        supervised: FineTuneSupervisedMethod,
    },
    DPO {
        dpo: FineTuneDPOMethod,
    },
    Reinforcement {
        // Boxed because https://rust-lang.github.io/rust-clippy/rust-1.91.0/index.html#large_enum_variant
        reinforcement: Box<FineTuneReinforcementMethod>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneSupervisedMethod {
    pub hyperparameters: FineTuneSupervisedHyperparameters,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneDPOMethod {
    pub hyperparameters: FineTuneDPOHyperparameters,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FineTuneReinforcementMethodGrader {
    StringCheck(GraderStringCheck),
    TextSimilarity(GraderTextSimilarity),
    Python(GraderPython),
    ScoreModel(GraderScoreModel),
    Multi(GraderMulti),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReasoningEffort {
    #[default]
    Default,
    Low,
    Medium,
    High,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum ComputeMultiplier {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    ComputeMultiplier(f32),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum EvalInterval {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    EvalInterval(u32),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum EvalSamples {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    EvalSamples(u32),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneReinforcementHyperparameters {
    /// Number of examples in each batch. A larger batch size means that model parameters
    /// are updated less frequently, but with lower variance.
    pub batch_size: BatchSize,
    /// Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
    /// overfitting.
    pub learning_rate_multiplier: LearningRateMultiplier,
    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    pub n_epochs: NEpochs,
    /// Level of reasoning effort.
    pub reasoning_effort: FineTuneReasoningEffort,
    /// Multiplier on amount of compute used for exploring search space during training.
    pub compute_multiplier: ComputeMultiplier,
    /// The number of training steps between evaluation runs.
    pub eval_interval: EvalInterval,
    /// Number of evaluation samples to generate per training step.
    pub eval_samples: EvalSamples,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneReinforcementMethod {
    pub grader: FineTuneReinforcementMethodGrader,
    pub hyperparameters: FineTuneReinforcementHyperparameters,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FineTuningJobIntegrationType {
    #[default]
    Wandb,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuningIntegration {
    /// The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported.
    pub r#type: FineTuningJobIntegrationType,

    /// The settings for your integration with Weights and Biases. This payload specifies the project that
    /// metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
    /// to your run, and set a default entity (team, username, etc) to be associated with your run.
    pub wandb: WandB,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct WandB {
    /// The name of the project that the new run will be created under.
    pub project: String,
    /// A display name to set for the run. If not set, we will use the Job ID as the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The entity to use for the run. This allows you to set the team or username of the WandB user that you would
    /// like associated with the run. If not set, the default entity for the registered WandB API key is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    /// A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some
    /// default tags are generated by OpenAI: "openai/finetune", "openai/{base-model}", "openai/{ftjob-abcdef}".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Hyperparameters {
    pub batch_size: BatchSize,
    pub learning_rate_multiplier: LearningRateMultiplier,
    pub n_epochs: NEpochs,
}

/// The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: u64,
    /// For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure.
    pub error: Option<FineTuneJobError>,
    /// The name of the fine-tuned model that is being created.
    /// The value will be null if the fine-tuning job is still running.
    pub fine_tuned_model: Option<String>, // nullable: true
    /// The Unix timestamp (in seconds) for when the fine-tuning job was finished.
    /// The value will be null if the fine-tuning job is still running.
    pub finished_at: Option<u64>, // nullable true

    /// The hyperparameters used for the fine-tuning job. This value will only be returned when running
    /// `supervised` jobs.
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

    /// A list of integrations to enable for this fine-tuning job.
    pub integrations: Option<Vec<FineTuningIntegration>>, // maxItems: 5

    /// The seed used for the fine-tuning job.
    pub seed: u32,

    /// The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish. The value will be null if the fine-tuning job is not running.
    pub estimated_finish: Option<u32>,

    pub method: Option<FineTuneMethod>,

    pub metadata: Option<Metadata>,
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
pub struct ListFineTuningJobCheckpointsResponse {
    pub data: Vec<FineTuningJobCheckpoint>,
    pub object: String,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
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
    /// The object identifier.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the fine-tuning job event was created.
    pub created_at: u64,
    /// The log level of the event.
    pub level: Level,
    /// The message of the event.
    pub message: String,
    /// The object type, which is always "fine_tuning.job.event".
    pub object: String,
    /// The type of event.
    pub r#type: Option<FineTuningJobEventType>,
    /// The data associated with the event.
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FineTuningJobEventType {
    Message,
    Metrics,
}

/// The `fine_tuning.job.checkpoint` object represents a model checkpoint for a fine-tuning job that is ready to use.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningJobCheckpoint {
    /// The checkpoint identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    pub created_at: u64,
    /// The name of the fine-tuned checkpoint model that is created.
    pub fine_tuned_model_checkpoint: String,
    /// The step number that the checkpoint was created at.
    pub step_number: u32,
    /// Metrics at the step number during the fine-tuning job.
    pub metrics: FineTuningJobCheckpointMetrics,
    /// The name of the fine-tuning job that this checkpoint was created from.
    pub fine_tuning_job_id: String,
    /// The object type, which is always "fine_tuning.job.checkpoint".
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningJobCheckpointMetrics {
    pub step: u32,
    pub train_loss: f32,
    pub train_mean_token_accuracy: f32,
    pub valid_loss: f32,
    pub valid_mean_token_accuracy: f32,
    pub full_valid_loss: f32,
    pub full_valid_mean_token_accuracy: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateFineTuningCheckpointPermissionRequest {
    /// The project identifiers to grant access to.
    pub project_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListFineTuningCheckpointPermissionResponse {
    pub data: Vec<FineTuningCheckpointPermission>,
    pub object: String,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

/// The `checkpoint.permission` object represents a permission for a fine-tuned model checkpoint.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FineTuningCheckpointPermission {
    /// The permission identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) for when the permission was created.
    pub created_at: u64,
    /// The project identifier that the permission is for.
    pub project_id: String,
    /// The object type, which is always "checkpoint.permission".
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteFineTuningCheckpointPermissionResponse {
    pub object: String,
    pub id: String,
    pub deleted: bool,
}
