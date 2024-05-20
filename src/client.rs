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
use std::{collections::HashMap, path::Path};
use tracing::error;

const API_URL_V1: &str = "https://api.openai.com/v1";

type APIResult<T> = Result<T, APIError>;

pub struct Client {
    pub endpoint: String,
    pub api_key: String,
    pub client: ReqwestClient,
}

fn headermap_to_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .filter_map(|(k, v)| Some((k.as_str().to_string(), v.to_str().ok()?.to_string())))
        .collect()
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let endpoint =
            std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            endpoint,
            api_key,
            client,
        }
    }

    pub async fn post<T: serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> APIResult<Response> {
        let url = format!("{}{}", self.endpoint, path);
        let response = self.client.post(&url).json(params).send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(APIError::Unknown(format!(
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
        let builder = self.client.get(&url);

        let response = builder.send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(APIError::Unknown(format!(
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
        let builder = self.client.delete(&url);

        let response = builder.send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(res)
                } else {
                    Err(APIError::Unknown(format!(
                        "{}: {}",
                        res.status(),
                        res.text().await.unwrap_or_default()
                    )))
                }
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn completion(
        &self,
        req: CompletionRequest,
    ) -> APIResult<CompletionResponse> {
        let url = format!("{}{}", self.endpoint, "/completions");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<CompletionResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn edit(&self, req: EditRequest) -> APIResult<EditResponse> {
        let url = format!("{}{}", self.endpoint, "/edits");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<EditResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> APIResult<ImageGenerationResponse> {
        let url = format!("{}{}", self.endpoint, "/images/generations");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<ImageGenerationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn image_edit(
        &self,
        req: ImageEditRequest,
    ) -> APIResult<ImageEditResponse> {
        let url = format!("{}{}", self.endpoint, "/images/edits");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<ImageEditResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> APIResult<ImageVariationResponse> {
        let url = format!("{}{}", self.endpoint, "/images/variations");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<ImageVariationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn embedding(&self, req: EmbeddingRequest) -> APIResult<EmbeddingResponse> {
        let url = format!("{}{}", self.endpoint, "/embeddings");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<EmbeddingResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn file_list(&self) -> APIResult<FileListResponse> {
        let url = format!("{}{}", self.endpoint, "/files");
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<FileListResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn file_upload(
        &self,
        req: FileUploadRequest,
    ) -> APIResult<FileUploadResponse> {
        let url = format!("{}{}", self.endpoint, "/files");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<FileUploadResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn file_delete(
        &self,
        req: FileDeleteRequest,
    ) -> APIResult<FileDeleteResponse> {
        let url = format!("{}{}/{}", self.endpoint, "/files", req.file_id);
        let response = self.client.delete(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<FileDeleteResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn file_retrieve(
        &self,
        req: FileRetrieveRequest,
    ) -> APIResult<FileRetrieveResponse> {
        let url = format!("{}{}/{}", self.endpoint, "/files", req.file_id);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<FileRetrieveResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> APIResult<FileRetrieveContentResponse> {
        let url = format!("{}{}/{}/content", self.endpoint, "/files", req.file_id);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<FileRetrieveContentResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> APIResult<ChatCompletionResponse> {
        let url = format!("{}{}", self.endpoint, "/chat/completions");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<ChatCompletionResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => {
                error!("Chat completion error: {e:?}");
                Err(APIError::ReqwestError(e))
            }
        }
    }

    pub async fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> APIResult<AudioTranscriptionResponse> {
        let url = format!("{}{}", self.endpoint, "/audio/transcriptions");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<AudioTranscriptionResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> APIResult<AudioTranslationResponse> {
        let url = format!("{}{}", self.endpoint, "/audio/translations");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<AudioTranslationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn audio_speech(
        &self,
        req: AudioSpeechRequest,
    ) -> APIResult<AudioSpeechResponse> {
        let url = format!("{}{}", self.endpoint, "/audio/speech");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();

        let bytes = response.bytes().await?;
        let path = Path::new(&req.output);
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;
        file.write_all(&bytes).await?;

        Ok(AudioSpeechResponse {
            result: true,
            headers: Some(headermap_to_map(&headers)),
        })
    }

    pub async fn create_fine_tuning_job(
        &self,
        req: CreateFineTuningJobRequest,
    ) -> APIResult<FineTuningJobObject> {
        let url = format!("{}{}", self.endpoint, "/fine_tuning/jobs");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<FineTuningJobObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_fine_tuning_jobs(
        &self,
    ) -> APIResult<FineTuningPagination<FineTuningJobObject>> {
        let url = format!("{}{}", self.endpoint, "/fine_tuning/jobs");
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();

        match response
            .json::<FineTuningPagination<FineTuningJobObject>>()
            .await
        {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_fine_tuning_job_events(
        &self,
        req: ListFineTuningJobEventsRequest,
    ) -> APIResult<FineTuningPagination<FineTuningJobEvent>> {
        let url = format!(
            "{}{}{}/events",
            self.endpoint, "/fine_tuning/jobs/", req.fine_tuning_job_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response
            .json::<FineTuningPagination<FineTuningJobEvent>>()
            .await
        {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_fine_tuning_job(
        &self,
        req: RetrieveFineTuningJobRequest,
    ) -> APIResult<FineTuningJobObject> {
        let url = format!(
            "{}{}{}",
            self.endpoint, "/fine_tuning/jobs/", req.fine_tuning_job_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<FineTuningJobObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn cancel_fine_tuning_job(
        &self,
        req: CancelFineTuningJobRequest,
    ) -> APIResult<FineTuningJobObject> {
        let url = format!(
            "{}{}{}/cancel",
            self.endpoint, "/fine_tuning/jobs/", req.fine_tuning_job_id
        );
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<FineTuningJobObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> APIResult<CreateModerationResponse> {
        let url = format!("{}{}", self.endpoint, "/moderations");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<CreateModerationResponse>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_assistant(
        &self,
        req: AssistantRequest,
    ) -> APIResult<AssistantObject> {
        let url = format!("{}{}", self.endpoint, "/assistants");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<AssistantObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_assistant(
        &self,
        assistant_id: String,
    ) -> APIResult<AssistantObject> {
        let url = format!("{}{}{}", self.endpoint, "/assistants/", assistant_id);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<AssistantObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn modify_assistant(
        &self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> APIResult<AssistantObject> {
        let url = format!("{}{}{}", self.endpoint, "/assistants/", assistant_id);
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<AssistantObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn delete_assistant(
        &self,
        assistant_id: String,
    ) -> APIResult<DeletionStatus> {
        let url = format!("{}{}{}", self.endpoint, "/assistants/", assistant_id);
        let response = self.client.delete(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<DeletionStatus>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_assistant(
        &self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> APIResult<ListAssistant> {
        let base_url = format!("{}{}", self.endpoint, "/assistants");
        let url = Client::query_params(limit, order, after, before, base_url);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ListAssistant>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_assistant_file(
        &self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> APIResult<AssistantFileObject> {
        let url = format!("{}{}{}/files", self.endpoint, "/assistants/", assistant_id);
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<AssistantFileObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    // TODO: FINISH

    pub async fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> APIResult<AssistantFileObject> {
        let url = format!(
            "{}{}{}/files/{}",
            self.endpoint, "/assistants/", assistant_id, file_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<AssistantFileObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> APIResult<DeletionStatus> {
        let url = format!(
            "{}{}{}/files/{}",
            self.endpoint, "/assistants/", assistant_id, file_id
        );
        let response = self.client.delete(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<DeletionStatus>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_assistant_file(
        &self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> APIResult<ListAssistantFile> {
        let base_url =
            format!("{}{}{}/files", self.endpoint, "/assistants/", assistant_id);
        let url = Client::query_params(limit, order, after, before, base_url);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ListAssistantFile>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_thread(
        &self,
        req: CreateThreadRequest,
    ) -> APIResult<ThreadObject> {
        let url = format!("{}{}", self.endpoint, "/threads");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<ThreadObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_thread(&self, thread_id: String) -> APIResult<ThreadObject> {
        let url = format!("{}{}{}", self.endpoint, "/threads/", thread_id);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ThreadObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn modify_thread(
        &self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> APIResult<ThreadObject> {
        let url = format!("{}{}{}", self.endpoint, "/threads/", thread_id);
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<ThreadObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn delete_thread(&self, thread_id: String) -> APIResult<DeletionStatus> {
        let url = format!("{}{}{}", self.endpoint, "/threads/", thread_id);
        let response = self.client.delete(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<DeletionStatus>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_message(
        &self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> APIResult<MessageObject> {
        let url = format!("{}{}{}/messages", self.endpoint, "/threads/", thread_id);
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<MessageObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> APIResult<MessageObject> {
        let url = format!(
            "{}{}{}/messages/{}",
            self.endpoint, "/threads/", thread_id, message_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<MessageObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> APIResult<MessageObject> {
        let url = format!(
            "{}{}{}/messages/{}",
            self.endpoint, "/threads/", thread_id, message_id
        );
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<MessageObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_messages(&self, thread_id: String) -> APIResult<ListMessage> {
        let url = format!("{}{}{}/messages", self.endpoint, "/threads/", thread_id);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ListMessage>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> APIResult<MessageFileObject> {
        let url = format!(
            "{}{}{}/messages/{}/files/{}",
            self.endpoint, "/threads/", thread_id, message_id, file_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<MessageFileObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_message_file(
        &self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> APIResult<ListMessageFile> {
        let base_url = format!(
            "{}{}{}/messages/{}/files",
            self.endpoint, "/threads/", thread_id, message_id
        );
        let url = Client::query_params(limit, order, after, before, base_url);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ListMessageFile>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_run(
        &self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> APIResult<RunObject> {
        let url = format!("{}{}{}/runs", self.endpoint, "/threads/", thread_id);
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<RunObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_run(
        &self,
        thread_id: String,
        run_id: String,
    ) -> APIResult<RunObject> {
        let url = format!(
            "{}{}{}/runs/{}",
            self.endpoint, "/threads/", thread_id, run_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<RunObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn modify_run(
        &self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> APIResult<RunObject> {
        let url = format!(
            "{}{}{}/runs/{}",
            self.endpoint, "/threads/", thread_id, run_id
        );
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<RunObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_run(
        &self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> APIResult<ListRun> {
        let base_url = format!("{}{}{}/runs", self.endpoint, "/threads/", thread_id);
        let url = Client::query_params(limit, order, after, before, base_url);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ListRun>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn cancel_run(
        &self,
        thread_id: String,
        run_id: String,
    ) -> APIResult<RunObject> {
        let url = format!(
            "{}{}{}/runs/{}/cancel",
            self.endpoint, "/threads/", thread_id, run_id
        );
        let empty_req = ModifyRunRequest::new(); // Ensure this call is appropriate for initializing your request.
        let response = self.client.post(&url).json(&empty_req).send().await?;
        let headers = response.headers().clone();
        match response.json::<RunObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn create_thread_and_run(
        &self,
        req: CreateThreadAndRunRequest,
    ) -> APIResult<RunObject> {
        let url = format!("{}{}", self.endpoint, "/threads/runs");
        let response = self.client.post(&url).json(&req).send().await?;
        let headers = response.headers().clone();
        match response.json::<RunObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> APIResult<RunStepObject> {
        let url = format!(
            "{}{}{}/runs/{}/steps/{}",
            self.endpoint, "/threads/", thread_id, run_id, step_id
        );
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<RunStepObject>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
    }

    pub async fn list_run_step(
        &self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> APIResult<ListRunStep> {
        let base_url = format!(
            "{}{}{}/runs/{}/steps",
            self.endpoint, "/threads/", thread_id, run_id
        );
        let url = Client::query_params(limit, order, after, before, base_url);
        let response = self.client.get(&url).send().await?;
        let headers = response.headers().clone();
        match response.json::<ListRunStep>().await {
            Ok(mut r) => {
                r.headers = Some(headermap_to_map(&headers));
                Ok(r)
            }
            Err(e) => Err(APIError::ReqwestError(e)),
        }
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
