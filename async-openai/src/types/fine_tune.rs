use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::client::OpenAIEventStream;

use crate::error::OpenAIError;

use super::OpenAIFile;

#[derive(Debug, Serialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateFineTuneRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateFineTuneRequest {
    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload file](https://platform.openai.com/docs/api-reference/files/upload) for how to upload a file.
    ///
    /// Your dataset must be formatted as a JSONL file, where each training
    /// example is a JSON object with the keys "prompt" and "completion".
    /// Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning/creating-training-data) for more details.
    pub training_file: String,

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation
    /// metrics periodically during fine-tuning. These metrics can be viewed in
    /// the [fine-tuning results file](https://platform.openai.com/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).
    /// Your train and validation data should be mutually exclusive.
    ///
    /// Your dataset must be formatted as a JSONL file, where each validation
    /// example is a JSON object with the keys "prompt" and "completion".
    /// Additionally, you must upload your file with the purpose `fine-tune`.
    ///
    /// See the [fine-tuning guide](https://platform.openai.com/docs/guides/fine-tuning/creating-training-data) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,

    /// The name of the base model to fine-tune. You can select one of "ada",
    /// "babbage", "curie", "davinci", or a fine-tuned model created after 2022-04-21.
    /// To learn more about these models, see the [Models](https://platform.openai.com/docs/models) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The number of epochs to train the model for. An epoch refers to one
    /// full cycle through the training dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<u32>, // default: 4

    /// The batch size to use for training. The batch size is the number of
    /// training examples used to train a single forward and backward pass.
    ///
    /// By default, the batch size will be dynamically configured to be
    /// ~0.2% of the number of examples in the training set, capped at 256 -
    /// in general, we've found that larger batch sizes tend to work better
    /// for larger datasets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>, // default: null

    /// The learning rate multiplier to use for training.
    /// The fine-tuning learning rate is the original learning rate used for
    /// pretraining multiplied by this value.
    ///
    /// By default, the learning rate multiplier is the 0.05, 0.1, or 0.2
    /// depending on final `batch_size` (larger learning rates tend to
    /// perform better with larger batch sizes). We recommend experimenting
    /// with values in the range 0.02 to 0.2 to see what produces the best
    /// results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<f32>, // default: null

    /// The weight to use for loss on the prompt tokens. This controls how
    /// much the model tries to learn to generate the prompt (as compared
    /// to the completion which always has a weight of 1.0), and can add
    /// a stabilizing effect to training when completions are short.
    ///
    /// If prompts are extremely long (relative to completions), it may make
    /// sense to reduce this weight so as to avoid over-prioritizing
    /// learning the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_loss_weight: Option<f32>, // default: 0.01

    /// If set, we calculate classification-specific metrics such as accuracy
    /// and F-1 score using the validation set at the end of every epoch.
    /// These metrics can be viewed in the [results file](https://platform.openai.com/docs/guides/fine-tuning/analyzing-your-fine-tuned-model).
    ///
    /// In order to compute classification metrics, you must provide a
    /// `validation_file`. Additionally, you must
    /// specify `classification_n_classes` for multiclass classification or
    /// `classification_positive_class` for binary classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_classification_metrics: Option<bool>, // default: false

    /// The number of classes in a classification task.
    ///
    /// This parameter is required for multiclass classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_n_classes: Option<u32>, // default: null

    /// The positive class in binary classification.
    ///
    /// This parameter is needed to generate precision, recall, and F1
    /// metrics when doing binary classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_positive_class: Option<String>, // default: null

    /// If this is provided, we calculate F-beta scores at the specified
    /// beta values. The F-beta score is a generalization of F-1 score.
    /// This is only used for binary classification.
    ///
    /// With a beta of 1 (i.e. the F-1 score), precision and recall are
    /// given the same weight. A larger beta score puts more weight on
    /// recall and less on precision. A smaller beta score puts more weight
    /// on precision and less on recall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_betas: Option<Vec<f32>>, // default: null

    /// A string of up to 40 characters that will be added to your fine-tuned model name.
    ///
    /// For example, a `suffix` of "custom-model-name" would produce a model name like `ada:ft-your-org:custom-model-name-2022-02-15-04-21-04`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>, // default: null, minLength:1, maxLength:40
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFineTuneResponse {
    pub object: String,
    pub data: Vec<FineTune>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTune {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub updated_at: u32,
    pub model: String,
    pub fine_tuned_model: Option<String>, // nullable: true
    pub organization_id: String,
    pub status: String,
    pub hyperparams: serde_json::Value,
    pub training_files: Vec<OpenAIFile>,
    pub validation_files: Vec<OpenAIFile>,
    pub result_files: Vec<OpenAIFile>,
    pub events: Option<Vec<FineTuneEvent>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FineTuneEvent {
    pub object: String,
    pub created_at: u32,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFineTuneEventsResponse {
    pub object: String,
    pub data: Vec<FineTuneEvent>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ListFineTuneEventsStreamResponse {
    pub object: String,
    pub data: Option<Vec<FineTuneEvent>>,
}

/// Parsed server side events stream until an \[DONE\] is received from server.
pub type FineTuneEventsResponseStream = OpenAIEventStream<ListFineTuneEventsStreamResponse>;
