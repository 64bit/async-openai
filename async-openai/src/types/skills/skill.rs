use serde::{Deserialize, Serialize};

use crate::types::InputSource;

/// Represents a skill object.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SkillResource {
    /// Unique identifier for the skill.
    pub id: String,
    /// The object type, which is always `skill`.
    pub object: String,
    /// Name of the skill.
    pub name: String,
    /// Description of the skill.
    pub description: String,
    /// Unix timestamp (in seconds) for when the skill was created.
    pub created_at: u64,
    /// Default version for the skill.
    pub default_version: String,
    /// Latest version for the skill.
    pub latest_version: String,
}

/// List of skills.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SkillListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// A list of skill objects.
    pub data: Vec<SkillResource>,
    /// The ID of the first item in the list.
    pub first_id: Option<String>,
    /// The ID of the last item in the list.
    pub last_id: Option<String>,
    /// Whether there are more items available.
    pub has_more: bool,
}

/// Request to create a skill by uploading files.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreateSkillRequest {
    /// Skill files to upload (directory upload) or a single zip file.
    pub files: Vec<InputSource>,
}

/// Request to update the default version pointer for a skill.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetDefaultSkillVersionRequest {
    /// The skill version number to set as default.
    pub default_version: String,
}

/// Confirmation of skill deletion.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeletedSkillResource {
    /// The object type, which is always `skill.deleted`.
    pub object: String,
    /// Whether the skill was deleted.
    pub deleted: bool,
    /// The skill ID.
    pub id: String,
}

/// Represents a skill version object.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SkillVersionResource {
    /// The object type, which is always `skill.version`.
    pub object: String,
    /// Unique identifier for the skill version.
    pub id: String,
    /// Identifier of the skill for this version.
    pub skill_id: String,
    /// Version number for this skill.
    pub version: String,
    /// Unix timestamp (in seconds) for when the skill version was created.
    pub created_at: u64,
    /// Name of the skill version.
    pub name: String,
    /// Description of the skill version.
    pub description: String,
}

/// List of skill versions.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SkillVersionListResource {
    /// The object type, which is always `list`.
    pub object: String,
    /// A list of skill version objects.
    pub data: Vec<SkillVersionResource>,
    /// The ID of the first item in the list.
    pub first_id: Option<String>,
    /// The ID of the last item in the list.
    pub last_id: Option<String>,
    /// Whether there are more items available.
    pub has_more: bool,
}

/// Request to create a new skill version by uploading files.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreateSkillVersionRequest {
    /// Skill files to upload (directory upload) or a single zip file.
    pub files: Vec<InputSource>,
    /// Whether to set this version as the default.
    pub default: Option<bool>,
}

/// Confirmation of skill version deletion.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DeletedSkillVersionResource {
    /// The object type, which is always `skill.version.deleted`.
    pub object: String,
    /// Whether the skill version was deleted.
    pub deleted: bool,
    /// The skill version ID.
    pub id: String,
    /// The deleted skill version number.
    pub version: String,
}
