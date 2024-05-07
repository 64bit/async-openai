use super::{
    AssistantTools, AssistantToolsCode, AssistantToolsFileSearch, AssistantToolsFunction,
    FunctionObject,
};

impl From<AssistantToolsCode> for AssistantTools {
    fn from(value: AssistantToolsCode) -> Self {
        Self::Code(value)
    }
}

impl From<AssistantToolsFileSearch> for AssistantTools {
    fn from(value: AssistantToolsFileSearch) -> Self {
        Self::FileSearch(value)
    }
}

impl From<AssistantToolsFunction> for AssistantTools {
    fn from(value: AssistantToolsFunction) -> Self {
        Self::Function(value)
    }
}

impl Default for AssistantToolsCode {
    fn default() -> Self {
        Self {
            r#type: "code_interpreter".into(),
        }
    }
}

impl Default for AssistantToolsFileSearch {
    fn default() -> Self {
        Self {
            r#type: "file_search".into(),
        }
    }
}

impl Default for AssistantToolsFunction {
    fn default() -> Self {
        Self {
            r#type: "function".into(),
            function: Default::default(),
        }
    }
}

impl From<FunctionObject> for AssistantToolsFunction {
    fn from(value: FunctionObject) -> Self {
        Self {
            r#type: "function".into(),
            function: value,
        }
    }
}

impl From<FunctionObject> for AssistantTools {
    fn from(value: FunctionObject) -> Self {
        Self::Function(value.into())
    }
}
