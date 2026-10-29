use crate::{
    config::Config,
    error::OpenAIError,
    types::vaults::{
        CreateVaultCredentialParams, CreateVaultParams, DeletedVaultCredentialResource,
        DeletedVaultResource, RotateVaultCredentialParams, UpdateVaultParams,
        VaultCredentialListResource, VaultCredentialResource, VaultListResource, VaultResource,
    },
    Client, RequestOptions,
};

/// Manage credential vaults used by agent tools.
pub struct Vaults<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Vaults<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    pub fn credentials(&self, vault_id: &str) -> VaultCredentials<'_, C> {
        VaultCredentials::new(self.client, vault_id)
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<VaultListResource, OpenAIError> {
        self.client.get("/vaults", &self.request_options).await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: CreateVaultParams) -> Result<VaultResource, OpenAIError> {
        self.client
            .post("/vaults", request, &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, vault_id: &str) -> Result<VaultResource, OpenAIError> {
        self.client
            .get(&format!("/vaults/{vault_id}"), &self.request_options)
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        vault_id: &str,
        request: UpdateVaultParams,
    ) -> Result<VaultResource, OpenAIError> {
        self.client
            .post(
                &format!("/vaults/{vault_id}"),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, vault_id: &str) -> Result<DeletedVaultResource, OpenAIError> {
        self.client
            .delete(&format!("/vaults/{vault_id}"), &self.request_options)
            .await
    }
}

/// Manage credentials in a vault.
pub struct VaultCredentials<'c, C: Config> {
    client: &'c Client<C>,
    vault_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> VaultCredentials<'c, C> {
    pub fn new(client: &'c Client<C>, vault_id: &str) -> Self {
        Self {
            client,
            vault_id: vault_id.to_owned(),
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<VaultCredentialListResource, OpenAIError> {
        self.client
            .get(
                &format!("/vaults/{}/credentials", self.vault_id),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(
        &self,
        request: CreateVaultCredentialParams,
    ) -> Result<VaultCredentialResource, OpenAIError> {
        self.client
            .post(
                &format!("/vaults/{}/credentials", self.vault_id),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(
        &self,
        credential_id: &str,
    ) -> Result<VaultCredentialResource, OpenAIError> {
        self.client
            .get(
                &format!("/vaults/{}/credentials/{credential_id}", self.vault_id),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, T1 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn rotate(
        &self,
        credential_id: &str,
        request: RotateVaultCredentialParams,
    ) -> Result<VaultCredentialResource, OpenAIError> {
        self.client
            .post(
                &format!("/vaults/{}/credentials/{credential_id}", self.vault_id),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(
        &self,
        credential_id: &str,
    ) -> Result<DeletedVaultCredentialResource, OpenAIError> {
        self.client
            .delete(
                &format!("/vaults/{}/credentials/{credential_id}", self.vault_id),
                &self.request_options,
            )
            .await
    }
}
