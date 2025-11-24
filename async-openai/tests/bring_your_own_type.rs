#![allow(dead_code)]
//! The purpose of this test to make sure that all _byot methods compiles with custom types.
use std::pin::Pin;

use async_openai::{error::OpenAIError, traits::RequestOptionsBuilder, Client};
use futures::Stream;
use serde_json::{json, Value};

impl async_openai::traits::AsyncTryFrom<MyJson> for reqwest::multipart::Form {
    type Error = OpenAIError;
    async fn try_from(_value: MyJson) -> Result<Self, Self::Error> {
        Ok(reqwest::multipart::Form::new())
    }
}

#[derive(Clone)]
pub struct MyJson(Value);

type MyStreamingType = Pin<Box<dyn Stream<Item = Result<Value, OpenAIError>> + Send>>;

#[tokio::test]
async fn test_byot_files() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.files().create_byot(MyJson(json!({}))).await;
    let _r: Result<Value, OpenAIError> = client
        .files()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client.files().retrieve_byot("file_id").await;
    let _r: Result<Value, OpenAIError> = client.files().delete_byot("file_id").await;
}

#[tokio::test]
async fn test_byot_assistants() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.assistants().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.assistants().retrieve_byot("aid").await;
    let _r: Result<Value, OpenAIError> = client.assistants().update_byot("aid", json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .assistants()
        .query(&[("limit", 2)])
        .unwrap()
        .list_byot()
        .await;
}

#[tokio::test]
async fn test_byot_models() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.models().list_byot().await;
    let _r: Result<Value, OpenAIError> = client.models().retrieve_byot("").await;
    let _r: Result<Value, OpenAIError> = client.models().delete_byot(String::new()).await;
}

#[tokio::test]
async fn test_byot_moderations() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.moderations().create_byot(json!({})).await;
}

#[tokio::test]
async fn test_byot_images() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.images().generate_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.images().edit_byot(MyJson(json!({}))).await;
    let _r: Result<Value, OpenAIError> = client
        .images()
        .create_variation_byot(MyJson(json!({})))
        .await;
}

#[tokio::test]
async fn test_byot_chat() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.chat().create_byot(json!({})).await;
    let _r: Result<MyStreamingType, OpenAIError> =
        client.chat().create_stream_byot(json!({})).await;
}

#[tokio::test]
async fn test_byot_completions() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.completions().create_byot(json!({})).await;
    let _r: Result<MyStreamingType, OpenAIError> =
        client.completions().create_stream_byot(json!({})).await;
}

#[tokio::test]
async fn test_byot_audio() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .audio()
        .transcription()
        .create_byot(MyJson(json!({})))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .audio()
        .transcription()
        .create_verbose_json_byot(MyJson(json!({})))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .audio()
        .translation()
        .create_byot(MyJson(json!({})))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .audio()
        .translation()
        .create_verbose_json_byot(MyJson(json!({})))
        .await;
}

#[tokio::test]
async fn test_byot_embeddings() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.embeddings().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.embeddings().create_base64_byot(json!({})).await;
}

#[tokio::test]
async fn test_byot_fine_tunning() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.fine_tuning().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .fine_tuning()
        .query(&[("limit", "2")])
        .unwrap()
        .list_paginated_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .fine_tuning()
        .retrieve_byot("fine_tunning_job_id")
        .await;
    let _r: Result<Value, OpenAIError> =
        client.fine_tuning().cancel_byot("fine_tuning_job_id").await;
    let _r: Result<Value, OpenAIError> = client
        .fine_tuning()
        .query(&[("limit", "2")])
        .unwrap()
        .list_events_byot("fine_tuning_job_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .fine_tuning()
        .query(&[("limit", "2")])
        .unwrap()
        .list_checkpoints_byot("fine_tuning_job_id")
        .await;
}

#[derive(Clone, serde::Deserialize)]
pub struct MyThreadJson(Value);

impl TryFrom<eventsource_stream::Event> for MyThreadJson {
    type Error = OpenAIError;
    fn try_from(_value: eventsource_stream::Event) -> Result<Self, Self::Error> {
        Ok(MyThreadJson(json!({})))
    }
}

type MyThreadStreamingType = Pin<Box<dyn Stream<Item = Result<MyThreadJson, OpenAIError>> + Send>>;

#[tokio::test]
async fn test_byot_threads() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.threads().create_and_run_byot(json!({})).await;
    let _r: Result<MyThreadStreamingType, OpenAIError> =
        client.threads().create_and_run_stream_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.threads().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.threads().retrieve_byot("thread_id").await;
    let _r: Result<Value, OpenAIError> = client.threads().update_byot("thread_id", json!({})).await;
    let _r: Result<Value, OpenAIError> = client.threads().delete_byot("thread_id").await;
}

