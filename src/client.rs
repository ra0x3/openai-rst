use crate::assistant::{
    AssistantFileObject, AssistantFileRequest, AssistantObject, AssistantRequest, DeletionStatus,
    ListAssistant, ListAssistantFile,
};
use crate::audio::{
    AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest, AudioTranscriptionResponse,
    AudioTranslationRequest, AudioTranslationResponse,
};
use crate::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
use crate::completion::{CompletionRequest, CompletionResponse};
use crate::edit::{EditRequest, EditResponse};
use crate::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::error::APIError;
use crate::file::{
    FileDeleteRequest, FileDeleteResponse, FileListResponse, FileRetrieveContentRequest,
    FileRetrieveContentResponse, FileRetrieveRequest, FileRetrieveResponse, FileUploadRequest,
    FileUploadResponse,
};
use crate::fine_tuning::{
    CancelFineTuningJobRequest, CreateFineTuningJobRequest, FineTuningJobEvent,
    FineTuningJobObject, FineTuningPagination, ListFineTuningJobEventsRequest,
    RetrieveFineTuningJobRequest,
};
use crate::image::{
    ImageEditRequest, ImageEditResponse, ImageGenerationRequest, ImageGenerationResponse,
    ImageVariationRequest, ImageVariationResponse,
};
use crate::message::{
    CreateMessageRequest, ListMessage, ListMessageFile, MessageFileObject, MessageObject,
    ModifyMessageRequest,
};
use crate::moderation::{CreateModerationRequest, CreateModerationResponse};
use crate::run::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRun, ListRunStep, ModifyRunRequest, RunObject,
    RunStepObject,
};
use crate::thread::{CreateThreadRequest, ModifyThreadRequest, ThreadObject};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, RequestBuilder, Response,
};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

const API_URL_V1: &str = "https://api.openai.com/v1";

type APIResult<T> = Result<T, APIError>;

