#[cfg(feature = "batch")]
use crate::batches::Batches;
#[cfg(feature = "chat-completion")]
use crate::chat::Chat;
#[cfg(feature = "chatkit")]
use crate::chatkit::{Chatkit, ChatkitSessions, ChatkitThreads};
#[cfg(feature = "completions")]
use crate::completion::Completions;
#[cfg(feature = "embedding")]
use crate::embedding::Embeddings;
#[cfg(feature = "file")]
use crate::file::Files;
#[cfg(feature = "finetuning")]
use crate::fine_tuning::FineTuning;
#[cfg(feature = "image")]
use crate::image::Images;
#[cfg(feature = "model")]
use crate::model::Models;
#[cfg(feature = "moderation")]
use crate::moderation::Moderations;
#[cfg(feature = "upload")]
use crate::uploads::Uploads;
#[cfg(feature = "video")]
use crate::video::Videos;
#[cfg(feature = "realtime")]
use crate::Realtime;
#[cfg(feature = "administration")]
use crate::{
    admin::AdminAPIKeys, admin::AuditLogs, admin::Certificates, admin::GroupRoles,
    admin::GroupUsers, admin::Groups, admin::Invites, admin::ProjectAPIKeys,
    admin::ProjectCertificates, admin::ProjectGroupRoles, admin::ProjectGroups,
    admin::ProjectRateLimits, admin::ProjectRoles, admin::ProjectServiceAccounts,
    admin::ProjectUserRoles, admin::ProjectUsers, admin::Projects, admin::Roles, admin::Usage,
    admin::UserRoles, admin::Users,
};
#[cfg(feature = "assistant")]
use crate::{
    assistants::Assistants, assistants::Messages, assistants::Runs, assistants::Steps,
    assistants::Threads,
};
#[cfg(feature = "audio")]
use crate::{audio::Audio, audio::Speech, audio::Transcriptions, audio::Translations};
#[cfg(feature = "container")]
use crate::{containers::ContainerFiles, containers::Containers};
#[cfg(feature = "evals")]
use crate::{evals::EvalRunOutputItems, evals::EvalRuns, evals::Evals};
#[cfg(feature = "responses")]
use crate::{responses::ConversationItems, responses::Conversations, responses::Responses};
#[cfg(feature = "vectorstore")]
use crate::{
    vectorstores::VectorStoreFileBatches, vectorstores::VectorStoreFiles,
    vectorstores::VectorStores,
};

// request builder impls macro

/// Macro to implement `RequestOptionsBuilder` for wrapper types containing `RequestOptions`
#[cfg(feature = "_api")]
macro_rules! impl_request_options_builder {
    ($type:ident) => {
        impl<'c, C: crate::config::Config> crate::traits::RequestOptionsBuilder for $type<'c, C> {
            fn options_mut(&mut self) -> &mut crate::RequestOptions {
                &mut self.request_options
            }

            fn options(&self) -> &crate::RequestOptions {
                &self.request_options
            }
        }
    };
}

#[cfg(feature = "administration")]
impl_request_options_builder!(AdminAPIKeys);
#[cfg(feature = "assistant")]
impl_request_options_builder!(Assistants);
#[cfg(feature = "audio")]
impl_request_options_builder!(Audio);
#[cfg(feature = "administration")]
impl_request_options_builder!(AuditLogs);
#[cfg(feature = "batch")]
impl_request_options_builder!(Batches);
#[cfg(feature = "administration")]
impl_request_options_builder!(Certificates);
#[cfg(feature = "chat-completion")]
impl_request_options_builder!(Chat);
#[cfg(feature = "chatkit")]
impl_request_options_builder!(Chatkit);
#[cfg(feature = "chatkit")]
impl_request_options_builder!(ChatkitSessions);
#[cfg(feature = "chatkit")]
impl_request_options_builder!(ChatkitThreads);
#[cfg(feature = "completions")]
impl_request_options_builder!(Completions);
#[cfg(feature = "container")]
impl_request_options_builder!(ContainerFiles);
#[cfg(feature = "container")]
impl_request_options_builder!(Containers);
#[cfg(feature = "responses")]
impl_request_options_builder!(ConversationItems);
#[cfg(feature = "responses")]
impl_request_options_builder!(Conversations);
#[cfg(feature = "embedding")]
impl_request_options_builder!(Embeddings);
#[cfg(feature = "evals")]
impl_request_options_builder!(Evals);
#[cfg(feature = "evals")]
impl_request_options_builder!(EvalRunOutputItems);
#[cfg(feature = "evals")]
impl_request_options_builder!(EvalRuns);
#[cfg(feature = "file")]
impl_request_options_builder!(Files);
#[cfg(feature = "finetuning")]
impl_request_options_builder!(FineTuning);
#[cfg(feature = "administration")]
impl_request_options_builder!(GroupRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(GroupUsers);
#[cfg(feature = "administration")]
impl_request_options_builder!(Groups);
#[cfg(feature = "image")]
impl_request_options_builder!(Images);
#[cfg(feature = "administration")]
impl_request_options_builder!(Invites);
#[cfg(feature = "assistant")]
impl_request_options_builder!(Messages);
#[cfg(feature = "model")]
impl_request_options_builder!(Models);
#[cfg(feature = "moderation")]
impl_request_options_builder!(Moderations);
#[cfg(feature = "administration")]
impl_request_options_builder!(Projects);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectGroupRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectGroups);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectUserRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectUsers);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectServiceAccounts);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectAPIKeys);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectRateLimits);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectCertificates);
#[cfg(feature = "administration")]
impl_request_options_builder!(Roles);
#[cfg(feature = "realtime")]
impl_request_options_builder!(Realtime);
#[cfg(feature = "responses")]
impl_request_options_builder!(Responses);
#[cfg(feature = "assistant")]
impl_request_options_builder!(Runs);
#[cfg(feature = "audio")]
impl_request_options_builder!(Speech);
#[cfg(feature = "assistant")]
impl_request_options_builder!(Steps);
#[cfg(feature = "assistant")]
impl_request_options_builder!(Threads);
#[cfg(feature = "audio")]
impl_request_options_builder!(Transcriptions);
#[cfg(feature = "audio")]
impl_request_options_builder!(Translations);
#[cfg(feature = "upload")]
impl_request_options_builder!(Uploads);
#[cfg(feature = "administration")]
impl_request_options_builder!(Usage);
#[cfg(feature = "administration")]
impl_request_options_builder!(UserRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(Users);
#[cfg(feature = "vectorstore")]
impl_request_options_builder!(VectorStoreFileBatches);
#[cfg(feature = "vectorstore")]
impl_request_options_builder!(VectorStoreFiles);
#[cfg(feature = "vectorstore")]
impl_request_options_builder!(VectorStores);
#[cfg(feature = "video")]
impl_request_options_builder!(Videos);