#[tokio::test]
async fn test_byot_messages() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .threads()
        .messages("thread_id")
        .create_byot(json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .messages("thread_id")
        .retrieve_byot("message_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .messages("thread_id")
        .update_byot("message_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .messages("thread_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .messages("thread_id")
        .delete_byot("message_id")
        .await;
}

#[tokio::test]
async fn test_byot_runs() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .create_byot(json!({}))
        .await;
    let _r: Result<MyThreadStreamingType, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .create_stream_byot(json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .retrieve_byot("run_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .update_byot("run_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .submit_tool_outputs_byot("run_id", json!({}))
        .await;
    let _r: Result<MyThreadStreamingType, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .submit_tool_outputs_stream_byot("run_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .cancel_byot("run_id")
        .await;
}

#[tokio::test]
async fn test_byot_run_steps() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .steps("run_id")
        .retrieve_byot("step_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .threads()
        .runs("thread_id")
        .steps("run_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
}

#[tokio::test]
async fn test_byot_vector_store_files() {
    let client = Client::new();
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .files("vector_store_id")
        .create_byot(json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .files("vector_store_id")
        .retrieve_byot("file_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .files("vector_store_id")
        .delete_byot("file_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .files("vector_store_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
}

#[tokio::test]
async fn test_byot_vector_store_file_batches() {
    let client = Client::new();
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .file_batches("vector_store_id")
        .create_byot(json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .file_batches("vector_store_id")
        .retrieve_byot("file_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .file_batches("vector_store_id")
        .cancel_byot("file_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .file_batches("vector_store_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_files_byot("batch_id")
        .await;
}

#[tokio::test]
async fn test_byot_batches() {
    let client = Client::new();
    let _r: Result<Value, OpenAIError> = client.batches().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .batches()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client.batches().retrieve_byot("batch_id").await;
    let _r: Result<Value, OpenAIError> = client.batches().cancel_byot("batch_id").await;
}

#[tokio::test]
async fn test_byot_audit_logs() {
    let client = Client::new();
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .audit_logs()
        .query(&[("limit", "2")])
        .unwrap()
        .get_byot()
        .await;
}

#[tokio::test]
async fn test_byot_invites() {
    let client = Client::new();
    let _r: Result<Value, OpenAIError> = client.admin().invites().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.admin().invites().retrieve_byot("invite_id").await;
    let _r: Result<Value, OpenAIError> = client.admin().invites().delete_byot("invite_id").await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .invites()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
}

#[tokio::test]
async fn test_byot_projects() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client.admin().projects().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> =
        client.admin().projects().retrieve_byot("project_id").await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .modify_byot("project_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client.admin().projects().archive_byot("project_id").await;
}

#[tokio::test]
async fn test_byot_project_api_keys() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .api_keys("project_id")
        .query(&[("query", "2")])
        .unwrap()
        .list_byot()
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .api_keys("project_id")
        .retrieve_byot("api_key")
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .api_keys("project_id")
        .delete_byot("api_key")
        .await;
}

#[tokio::test]
async fn test_byot_project_service_accounts() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .service_accounts("project_id")
        .create_byot(json!({}))
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .service_accounts("project_id")
        .delete_byot("service_account_id")
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .service_accounts("project_id")
        .retrieve_byot("service_account_id")
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .service_accounts("project_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
}

#[tokio::test]
async fn test_byot_project_users() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .users("project_id")
        .create_byot(json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .users("project_id")
        .delete_byot("user_id")
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .users("project_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .users("project_id")
        .retrieve_byot("user_id")
        .await;
}

#[tokio::test]
async fn test_byot_uploads() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.uploads().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .uploads()
        .add_part_byot("upload_id", MyJson(json!({})))
        .await;
    let _r: Result<Value, OpenAIError> =
        client.uploads().complete_byot("upload_id", json!({})).await;
    let _r: Result<Value, OpenAIError> = client.uploads().cancel_byot("upload_id").await;
}

#[tokio::test]
async fn test_byot_users() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .users()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .users()
        .modify_byot("user_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client.admin().users().retrieve_byot("user_id").await;
    let _r: Result<Value, OpenAIError> = client.admin().users().delete_byot("user_id").await;
}

#[tokio::test]
async fn test_byot_vector_stores() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.vector_stores().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .retrieve_byot("vector_store_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> =
        client.vector_stores().delete_byot("vector_store_id").await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .update_byot("vector_store_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .vector_stores()
        .search_byot("vector_store_id", json!({}))
        .await;
}

#[tokio::test]
async fn test_byot_speech() {
    let client = Client::new();

    let _r: Result<MyStreamingType, OpenAIError> =
        client.audio().speech().create_stream_byot(json!({})).await;
}

#[tokio::test]
async fn test_byot_responses() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.responses().create_byot(json!({})).await;
    let _r: Result<MyStreamingType, OpenAIError> =
        client.responses().create_stream_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .responses()
        .query(&[("limit", "2")])
        .unwrap()
        .retrieve_byot("response_id")
        .await;
    let _r: Result<Value, OpenAIError> = client.responses().delete_byot("response_id").await;
    let _r: Result<Value, OpenAIError> = client.responses().cancel_byot("response_id").await;
    let _r: Result<Value, OpenAIError> = client
        .responses()
        .query(&[("limit", "2")])
        .unwrap()
        .list_input_items_byot("response_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .responses()
        .get_input_token_counts_byot(json!({}))
        .await;
}

#[tokio::test]
async fn test_byot_conversations() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.conversations().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .conversations()
        .retrieve_byot("conversation_id")
        .await;
    let _r: Result<Value, OpenAIError> =
        client.conversations().delete_byot("conversation_id").await;
    let _r: Result<Value, OpenAIError> = client
        .conversations()
        .update_byot("conversation_id", json!({}))
        .await;
}

#[tokio::test]
async fn test_byot_conversation_items() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .conversations()
        .items("conversation_id")
        .create_byot(json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .conversations()
        .items("conversation_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .conversations()
        .items("conversation_id")
        .retrieve_byot("item_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .conversations()
        .items("conversation_id")
        .delete_byot("item_id")
        .await;
}

#[tokio::test]
async fn test_byot_usage() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .audio_speeches_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .audio_transcriptions_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .code_interpreter_sessions_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .completions_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .embeddings_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .images_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .moderations_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .vector_stores_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .usage()
        .query(&[("limit", "2")])
        .unwrap()
        .costs_byot()
        .await;
}

#[tokio::test]
async fn test_byot_chatkit() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.chatkit().sessions().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> =
        client.chatkit().sessions().cancel_byot("session_id").await;
    let _r: Result<Value, OpenAIError> = client
        .chatkit()
        .threads()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> =
        client.chatkit().threads().retrieve_byot("thread_id").await;
    let _r: Result<Value, OpenAIError> = client.chatkit().threads().delete_byot("thread_id").await;
    let _r: Result<Value, OpenAIError> = client
        .chatkit()
        .threads()
        .query(&[("limit", "2")])
        .unwrap()
        .list_items_byot("thread_id")
        .await;
}

#[tokio::test]
async fn test_byot_containers() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.containers().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client
        .containers()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client.containers().retrieve_byot("container_id").await;
    let _r: Result<Value, OpenAIError> = client.containers().delete_byot("container_id").await;
}

#[tokio::test]
async fn test_byot_container_files() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .containers()
        .files("container_id")
        .create_byot(MyJson(json!({})))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .containers()
        .files("container_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .containers()
        .files("container_id")
        .retrieve_byot("file_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .containers()
        .files("container_id")
        .delete_byot("file_id")
        .await;
}

#[tokio::test]
async fn test_byot_admin_api_keys() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .api_keys()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client.admin().api_keys().retrieve_byot("key_id").await;
    let _r: Result<Value, OpenAIError> = client.admin().api_keys().delete_byot("key_id").await;
}

#[tokio::test]
async fn test_byot_certificates() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .certificates()
        .query(&[("limit", "2")])
        .unwrap()
        .list_organization_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .certificates()
        .retrieve_byot("certificate_id")
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .certificates()
        .modify_byot("certificate_id", json!({}))
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .certificates()
        .delete_byot("certificate_id")
        .await;
}

#[tokio::test]
async fn test_byot_project_rate_limits() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .rate_limits("project_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .admin()
        .projects()
        .rate_limits("project_id")
        .update_byot("rate_limit_id", json!({}))
        .await;
}

#[tokio::test]
async fn test_byot_evals() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .evals()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client.evals().create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> = client.evals().retrieve_byot("eval_id").await;
    let _r: Result<Value, OpenAIError> = client.evals().update_byot("eval_id", json!({})).await;
    let _r: Result<Value, OpenAIError> = client.evals().delete_byot("eval_id").await;
}

#[tokio::test]
async fn test_byot_eval_runs() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .evals()
        .runs("eval_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> =
        client.evals().runs("eval_id").create_byot(json!({})).await;
    let _r: Result<Value, OpenAIError> =
        client.evals().runs("eval_id").retrieve_byot("run_id").await;
    let _r: Result<Value, OpenAIError> = client.evals().runs("eval_id").cancel_byot("run_id").await;
    let _r: Result<Value, OpenAIError> = client.evals().runs("eval_id").delete_byot("run_id").await;
}

#[tokio::test]
async fn test_byot_eval_run_output_items() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client
        .evals()
        .runs("eval_id")
        .output_items("run_id")
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
    let _r: Result<Value, OpenAIError> = client
        .evals()
        .runs("eval_id")
        .output_items("run_id")
        .retrieve_byot("output_item_id")
        .await;
}

#[tokio::test]
async fn test_byot_videos() {
    let client = Client::new();

    let _r: Result<Value, OpenAIError> = client.videos().create_byot(MyJson(json!({}))).await;
    let _r: Result<Value, OpenAIError> = client.videos().remix_byot("video_id", json!({})).await;
    let _r: Result<Value, OpenAIError> = client.videos().retrieve_byot("video_id").await;
    let _r: Result<Value, OpenAIError> = client.videos().delete_byot("video_id").await;
    let _r: Result<Value, OpenAIError> = client
        .videos()
        .query(&[("limit", "2")])
        .unwrap()
        .list_byot()
        .await;
}
