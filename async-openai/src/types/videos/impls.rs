use std::fmt::Display;

use crate::types::videos::VideoSize;

impl Display for VideoSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::S720x1280 => "720x1280",
                Self::S1280x720 => "1280x720",
                Self::S1024x1792 => "1024x1792",
                Self::S1792x1024 => "1792x1024",
            }
        )
    }
}
