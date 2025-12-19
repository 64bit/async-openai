use std::fmt::Display;

use crate::types::files::{FileExpirationAfterAnchor, FilePurpose};

impl Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Assistants => "assistants",
                Self::Batch => "batch",
                Self::FineTune => "fine-tune",
                Self::Vision => "vision",
                Self::UserData => "user_data",
                Self::Evals => "evals",
            }
        )
    }
}

impl Display for FileExpirationAfterAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CreatedAt => "created_at",
            }
        )
    }
}
