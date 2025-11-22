#[cfg(feature = "responses")]
use crate::responses::Responses;
#[cfg(feature = "platform")]
use crate::{
    audio::Audio,
    batches::Batches,
    embedding::Embeddings,
    eval_run_output_items::EvalRunOutputItems,
    eval_runs::EvalRuns,
    evals::Evals,
    file::Files,
    fine_tuning::FineTuning,
    image::Images,
    model::Models,
    moderation::Moderations,
    speech::Speech,
    transcriptions::Transcriptions,
    translations::Translations,
    uploads::Uploads,
    video::Videos,
};
#[cfg(feature = "vector-stores")]
use crate::{
    vector_store_file_batches::VectorStoreFileBatches,
    vector_store_files::VectorStoreFiles,
    vector_stores::VectorStores,
};
#[cfg(feature = "chatkit")]
use crate::chatkit::{Chatkit, ChatkitSessions, ChatkitThreads};
#[cfg(feature = "container")]
use crate::{container_files::ContainerFiles, containers::Containers};
#[cfg(feature = "realtime")]
use crate::Realtime;
#[cfg(feature = "chat-completion")]
use crate::{chat::Chat, conversation_items::ConversationItems, conversations::Conversations};
#[cfg(feature = "assistants")]
use crate::{assistants::Assistants, messages::Messages, runs::Runs, steps::Steps, threads::Threads};
#[cfg(feature = "administration")]
use crate::{
    admin_api_keys::AdminAPIKeys,
    audit_logs::AuditLogs,
    certificates::Certificates,
    group_roles::GroupRoles,
    group_users::GroupUsers,
    groups::Groups,
    invites::Invites,
    project_api_keys::ProjectAPIKeys,
    project_certificates::ProjectCertificates,
    project_group_roles::ProjectGroupRoles,
    project_groups::ProjectGroups,
    project_rate_limits::ProjectRateLimits,
    project_roles::ProjectRoles,
    project_service_accounts::ProjectServiceAccounts,
    project_user_roles::ProjectUserRoles,
    project_users::ProjectUsers,
    projects::Projects,
    roles::Roles,
    usage::Usage,
    user_roles::UserRoles,
    users::Users,
};
#[cfg(feature = "completions")]
use crate::completion::Completions;

// request builder impls macro

/// Macro to implement `RequestOptionsBuilder` for wrapper types containing `RequestOptions`
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

#[cfg(feature = "responses")]
impl_request_options_builder!(Responses);
#[cfg(feature = "platform")]
impl_request_options_builder!(Audio);
#[cfg(feature = "platform")]
impl_request_options_builder!(Batches);
#[cfg(feature = "platform")]
impl_request_options_builder!(Embeddings);
#[cfg(feature = "platform")]
impl_request_options_builder!(Evals);
#[cfg(feature = "platform")]
impl_request_options_builder!(EvalRunOutputItems);
#[cfg(feature = "platform")]
impl_request_options_builder!(EvalRuns);
#[cfg(feature = "platform")]
impl_request_options_builder!(Files);
#[cfg(feature = "platform")]
impl_request_options_builder!(FineTuning);
#[cfg(feature = "platform")]
impl_request_options_builder!(Images);
#[cfg(feature = "platform")]
impl_request_options_builder!(Models);
#[cfg(feature = "platform")]
impl_request_options_builder!(Moderations);
#[cfg(feature = "platform")]
impl_request_options_builder!(Speech);
#[cfg(feature = "platform")]
impl_request_options_builder!(Transcriptions);
#[cfg(feature = "platform")]
impl_request_options_builder!(Translations);
#[cfg(feature = "platform")]
impl_request_options_builder!(Uploads);
#[cfg(feature = "platform")]
impl_request_options_builder!(Videos);
#[cfg(feature = "vector-stores")]
impl_request_options_builder!(VectorStoreFileBatches);
#[cfg(feature = "vector-stores")]
impl_request_options_builder!(VectorStoreFiles);
#[cfg(feature = "vector-stores")]
impl_request_options_builder!(VectorStores);
#[cfg(feature = "chatkit")]
impl_request_options_builder!(Chatkit);
#[cfg(feature = "chatkit")]
impl_request_options_builder!(ChatkitSessions);
#[cfg(feature = "chatkit")]
impl_request_options_builder!(ChatkitThreads);
#[cfg(feature = "container")]
impl_request_options_builder!(ContainerFiles);
#[cfg(feature = "container")]
impl_request_options_builder!(Containers);
#[cfg(feature = "realtime")]
impl_request_options_builder!(Realtime);
#[cfg(feature = "chat-completion")]
impl_request_options_builder!(Chat);
#[cfg(feature = "chat-completion")]
impl_request_options_builder!(ConversationItems);
#[cfg(feature = "chat-completion")]
impl_request_options_builder!(Conversations);
#[cfg(feature = "assistants")]
impl_request_options_builder!(Assistants);
#[cfg(feature = "assistants")]
impl_request_options_builder!(Messages);
#[cfg(feature = "assistants")]
impl_request_options_builder!(Runs);
#[cfg(feature = "assistants")]
impl_request_options_builder!(Steps);
#[cfg(feature = "assistants")]
impl_request_options_builder!(Threads);
#[cfg(feature = "administration")]
impl_request_options_builder!(AdminAPIKeys);
#[cfg(feature = "administration")]
impl_request_options_builder!(AuditLogs);
#[cfg(feature = "administration")]
impl_request_options_builder!(Certificates);
#[cfg(feature = "administration")]
impl_request_options_builder!(GroupRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(GroupUsers);
#[cfg(feature = "administration")]
impl_request_options_builder!(Groups);
#[cfg(feature = "administration")]
impl_request_options_builder!(Invites);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectAPIKeys);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectCertificates);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectGroupRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectGroups);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectRateLimits);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectServiceAccounts);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectUserRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(ProjectUsers);
#[cfg(feature = "administration")]
impl_request_options_builder!(Projects);
#[cfg(feature = "administration")]
impl_request_options_builder!(Roles);
#[cfg(feature = "administration")]
impl_request_options_builder!(Usage);
#[cfg(feature = "administration")]
impl_request_options_builder!(UserRoles);
#[cfg(feature = "administration")]
impl_request_options_builder!(Users);
#[cfg(feature = "completions")]
impl_request_options_builder!(Completions);
