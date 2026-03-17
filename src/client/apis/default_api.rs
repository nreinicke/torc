use super::ResponseContent;
use super::{ContentType, Error, configuration};
use crate::client::apis::workflows_api;
use crate::models;
use serde::{Deserialize, Serialize, de::Error as _};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTaskError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializeJobsError {
    Status409(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetJobStatusError {
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetWorkflowStatusError {
    UnknownValue(serde_json::Value),
}

pub fn get_task(
    configuration: &configuration::Configuration,
    id: i64,
) -> Result<models::TaskModel, Error<GetTaskError>> {
    let uri_str = format!("{}/tasks/{id}", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = configuration.apply_auth(req_builder);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TaskModel`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TaskModel`"
                ))))
            }
        }
    } else {
        let content = resp.text()?;
        let entity = match status.as_u16() {
            404 => serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(GetTaskError::Status404)
                .or_else(|| {
                    serde_json::from_str::<serde_json::Value>(&content)
                        .ok()
                        .map(GetTaskError::UnknownValue)
                }),
            500 => serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(GetTaskError::Status500)
                .or_else(|| {
                    serde_json::from_str::<serde_json::Value>(&content)
                        .ok()
                        .map(GetTaskError::UnknownValue)
                }),
            _ => serde_json::from_str::<serde_json::Value>(&content)
                .ok()
                .map(GetTaskError::UnknownValue),
        };
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub fn initialize_jobs(
    configuration: &configuration::Configuration,
    id: i64,
    only_uninitialized: Option<bool>,
    clear_ephemeral_user_data: Option<bool>,
    body: Option<serde_json::Value>,
) -> Result<serde_json::Value, Error<InitializeJobsError>> {
    initialize_jobs_with_async(
        configuration,
        id,
        only_uninitialized,
        clear_ephemeral_user_data,
        None,
        body,
    )
}

pub fn initialize_jobs_with_async(
    configuration: &configuration::Configuration,
    id: i64,
    only_uninitialized: Option<bool>,
    clear_ephemeral_user_data: Option<bool>,
    async_: Option<bool>,
    body: Option<serde_json::Value>,
) -> Result<serde_json::Value, Error<InitializeJobsError>> {
    let mut req_builder = configuration.client.request(
        reqwest::Method::POST,
        format!("{}/workflows/{id}/initialize_jobs", configuration.base_path),
    );

    if let Some(ref param_value) = only_uninitialized {
        req_builder = req_builder.query(&[("only_uninitialized", &param_value.to_string())]);
    }
    if let Some(ref param_value) = clear_ephemeral_user_data {
        req_builder = req_builder.query(&[("clear_ephemeral_user_data", &param_value.to_string())]);
    }
    if let Some(ref param_value) = async_ {
        req_builder = req_builder.query(&[("async", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = configuration.apply_auth(req_builder).json(&body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))))
            }
        }
    } else {
        let content = resp.text()?;
        let entity = match status.as_u16() {
            409 => serde_json::from_str::<models::ErrorResponse>(&content)
                .ok()
                .map(InitializeJobsError::Status409)
                .or_else(|| {
                    serde_json::from_str::<serde_json::Value>(&content)
                        .ok()
                        .map(InitializeJobsError::UnknownValue)
                }),
            _ => serde_json::from_str::<serde_json::Value>(&content)
                .ok()
                .map(InitializeJobsError::UnknownValue),
        };
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub fn reset_job_status(
    configuration: &configuration::Configuration,
    id: i64,
    failed_only: Option<bool>,
    _body: Option<serde_json::Value>,
) -> Result<serde_json::Value, Error<ResetJobStatusError>> {
    match workflows_api::reset_job_status(configuration, id, failed_only) {
        Ok(value) => serde_json::to_value(value).map_err(Error::from),
        Err(err) => Err(match err {
            Error::Reqwest(e) => Error::Reqwest(e),
            Error::Serde(e) => Error::Serde(e),
            Error::Io(e) => Error::Io(e),
            Error::ResponseError(resp) => Error::ResponseError(ResponseContent {
                status: resp.status,
                content: resp.content,
                entity: resp.entity.map(|e| match e {
                    workflows_api::ResetJobStatusError::UnknownValue(v) => {
                        ResetJobStatusError::UnknownValue(v)
                    }
                }),
            }),
        }),
    }
}

pub fn reset_workflow_status(
    configuration: &configuration::Configuration,
    id: i64,
    force: Option<bool>,
    _body: Option<serde_json::Value>,
) -> Result<serde_json::Value, Error<ResetWorkflowStatusError>> {
    workflows_api::reset_workflow_status(configuration, id, force).map_err(|err| match err {
        Error::Reqwest(e) => Error::Reqwest(e),
        Error::Serde(e) => Error::Serde(e),
        Error::Io(e) => Error::Io(e),
        Error::ResponseError(resp) => Error::ResponseError(ResponseContent {
            status: resp.status,
            content: resp.content,
            entity: resp.entity.map(|e| match e {
                workflows_api::ResetWorkflowStatusError::UnknownValue(v) => {
                    ResetWorkflowStatusError::UnknownValue(v)
                }
            }),
        }),
    })
}
