/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_note`]
#[derive(Clone, Debug)]
pub struct CreateNoteParams {
    /// The organization id to use for the request
    pub organization: String,
    /// JSON request payload to create a new note
    pub create_note_req_payload: models::CreateNoteReqPayload
}

/// struct for passing parameters to the method [`delete_note`]
#[derive(Clone, Debug)]
pub struct DeleteNoteParams {
    /// The organization id to use for the request
    pub organization: String,
    /// The id of the note you want to delete.
    pub note_id: String
}

/// struct for passing parameters to the method [`get_note_by_id`]
#[derive(Clone, Debug)]
pub struct GetNoteByIdParams {
    /// The organization id to use for the request
    pub organization: String,
    /// The id of the organization you want to fetch.
    pub note_id: String
}

/// struct for passing parameters to the method [`get_notes_for_org`]
#[derive(Clone, Debug)]
pub struct GetNotesForOrgParams {
    /// Limit the number of results. Default is 10
    pub limit: Option<i64>,
    /// Offset the results. Default is 0
    pub offset: Option<i64>
}

/// struct for passing parameters to the method [`update_note`]
#[derive(Clone, Debug)]
pub struct UpdateNoteParams {
    /// The organization id to use for the request
    pub organization: String,
    /// The id of the note you want to update.
    pub note_id: String,
    /// JSON request payload to rename the organization
    pub update_note_req_payload: models::UpdateNoteReqPayload
}


/// struct for typed successes of method [`create_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNoteSuccess {
    Status201(models::Note),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNoteSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_note_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNoteByIdSuccess {
    Status200(models::Note),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_notes_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNotesForOrgSuccess {
    Status200(Vec<models::Org>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNoteSuccess {
    Status200(models::Note),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNoteError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNoteError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_note_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNoteByIdError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_notes_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNotesForOrgError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_note`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNoteError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}


pub async fn create_note(configuration: &configuration::Configuration, params: CreateNoteParams) -> Result<ResponseContent<CreateNoteSuccess>, Error<CreateNoteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let create_note_req_payload = params.create_note_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/notes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&create_note_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateNoteSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateNoteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_note(configuration: &configuration::Configuration, params: DeleteNoteParams) -> Result<ResponseContent<DeleteNoteSuccess>, Error<DeleteNoteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let note_id = params.note_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/notes/{note_id}", local_var_configuration.base_path, note_id=crate::apis::urlencode(note_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<DeleteNoteSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteNoteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_note_by_id(configuration: &configuration::Configuration, params: GetNoteByIdParams) -> Result<ResponseContent<GetNoteByIdSuccess>, Error<GetNoteByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let note_id = params.note_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/notes/{note_id}", local_var_configuration.base_path, note_id=crate::apis::urlencode(note_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetNoteByIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetNoteByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_notes_for_org(configuration: &configuration::Configuration, params: GetNotesForOrgParams) -> Result<ResponseContent<GetNotesForOrgSuccess>, Error<GetNotesForOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let offset = params.offset;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/notes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<GetNotesForOrgSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetNotesForOrgError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_note(configuration: &configuration::Configuration, params: UpdateNoteParams) -> Result<ResponseContent<UpdateNoteSuccess>, Error<UpdateNoteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let note_id = params.note_id;
    let update_note_req_payload = params.update_note_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/notes/{note_id}", local_var_configuration.base_path, note_id=crate::apis::urlencode(note_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Organization", organization.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&update_note_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateNoteSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateNoteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

