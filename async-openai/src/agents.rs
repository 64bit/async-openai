use bytes::Bytes;

use crate::{config::Config, error::OpenAIError, types::agents::*, Client, RequestOptions};

/// Create and manage persistent agents.
pub struct Agents<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Agents<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    pub fn environment_templates(&self) -> AgentEnvironmentTemplates<'_, C> {
        AgentEnvironmentTemplates::new(self.client)
    }

    pub fn environments(&self) -> AgentEnvironments<'_, C> {
        AgentEnvironments::new(self.client)
    }

    pub fn sessions(&self) -> AgentSessions<'_, C> {
        AgentSessions::new(self.client)
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<AgentListResource, OpenAIError> {
        self.client.get("/agents", &self.request_options).await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: CreateAgentParams) -> Result<AgentResource, OpenAIError> {
        self.client
            .post("/agents", request, &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, agent_id: &str) -> Result<AgentResource, OpenAIError> {
        self.client
            .get(&format!("/agents/{agent_id}"), &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        agent_id: &str,
        request: UpdateAgentParams,
    ) -> Result<AgentResource, OpenAIError> {
        self.client
            .post(
                &format!("/agents/{agent_id}"),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, agent_id: &str) -> Result<DeletedAgentResource, OpenAIError> {
        self.client
            .delete(&format!("/agents/{agent_id}"), &self.request_options)
            .await
    }
}

pub struct AgentEnvironmentTemplates<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> AgentEnvironmentTemplates<'c, C> {
    fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<EnvironmentTemplateListResource, OpenAIError> {
        self.client
            .get("/agents/environments/templates", &self.request_options)
            .await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateEnvironmentTemplateParams,
    ) -> Result<EnvironmentTemplateResource, OpenAIError> {
        self.client
            .post(
                "/agents/environments/templates",
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, id: &str) -> Result<EnvironmentTemplateResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/environments/templates/{id}"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        id: &str,
        request: UpdateEnvironmentTemplateParams,
    ) -> Result<EnvironmentTemplateResource, OpenAIError> {
        self.client
            .post(
                &format!("/agents/environments/templates/{id}"),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        id: &str,
    ) -> Result<DeletedEnvironmentTemplateResource, OpenAIError> {
        self.client
            .delete(
                &format!("/agents/environments/templates/{id}"),
                &self.request_options,
            )
            .await
    }
}

pub struct AgentEnvironments<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> AgentEnvironments<'c, C> {
    fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, id: &str) -> Result<PublicEnvironmentResource, OpenAIError> {
        self.client
            .get(&format!("/agents/environments/{id}"), &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn list_files(&self, id: &str) -> Result<EnvironmentFileListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/environments/{id}/files"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create_file(
        &self,
        id: &str,
        request: HostedEnvironmentFileParam,
    ) -> Result<EnvironmentFileResource, OpenAIError> {
        self.client
            .post(
                &format!("/agents/environments/{id}/files"),
                request,
                &self.request_options,
            )
            .await
    }
}

pub struct AgentSessions<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> AgentSessions<'c, C> {
    fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<SessionListResource, OpenAIError> {
        self.client
            .get("/agents/sessions", &self.request_options)
            .await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateAgentSessionParams,
    ) -> Result<SessionResource, OpenAIError> {
        self.client
            .post("/agents/sessions", request, &self.request_options)
            .await
    }

    pub async fn create_stream(
        &self,
        request: CreateAgentSessionParams,
    ) -> Result<SessionEventStream, OpenAIError> {
        self.client
            .post_stream("/agents/sessions", request, &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, id: &str) -> Result<SessionResource, OpenAIError> {
        self.client
            .get(&format!("/agents/sessions/{id}"), &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        id: &str,
        request: UpdateAgentSessionParams,
    ) -> Result<SessionResource, OpenAIError> {
        self.client
            .post(
                &format!("/agents/sessions/{id}"),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, id: &str) -> Result<DeletedSessionResource, OpenAIError> {
        self.client
            .delete(&format!("/agents/sessions/{id}"), &self.request_options)
            .await
    }

    pub async fn events(&self, id: &str) -> Result<SessionEventStream, OpenAIError> {
        self.client
            .get_stream(
                &format!("/agents/sessions/{id}/events"),
                &self.request_options,
            )
            .await
    }

    pub async fn create_events(
        &self,
        id: &str,
        request: CreateSessionEventsParams,
    ) -> Result<(), OpenAIError> {
        self.client
            .post_raw(
                &format!("/agents/sessions/{id}/events"),
                request,
                &self.request_options,
            )
            .await
            .map(|_| ())
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn items(&self, id: &str) -> Result<SessionItemListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/items"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn traces(&self, id: &str) -> Result<SessionTraceListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/traces"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn turns(&self, id: &str) -> Result<SessionTurnListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/turns"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve_turn(
        &self,
        id: &str,
        turn_id: &str,
    ) -> Result<TurnResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/turns/{turn_id}"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn turn_items(
        &self,
        id: &str,
        turn_id: &str,
    ) -> Result<SessionItemListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/turns/{turn_id}/items"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn artifacts(&self, id: &str) -> Result<SessionArtifactListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/artifacts"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve_artifact(
        &self,
        id: &str,
        artifact_id: &str,
    ) -> Result<SessionArtifactResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/artifacts/{artifact_id}"),
                &self.request_options,
            )
            .await
    }

    pub async fn artifact_content(
        &self,
        id: &str,
        artifact_id: &str,
    ) -> Result<Bytes, OpenAIError> {
        self.client
            .get_raw(
                &format!("/agents/sessions/{id}/artifacts/{artifact_id}/content"),
                &self.request_options,
            )
            .await
            .map(|(bytes, _)| bytes)
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete_artifact(
        &self,
        id: &str,
        artifact_id: &str,
    ) -> Result<DeletedSessionArtifactResource, OpenAIError> {
        self.client
            .delete(
                &format!("/agents/sessions/{id}/artifacts/{artifact_id}"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn subagents(&self, id: &str) -> Result<SubagentListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/subagents"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve_subagent(
        &self,
        id: &str,
        subagent_id: &str,
    ) -> Result<SubagentResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/subagents/{subagent_id}"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn subagent_items(
        &self,
        id: &str,
        subagent_id: &str,
    ) -> Result<SessionItemListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/subagents/{subagent_id}/items"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn subagent_turns(
        &self,
        id: &str,
        subagent_id: &str,
    ) -> Result<SessionTurnListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/subagents/{subagent_id}/turns"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, T2 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve_subagent_turn(
        &self,
        id: &str,
        subagent_id: &str,
        turn_id: &str,
    ) -> Result<TurnResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/subagents/{subagent_id}/turns/{turn_id}"),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = std::fmt::Display, T2 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn subagent_turn_items(
        &self,
        id: &str,
        subagent_id: &str,
        turn_id: &str,
    ) -> Result<SessionItemListResource, OpenAIError> {
        self.client
            .get(
                &format!("/agents/sessions/{id}/subagents/{subagent_id}/turns/{turn_id}/items"),
                &self.request_options,
            )
            .await
    }
}
