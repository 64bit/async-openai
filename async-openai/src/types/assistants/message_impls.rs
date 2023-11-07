use super::{
    FilePath, ImageFile, MessageContent, MessageContentImageFileObject,
    MessageContentTextAnnotations, MessageContentTextAnnotationsFileCitationObject,
    MessageContentTextAnnotationsFilePathObject, MessageContentTextObject, TextData,
};

impl From<MessageContentTextObject> for MessageContent {
    fn from(value: MessageContentTextObject) -> Self {
        Self::Text(value)
    }
}

impl From<MessageContentImageFileObject> for MessageContent {
    fn from(value: MessageContentImageFileObject) -> Self {
        Self::ImageFile(value)
    }
}

impl From<ImageFile> for MessageContentImageFileObject {
    fn from(value: ImageFile) -> Self {
        Self {
            r#type: "image_file".into(),
            image_file: value,
        }
    }
}

impl From<TextData> for MessageContentTextObject {
    fn from(value: TextData) -> Self {
        Self {
            r#type: "text".into(),
            text: value,
        }
    }
}

macro_rules! from_for_file_id {
    ($from_typ:ty, $to_typ:ty) => {
        impl From<$from_typ> for $to_typ {
            fn from(value: $from_typ) -> Self {
                Self {
                    file_id: value.into(),
                }
            }
        }
    };
}

from_for_file_id!(&str, ImageFile);
from_for_file_id!(String, ImageFile);

from_for_file_id!(&str, FilePath);
from_for_file_id!(String, FilePath);

impl From<MessageContentTextAnnotationsFileCitationObject> for MessageContentTextAnnotations {
    fn from(value: MessageContentTextAnnotationsFileCitationObject) -> Self {
        Self::FileCitation(value)
    }
}

impl From<MessageContentTextAnnotationsFilePathObject> for MessageContentTextAnnotations {
    fn from(value: MessageContentTextAnnotationsFilePathObject) -> Self {
        Self::FilePath(value)
    }
}