pub struct Client {
    pub endpoint: String,
    pub api_key: String,
    pub organization: Option<String>,
    pub proxy: Option<String>,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self::new_with_endpoint(endpoint, api_key)
    }

    pub fn new_with_endpoint(endpoint: String, api_key: String) -> Self {
        Self {
            endpoint,
            api_key,
            organization: None,
            proxy: None,
        }
    }

    pub fn new_with_organization(api_key: String, organization: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self {
            endpoint: endpoint,
            api_key,
            organization: organization.into(),
            proxy: None,
        }
    }

    pub fn new_with_proxy(api_key: String, proxy: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self {
            endpoint: endpoint,
            api_key,
            organization: None,
            proxy: Some(proxy),
        }
    }

    pub fn build_request(&self, builder: RequestBuilder, is_beta: bool) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );

        if let Some(org) = &self.organization {
            headers.insert("openai-organization", HeaderValue::from_str(org).unwrap());
        }

        if is_beta {
            headers.insert("OpenAI-Beta", HeaderValue::from_static("assistants=v1"));
        }

        client
    }

    pub fn post<T: serde::ser::Serialize>(&self, path: &str, params: &T) -> APIResult<Response> {
        let url = format!("{}{}", self.endpoint, path);
        let client = Client::new();
        let builder = self
            .build_request(client.post(&url), Self::is_beta(path))
            .await;
        let response = request_builder.json(params).send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(APIError::GenericError(format!(
                        "{}: {}",
                        res.status(),
                        res.text().await.unwrap_or_default()
                    )))
                }
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn get(&self, path: &str) -> APIResult<Response> {
        let url = format!("{}{}", self.endpoint, path);
        let client = Client::new();
        let builder = self
            .build_request(client.get(&url), Self::is_beta(path))
            .await;

        let response = builder.send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(APIError::ResponseError(format!(
                        "{}: {}",
                        res.status(),
                        res.text().await.unwrap_or_default()
                    )))
                }
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn delete(&self, path: &str) -> APIResult<Response> {
        let url = format!("{}{}", self.endpoint, path);
        let client = Client::new();
        let builder = self
            .build_request(client.delete(&url), Self::is_beta(path))
            .await;

        let response = builder.send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(APIError::ResponseError(format!(
                        "{}: {}",
                        res.status(),
                        res.text().await.unwrap_or_default()
                    )))
                }
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn completion(&self, req: CompletionRequest) -> APIResult<CompletionResponse> {
        let url = format!("{}{}", self.endpoint, "/completions");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<CompletionResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn edit(&self, req: EditRequest) -> APIResult<EditResponse> {
        let url = format!("{}{}", self.endpoint, "/edits");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<EditResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> APIResult<ImageGenerationResponse> {
        let url = format!("{}{}", self.endpoint, "/images/generations");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<ImageGenerationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn image_edit(&self, req: ImageEditRequest) -> APIResult<ImageEditResponse> {
        let url = format!("{}{}", self.endpoint, "/images/edits");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<ImageEditResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> APIResult<ImageVariationResponse> {
        let url = format!("{}{}", self.endpoint, "/images/variations");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<ImageVariationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn embedding(&self, req: EmbeddingRequest) -> APIResult<EmbeddingResponse> {
        let url = format!("{}{}", self.endpoint, "/embeddings");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<EmbeddingResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn file_list(&self) -> APIResult<FileListResponse> {
        let url = format!("{}{}", self.endpoint, "/files");
        let response = self.client.get(&url).send().await?;

        match response.json::<FileListResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn file_upload(&self, req: FileUploadRequest) -> APIResult<FileUploadResponse> {
        let url = format!("{}{}", self.endpoint, "/files");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<FileUploadResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn file_delete(&self, req: FileDeleteRequest) -> APIResult<FileDeleteResponse> {
        let url = format!("{}{}/{}", self.endpoint, "/files", req.file_id);
        let response = self.client.delete(&url).send().await?;

        match response.json::<FileDeleteResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn file_retrieve(&self, req: FileRetrieveRequest) -> APIResult<FileRetrieveResponse> {
        let url = format!("{}{}/{}", self.endpoint, "/files", req.file_id);
        let response = self.client.get(&url).send().await?;

        match response.json::<FileRetrieveResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> APIResult<FileRetrieveContentResponse> {
        let url = format!("{}{}/{}/content", self.endpoint, "/files", req.file_id);
        let response = self.client.get(&url).send().await?;

        match response.json::<FileRetrieveContentResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> APIResult<ChatCompletionResponse> {
        let url = format!("{}{}", self.api_endpoint, "/chat/completions");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<ChatCompletionResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> APIResult<AudioTranscriptionResponse> {
        let url = format!("{}{}", self.api_endpoint, "/audio/transcriptions");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<AudioTranscriptionResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> APIResult<AudioTranslationResponse> {
        let url = format!("{}{}", self.api_endpoint, "/audio/translations");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<AudioTranslationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn audio_speech(&self, req: AudioSpeechRequest) -> APIResult<AudioSpeechResponse> {
        let url = format!("{}{}", self.api_endpoint, "/audio/speech");
        let response = self.client.post(&url).json(&req).send().await?;

        let bytes = response.bytes().await?;
        let path = Path::new(&req.output);
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        file.write_all(&bytes).await?;

        Ok(AudioSpeechResponse {
            result: true,
            headers: Some(response.headers().clone()),
        })
    }

    pub async fn create_fine_tuning_job(
        &self,
        req: CreateFineTuningJobRequest,
    ) -> APIResult<FineTuningJobObject> {
        let url = format!("{}{}", self.api_endpoint, "/fine_tuning/jobs");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<FineTuningJobObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn list_fine_tuning_jobs(
        &self,
    ) -> APIResult<FineTuningPagination<FineTuningJobObject>> {
        let url = format!("{}{}", self.api_endpoint, "/fine_tuning/jobs");
        let response = self.client.get(&url).send().await?;

        match response
            .json::<FineTuningPagination<FineTuningJobObject>>()
            .await
        {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn list_fine_tuning_job_events(
        &self,
        req: ListFineTuningJobEventsRequest,
    ) -> APIResult<FineTuningPagination<FineTuningJobEvent>> {
        let url = format!(
            "{}{}{}/events",
            self.api_endpoint, "/fine_tuning/jobs/", req.fine_tuning_job_id
        );
        let response = self.client.get(&url).send().await?;

        match response
            .json::<FineTuningPagination<FineTuningJobEvent>>()
            .await
        {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn retrieve_fine_tuning_job(
        &self,
        req: RetrieveFineTuningJobRequest,
    ) -> APIResult<FineTuningJobObject> {
        let url = format!(
            "{}{}{}",
            self.api_endpoint, "/fine_tuning/jobs/", req.fine_tuning_job_id
        );
        let response = self.client.get(&url).send().await?;

        match response.json::<FineTuningJobObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn cancel_fine_tuning_job(
        &self,
        req: CancelFineTuningJobRequest,
    ) -> APIResult<FineTuningJobObject> {
        let url = format!(
            "{}{}{}/cancel",
            self.api_endpoint, "/fine_tuning/jobs/", req.fine_tuning_job_id
        );
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<FineTuningJobObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> APIResult<CreateModerationResponse> {
        let url = format!("{}{}", self.api_endpoint, "/moderations");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<CreateModerationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn create_assistant(&self, req: AssistantRequest) -> APIResult<AssistantObject> {
        let url = format!("{}{}", self.api_endpoint, "/assistants");
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<AssistantObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn retrieve_assistant(&self, assistant_id: String) -> APIResult<AssistantObject> {
        let url = format!("{}{}{}", self.api_endpoint, "/assistants/", assistant_id);
        let response = self.client.get(&url).send().await?;

        match response.json::<AssistantObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn modify_assistant(
        &self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> APIResult<AssistantObject> {
        let url = format!("{}{}{}", self.api_endpoint, "/assistants/", assistant_id);
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<AssistantObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn delete_assistant(&self, assistant_id: String) -> APIResult<DeletionStatus> {
        let url = format!("{}{}{}", self.api_endpoint, "/assistants/", assistant_id);
        let response = self.client.delete(&url).send().await?;

        match response.json::<DeletionStatus>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn list_assistant(
        &self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> APIResult<ListAssistant> {
        let base_url = format!("{}{}", self.api_endpoint, "/assistants");
        let mut url = self.query_params(limit, order, after, before, base_url);
        let response = self.client.get(&url).send().await?;

        match response.json::<ListAssistant>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    pub async fn create_assistant_file(
        &self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> APIResult<AssistantFileObject> {
        let url = format!(
            "{}{}{}/files",
            self.api_endpoint, "/assistants/", assistant_id
        );
        let response = self.client.post(&url).json(&req).send().await?;

        match response.json::<AssistantFileObject>().await {
            Ok(mut r) => {
                r.headers = Some(response.headers().clone());
                Ok(r)
            }
            Err(e) => Err(APIError::JsonError(e)),
        }
    }

    // TODO: FINISH

    pub fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<AssistantFileObject, APIError> {
        let res = self.get(&format!("/assistants/{}/files/{}", assistant_id, file_id))?;
        let r = res.json::<AssistantFileObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<DeletionStatus, APIError> {
        let res = self.delete(&format!("/assistants/{}/files/{}", assistant_id, file_id))?;
        let r = res.json::<DeletionStatus>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn list_assistant_file(
        &self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistantFile, APIError> {
        let mut url = format!("/assistants/{}/files", assistant_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListAssistantFile>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn create_thread(&self, req: CreateThreadRequest) -> Result<ThreadObject, APIError> {
        let res = self.post("/threads", &req)?;
        let r = res.json::<ThreadObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn retrieve_thread(&self, thread_id: String) -> Result<ThreadObject, APIError> {
        let res = self.get(&format!("/threads/{}", thread_id))?;
        let r = res.json::<ThreadObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn modify_thread(
        &self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> Result<ThreadObject, APIError> {
        let res = self.post(&format!("/threads/{}", thread_id), &req)?;
        let r = res.json::<ThreadObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn delete_thread(&self, thread_id: String) -> Result<DeletionStatus, APIError> {
        let res = self.delete(&format!("/threads/{}", thread_id))?;
        let r = res.json::<DeletionStatus>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn create_message(
        &self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> Result<MessageObject, APIError> {
        let res = self.post(&format!("/threads/{}/messages", thread_id), &req)?;
        let r = res.json::<MessageObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<MessageObject, APIError> {
        let res = self.get(&format!("/threads/{}/messages/{}", thread_id, message_id))?;
        let r = res.json::<MessageObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> Result<MessageObject, APIError> {
        let res = self.post(
            &format!("/threads/{}/messages/{}", thread_id, message_id),
            &req,
        )?;
        let r = res.json::<MessageObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn list_messages(&self, thread_id: String) -> Result<ListMessage, APIError> {
        let res = self.get(&format!("/threads/{}/messages", thread_id))?;
        let r = res.json::<ListMessage>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<MessageFileObject, APIError> {
        let res = self.get(&format!(
            "/threads/{}/messages/{}/files/{}",
            thread_id, message_id, file_id
        ))?;
        let r = res.json::<MessageFileObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn list_message_file(
        &self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListMessageFile, APIError> {
        let mut url = format!("/threads/{}/messages/{}/files", thread_id, message_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListMessageFile>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn create_run(
        &self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> Result<RunObject, APIError> {
        let res = self.post(&format!("/threads/{}/runs", thread_id), &req)?;
        let r = res.json::<RunObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn retrieve_run(&self, thread_id: String, run_id: String) -> Result<RunObject, APIError> {
        let res = self.get(&format!("/threads/{}/runs/{}", thread_id, run_id))?;
        let r = res.json::<RunObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn modify_run(
        &self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> Result<RunObject, APIError> {
        let res = self.post(&format!("/threads/{}/runs/{}", thread_id, run_id), &req)?;
        let r = res.json::<RunObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn list_run(
        &self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRun, APIError> {
        let mut url = format!("/threads/{}/runs", thread_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListRun>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn cancel_run(&self, thread_id: String, run_id: String) -> Result<RunObject, APIError> {
        let empty_req = ModifyRunRequest::new();
        let res = self.post(
            &format!("/threads/{}/runs/{}/cancel", thread_id, run_id),
            &empty_req,
        )?;
        let r = res.json::<RunObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn create_thread_and_run(
        &self,
        req: CreateThreadAndRunRequest,
    ) -> Result<RunObject, APIError> {
        let res = self.post("/threads/runs", &req)?;
        let r = res.json::<RunObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStepObject, APIError> {
        let res = self.get(&format!(
            "/threads/{}/runs/{}/steps/{}",
            thread_id, run_id, step_id
        ))?;
        let r = res.json::<RunStepObject>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    pub fn list_run_step(
        &self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRunStep, APIError> {
        let mut url = format!("/threads/{}/runs/{}/steps", thread_id, run_id);
        url = Self::query_params(limit, order, after, before, url);
        let res = self.get(&url)?;
        let r = res.json::<ListRunStep>();
        match r {
            Ok(mut r) => {
                r.headers = Some(res.headers);
                Ok(r)
            }
            Err(e) => Err(APIError::GenericError(e)),
        }
    }

    fn is_beta(path: &str) -> bool {
        path.starts_with("/assistants") || path.starts_with("/threads")
    }

    fn query_params(
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
        mut url: String,
    ) -> String {
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        url
    }
}
