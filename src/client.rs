//! This module defines the `Client` struct for interacting with the OpenAI API.
//! It includes methods for handling various types of requests such as text completion,
//! image generation, file management, and more.
//! The `Client` struct encapsulates the logic for making HTTP requests to the API endpoints.

use crate::{
    assistant::{
        AssistantFileObject, AssistantFileRequest, AssistantObject, AssistantRequest,
        DeletionStatus, ListAssistant, ListAssistantFile,
    },
    audio::{
        AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest,
        AudioTranscriptionResponse, AudioTranslationRequest, AudioTranslationResponse,
    },
    chat_completion::{ChatCompletionRequest, ChatCompletionResponse},
    completion::{CompletionRequest, CompletionResponse},
    edit::{EditRequest, EditResponse},
    embedding::{EmbeddingRequest, EmbeddingResponse},
    error::APIError,
    file::{
        FileDeleteRequest, FileDeleteResponse, FileListResponse,
        FileRetrieveContentRequest, FileRetrieveContentResponse, FileRetrieveRequest,
        FileRetrieveResponse, FileUploadRequest, FileUploadResponse,
    },
    fine_tuning::{
        CancelFineTuningJobRequest, CreateFineTuningJobRequest, FineTuningJobEvent,
        FineTuningJobObject, FineTuningPagination, ListFineTuningJobEventsRequest,
        RetrieveFineTuningJobRequest,
    },
    image::{
        ImageEditRequest, ImageEditResponse, ImageGenerationRequest,
        ImageGenerationResponse, ImageVariationRequest, ImageVariationResponse,
    },
    message::{
        CreateMessageRequest, ListMessage, ListMessageFile, MessageFileObject,
        MessageObject, ModifyMessageRequest,
    },
    moderation::{CreateModerationRequest, CreateModerationResponse},
    run::{
        CreateRunRequest, CreateThreadAndRunRequest, ListRun, ListRunStep,
        ModifyRunRequest, RunObject, RunStepObject,
    },
    thread::{CreateThreadRequest, ModifyThreadRequest, ThreadObject},
};
use async_std::{
    fs::{create_dir_all, File},
    io::WriteExt,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client as ReqwestClient, Response,
};
use std::path::Path;

const API_URL_V1: &str = "https://api.openai.com/v1";

/// Result type alias for client operations.
type ClientResult<T> = Result<T, APIError>;

/// The `Client` struct for interacting with the OpenAI API.
pub struct Client {
    /// API endpoint URL.
    pub endpoint: String,
    /// API key for authentication.
    pub api_key: String,
    /// Reqwest client for making HTTP requests.
    pub client: ReqwestClient,
}

impl Client {
    /// Creates a new `Client` instance from environment variables.
    pub fn from_env() -> ClientResult<Self> {
        let endpoint =
            std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );

        let client = ReqwestClient::builder().default_headers(headers).build()?;

