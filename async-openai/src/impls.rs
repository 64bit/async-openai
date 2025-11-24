use crate::{
    admin::AdminAPIKeys,
    admin::AuditLogs,
    admin::Certificates,
    admin::GroupRoles,
    admin::GroupUsers,
    admin::Groups,
    admin::Invites,
    admin::ProjectAPIKeys,
    admin::ProjectCertificates,
    admin::ProjectGroupRoles,
    admin::ProjectGroups,
    admin::ProjectRateLimits,
    admin::ProjectRoles,
    admin::ProjectServiceAccounts,
    admin::ProjectUserRoles,
    admin::ProjectUsers,
    admin::Projects,
    admin::Roles,
    admin::Usage,
    admin::UserRoles,
    admin::Users,
    assistants::Assistants,
    audio::Audio,
    audio::Speech,
    audio::Transcriptions,
    audio::Translations,
    batches::Batches,
    chat::Chat,
    chatkit::{Chatkit, ChatkitSessions, ChatkitThreads},
    completion::Completions,
    container_files::ContainerFiles,
    containers::Containers,
    conversation_items::ConversationItems,
    conversations::Conversations,
    embedding::Embeddings,
    eval_run_output_items::EvalRunOutputItems,
    eval_runs::EvalRuns,
    evals::Evals,
    file::Files,
    fine_tuning::FineTuning,
    image::Images,
    messages::Messages,
    model::Models,
    moderation::Moderations,
    responses::Responses,
    runs::Runs,
    steps::Steps,
    threads::Threads,
    uploads::Uploads,
    vector_store_file_batches::VectorStoreFileBatches,
    vector_store_files::VectorStoreFiles,
    vector_stores::VectorStores,
    video::Videos,
};

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

#[cfg(feature = "realtime")]
use crate::Realtime;

impl_request_options_builder!(AdminAPIKeys);
impl_request_options_builder!(Assistants);
impl_request_options_builder!(Audio);
impl_request_options_builder!(AuditLogs);
impl_request_options_builder!(Batches);
impl_request_options_builder!(Certificates);
impl_request_options_builder!(Chat);
impl_request_options_builder!(Chatkit);
impl_request_options_builder!(ChatkitSessions);
impl_request_options_builder!(ChatkitThreads);
impl_request_options_builder!(Completions);
impl_request_options_builder!(ContainerFiles);
impl_request_options_builder!(Containers);
impl_request_options_builder!(ConversationItems);
impl_request_options_builder!(Conversations);
impl_request_options_builder!(Embeddings);
impl_request_options_builder!(Evals);
impl_request_options_builder!(EvalRunOutputItems);
impl_request_options_builder!(EvalRuns);
impl_request_options_builder!(Files);
impl_request_options_builder!(FineTuning);
impl_request_options_builder!(GroupRoles);
impl_request_options_builder!(GroupUsers);
impl_request_options_builder!(Groups);
impl_request_options_builder!(Images);
impl_request_options_builder!(Invites);
impl_request_options_builder!(Messages);
impl_request_options_builder!(Models);
impl_request_options_builder!(Moderations);
impl_request_options_builder!(Projects);
impl_request_options_builder!(ProjectGroupRoles);
impl_request_options_builder!(ProjectGroups);
impl_request_options_builder!(ProjectRoles);
impl_request_options_builder!(ProjectUserRoles);
impl_request_options_builder!(ProjectUsers);
impl_request_options_builder!(ProjectServiceAccounts);
impl_request_options_builder!(ProjectAPIKeys);
impl_request_options_builder!(ProjectRateLimits);
impl_request_options_builder!(ProjectCertificates);
impl_request_options_builder!(Roles);
#[cfg(feature = "realtime")]
impl_request_options_builder!(Realtime);
impl_request_options_builder!(Responses);
impl_request_options_builder!(Runs);
impl_request_options_builder!(Speech);
impl_request_options_builder!(Steps);
impl_request_options_builder!(Threads);
impl_request_options_builder!(Transcriptions);
impl_request_options_builder!(Translations);
impl_request_options_builder!(Uploads);
impl_request_options_builder!(Usage);
impl_request_options_builder!(UserRoles);
impl_request_options_builder!(Users);
impl_request_options_builder!(VectorStoreFileBatches);
impl_request_options_builder!(VectorStoreFiles);
impl_request_options_builder!(VectorStores);
impl_request_options_builder!(Videos);
