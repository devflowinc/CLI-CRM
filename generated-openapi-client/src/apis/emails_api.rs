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

/// struct for passing parameters to the method [`create_email`]
#[derive(Clone, Debug)]
pub struct CreateEmailParams {
    /// JSON request payload to create a new email
    pub create_email_req_payload: models::CreateEmailReqPayload
}

/// struct for passing parameters to the method [`delete_email`]
#[derive(Clone, Debug)]
pub struct DeleteEmailParams {
    /// The email id to use for the request
    pub email_id: String
}

/// struct for passing parameters to the method [`get_email`]
#[derive(Clone, Debug)]
pub struct GetEmailParams {
    /// The email id to use for the request
    pub email_id: String
}

/// struct for passing parameters to the method [`update_email`]
#[derive(Clone, Debug)]
pub struct UpdateEmailParams {
    /// The email id to use for the request
    pub email_id: String,
    /// JSON request payload to update the email
    pub update_email_req_payload: models::UpdateEmailReqPayload
}


/// struct for typed successes of method [`create_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmailSuccess {
    Status201(models::Org),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEmailSuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEmailSuccess {
    Status200(models::Org),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEmailSuccess {
    Status200(models::Org),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmailError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEmailError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEmailError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEmailError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}


pub async fn create_email(configuration: &configuration::Configuration, params: CreateEmailParams) -> Result<ResponseContent<CreateEmailSuccess>, Error<CreateEmailError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_email_req_payload = params.create_email_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/emails", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&create_email_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateEmailSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_email(configuration: &configuration::Configuration, params: DeleteEmailParams) -> Result<ResponseContent<DeleteEmailSuccess>, Error<DeleteEmailError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let email_id = params.email_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/emails/{email_id}", local_var_configuration.base_path, email_id=crate::apis::urlencode(email_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteEmailSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_email(configuration: &configuration::Configuration, params: GetEmailParams) -> Result<ResponseContent<GetEmailSuccess>, Error<GetEmailError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let email_id = params.email_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/emails/{email_id}", local_var_configuration.base_path, email_id=crate::apis::urlencode(email_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetEmailSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_email(configuration: &configuration::Configuration, params: UpdateEmailParams) -> Result<ResponseContent<UpdateEmailSuccess>, Error<UpdateEmailError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let email_id = params.email_id;
    let update_email_req_payload = params.update_email_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/emails/{email_id}", local_var_configuration.base_path, email_id=crate::apis::urlencode(email_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_email_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateEmailSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