        Ok(Self {
            endpoint,
            api_key,
            client,
        })
    }

    /// Creates a new `Client` instance with the given API key.
    pub fn new(api_key: String) -> ClientResult<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );

        let client = ReqwestClient::builder().default_headers(headers).build()?;

        Ok(Self {
            endpoint: API_URL_V1.to_owned(),
            api_key,
            client,
        })
    }

    /// Constructs a full API path from a given endpoint path.
    fn from_path(p: &str) -> String {
        format!("{}{}", API_URL_V1, p)
    }

    /// Sends a POST request with the given path and parameters.
    pub async fn post<T: serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> ClientResult<Response> {
        let url = Client::from_path(path);
        self.client
            .post(&url)
            .json(params)
            .send()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends a GET request to the given path.
    pub async fn get(&self, path: &str) -> ClientResult<Response> {
        let url = Client::from_path(path);
        self.client
            .get(&url)
            .send()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends a DELETE request to the given path.
    pub async fn delete(&self, path: &str) -> ClientResult<Response> {
        let url = Client::from_path(path);
        self.client
            .delete(&url)
            .send()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends a completion request and returns the response.
    pub async fn completion(
        &self,
        req: CompletionRequest,
    ) -> ClientResult<CompletionResponse> {
        let url = Client::from_path("/completions");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<CompletionResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an edit request and returns the response.
    pub async fn edit(&self, req: EditRequest) -> ClientResult<EditResponse> {
        let url = Client::from_path("/edits");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<EditResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an image generation request and returns the response.
    pub async fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> ClientResult<ImageGenerationResponse> {
        let url = Client::from_path("/images/generations");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<ImageGenerationResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an image edit request and returns the response.
    pub async fn image_edit(
        &self,
        req: ImageEditRequest,
    ) -> ClientResult<ImageEditResponse> {
        let url = Client::from_path("/images/edits");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<ImageEditResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an image variation request and returns the response.
    pub async fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> ClientResult<ImageVariationResponse> {
        let url = Client::from_path("/images/variations");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<ImageVariationResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an embedding request and returns the response.
    pub async fn embedding(
        &self,
        req: EmbeddingRequest,
    ) -> ClientResult<EmbeddingResponse> {
        let url = Client::from_path("/embeddings");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<EmbeddingResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a list of files.
    pub async fn file_list(&self) -> ClientResult<FileListResponse> {
        let url = Client::from_path("/files");
        self.client
            .get(&url)
            .send()
            .await?
            .json::<FileListResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Uploads a file and returns the response.
    pub async fn file_upload(
        &self,
        req: FileUploadRequest,
    ) -> ClientResult<FileUploadResponse> {
        let url = Client::from_path("/files");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<FileUploadResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Deletes a file and returns the response.
    pub async fn file_delete(
        &self,
        req: FileDeleteRequest,
    ) -> ClientResult<FileDeleteResponse> {
        let path = format!("/files/{}", req.file_id);
        let url = Client::from_path(&path);
        self.client
            .delete(&url)
            .send()
            .await?
            .json::<FileDeleteResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a file's metadata and returns the response.
    pub async fn file_retrieve(
        &self,
        req: FileRetrieveRequest,
    ) -> ClientResult<FileRetrieveResponse> {
        let path = format!("/files/{}", req.file_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<FileRetrieveResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves the content of a file and returns the response.
    pub async fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> ClientResult<FileRetrieveContentResponse> {
        let path = format!("/files/{}/content", req.file_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<FileRetrieveContentResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends a chat completion request and returns the response.
    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> ClientResult<ChatCompletionResponse> {
        let url = Client::from_path("/chat/completions");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an audio transcription request and returns the response.
    pub async fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> ClientResult<AudioTranscriptionResponse> {
        let url = Client::from_path("/audio/transcriptions");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<AudioTranscriptionResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an audio translation request and returns the response.
    pub async fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> ClientResult<AudioTranslationResponse> {
        let url = Client::from_path("/audio/translations");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<AudioTranslationResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Sends an audio speech request, saves the response to a file, and returns the response.
    pub async fn audio_speech(
        &self,
        req: AudioSpeechRequest,
    ) -> ClientResult<AudioSpeechResponse> {
        let url = Client::from_path("/audio/speech");
        let response = self.client.post(&url).json(&req).send().await?;

        let bytes = response.bytes().await?;
        let path = Path::new(&req.output);
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        file.write_all(&bytes).await?;

        Ok(AudioSpeechResponse { result: true })
    }

    /// Creates a fine-tuning job and returns the response.
    pub async fn create_fine_tuning_job(
        &self,
        req: CreateFineTuningJobRequest,
    ) -> ClientResult<FineTuningJobObject> {
        let url = Client::from_path("/fine_tuning/jobs");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<FineTuningJobObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists fine-tuning jobs and returns the response.
    pub async fn list_fine_tuning_jobs(
        &self,
    ) -> ClientResult<FineTuningPagination<FineTuningJobObject>> {
        let url = Client::from_path("/fine_tuning/jobs");
        self.client
            .get(&url)
            .send()
            .await?
            .json::<FineTuningPagination<FineTuningJobObject>>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists fine-tuning job events and returns the response.
    pub async fn list_fine_tuning_job_events(
        &self,
        req: ListFineTuningJobEventsRequest,
    ) -> ClientResult<FineTuningPagination<FineTuningJobEvent>> {
        let path = format!("/fine_tuning/jobs/{}/events", req.fine_tuning_job_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<FineTuningPagination<FineTuningJobEvent>>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a fine-tuning job and returns the response.
    pub async fn retrieve_fine_tuning_job(
        &self,
        req: RetrieveFineTuningJobRequest,
    ) -> ClientResult<FineTuningJobObject> {
        let path = format!("/fine_tuning/jobs/{}", req.fine_tuning_job_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<FineTuningJobObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Cancels a fine-tuning job and returns the response.
    pub async fn cancel_fine_tuning_job(
        &self,
        req: CancelFineTuningJobRequest,
    ) -> ClientResult<FineTuningJobObject> {
        let path = format!("/fine_tuning/jobs/{}/cancel", req.fine_tuning_job_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .send()
            .await?
            .json::<FineTuningJobObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates a moderation request and returns the response.
    pub async fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> ClientResult<CreateModerationResponse> {
        let url = Client::from_path("/content-moderation");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<CreateModerationResponse>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates an assistant and returns the response.
    pub async fn create_assistant(
        &self,
        req: AssistantRequest,
    ) -> ClientResult<AssistantObject> {
        let url = Client::from_path("/assistants");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<AssistantObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves an assistant and returns the response.
    pub async fn retrieve_assistant(
        &self,
        assistant_id: String,
    ) -> ClientResult<AssistantObject> {
        let path = format!("/assistants/{}", assistant_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<AssistantObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Modifies an assistant and returns the response.
    pub async fn modify_assistant(
        &self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> ClientResult<AssistantObject> {
        let path = format!("/assistants/{}", assistant_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<AssistantObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Deletes an assistant and returns the response.
    pub async fn delete_assistant(
        &self,
        assistant_id: String,
    ) -> ClientResult<DeletionStatus> {
        let path = format!("/assistants/{}", assistant_id);
        let url = Client::from_path(&path);
        self.client
            .delete(&url)
            .send()
            .await?
            .json::<DeletionStatus>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists assistants and returns the response.
    pub async fn list_assistant(
        &self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> ClientResult<ListAssistant> {
        let base_url = Client::from_path("/assistants");
        let url = Client::query_params(limit, order, after, before, base_url);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ListAssistant>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates an assistant file and returns the response.
    pub async fn create_assistant_file(
        &self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> ClientResult<AssistantFileObject> {
        let path = format!("/assistants/{}/files", assistant_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<AssistantFileObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves an assistant file and returns the response.
    pub async fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> ClientResult<AssistantFileObject> {
        let path = format!("/assistants/{}/files/{}", assistant_id, file_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<AssistantFileObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Deletes an assistant file and returns the response.
    pub async fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> ClientResult<DeletionStatus> {
        let path = format!("/assistants/{}/files/{}", assistant_id, file_id);
        let url = Client::from_path(&path);
        self.client
            .delete(&url)
            .send()
            .await?
            .json::<DeletionStatus>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists assistant files and returns the response.
    pub async fn list_assistant_file(
        &self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> ClientResult<ListAssistantFile> {
        let path = format!("/assistants/{}/files", assistant_id);
        let path = Client::query_params(limit, order, after, before, path);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ListAssistantFile>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates a thread and returns the response.
    pub async fn create_thread(
        &self,
        req: CreateThreadRequest,
    ) -> ClientResult<ThreadObject> {
        let url = Client::from_path("/threads");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<ThreadObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a thread and returns the response.
    pub async fn retrieve_thread(&self, thread_id: String) -> ClientResult<ThreadObject> {
        let path = format!("/threads/{}", thread_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ThreadObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Modifies a thread and returns the response.
    pub async fn modify_thread(
        &self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> ClientResult<ThreadObject> {
        let path = format!("/threads/{}", thread_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<ThreadObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Deletes a thread and returns the response.
    pub async fn delete_thread(&self, thread_id: String) -> ClientResult<DeletionStatus> {
        let path = format!("/threads/{}", thread_id);
        let url = Client::from_path(&path);
        self.client
            .delete(&url)
            .send()
            .await?
            .json::<DeletionStatus>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates a message in a thread and returns the response.
    pub async fn create_message(
        &self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> ClientResult<MessageObject> {
        let path = format!("/threads/{}/messages", thread_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<MessageObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a message in a thread and returns the response.
    pub async fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> ClientResult<MessageObject> {
        let path = format!("/threads/{}/messages/{}", thread_id, message_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<MessageObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Modifies a message in a thread and returns the response.
    pub async fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> ClientResult<MessageObject> {
        let path = format!("/threads/{}/messages/{}", thread_id, message_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<MessageObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists messages in a thread and returns the response.
    pub async fn list_messages(&self, thread_id: String) -> ClientResult<ListMessage> {
        let path = format!("/threads/{}/messages", thread_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ListMessage>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a file associated with a message and returns the response.
    pub async fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> ClientResult<MessageFileObject> {
        let path = format!(
            "/threads/{}/messages/{}/files/{}",
            thread_id, message_id, file_id
        );
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<MessageFileObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists files associated with a message and returns the response.
    pub async fn list_message_file(
        &self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> ClientResult<ListMessageFile> {
        let path = format!("/threads/{}/messages/{}/files", thread_id, message_id);
        let path = Client::query_params(limit, order, after, before, path);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ListMessageFile>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates a run in a thread and returns the response.
    pub async fn create_run(
        &self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> ClientResult<RunObject> {
        let path = format!("/threads/{}/runs", thread_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<RunObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a run in a thread and returns the response.
    pub async fn retrieve_run(
        &self,
        thread_id: String,
        run_id: String,
    ) -> ClientResult<RunObject> {
        let path = format!("/threads/{}/runs/{}", thread_id, run_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<RunObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Modifies a run in a thread and returns the response.
    pub async fn modify_run(
        &self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> ClientResult<RunObject> {
        let path = format!("/threads/{}/runs/{}", thread_id, run_id);
        let url = Client::from_path(&path);
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<RunObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists runs in a thread and returns the response.
    pub async fn list_run(
        &self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> ClientResult<ListRun> {
        let path = format!("/threads/{}/runs", thread_id);
        let path = Client::query_params(limit, order, after, before, path);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ListRun>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Cancels a run in a thread and returns the response.
    pub async fn cancel_run(
        &self,
        thread_id: String,
        run_id: String,
    ) -> ClientResult<RunObject> {
        let path = format!("/threads/{}/runs/{}/cancel", thread_id, run_id);
        let url = Client::from_path(&path);
        let empty_req = ModifyRunRequest::new();
        self.client
            .post(&url)
            .json(&empty_req)
            .send()
            .await?
            .json::<RunObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Creates a thread and a run and returns the response.
    pub async fn create_thread_and_run(
        &self,
        req: CreateThreadAndRunRequest,
    ) -> ClientResult<RunObject> {
        let url = Client::from_path("/threads/runs");
        self.client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json::<RunObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Retrieves a step in a run and returns the response.
    pub async fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> ClientResult<RunStepObject> {
        let path = format!("/threads/{}/runs/{}/steps/{}", thread_id, run_id, step_id);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<RunStepObject>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Lists steps in a run and returns the response.
    pub async fn list_run_step(
        &self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> ClientResult<ListRunStep> {
        let path = format!("/threads/{}/runs/{}/steps", thread_id, run_id);
        let path = Client::query_params(limit, order, after, before, path);
        let url = Client::from_path(&path);
        self.client
            .get(&url)
            .send()
            .await?
            .json::<ListRunStep>()
            .await
            .map_err(APIError::ReqwestError)
    }

    /// Constructs a query parameter string from the given options and appends it to the URL.
    fn query_params(
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
        mut url: String,
    ) -> String {
        let mut params = String::new();
        if let Some(limit) = limit {
            params.push_str(&format!("limit={}&", limit));
        }
        if let Some(order) = order {
            params.push_str(&format!("order={}&", order));
        }
        if let Some(after) = after {
            params.push_str(&format!("after={}&", after));
        }
        if let Some(before) = before {
            params.push_str(&format!("before={}&", before));
        }
        if !params.is_empty() {
            url.push_str(&format!("?{params}"));
        }
        url
    }
}
