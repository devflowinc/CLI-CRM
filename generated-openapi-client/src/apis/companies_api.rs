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

/// struct for passing parameters to the method [`create_company`]
#[derive(Clone, Debug)]
pub struct CreateCompanyParams {
    /// The organization id to use for the request
    pub organization: String,
    /// JSON request payload to create a new company
    pub create_company_req_payload: models::CreateCompanyReqPayload
}

/// struct for passing parameters to the method [`delete_company`]
#[derive(Clone, Debug)]
pub struct DeleteCompanyParams {
    /// The organization id to use for the request
    pub organization: String,
    /// The id of the company you want to delete.
    pub company_id: String
}

/// struct for passing parameters to the method [`get_companies_for_org`]
#[derive(Clone, Debug)]
pub struct GetCompaniesForOrgParams {
    /// The organization id to use for the request
    pub organization: String,
    /// Limit the number of results. Default is 10
    pub limit: Option<i64>,
    /// Offset the results. Default is 0
    pub offset: Option<i64>
}

/// struct for passing parameters to the method [`get_company_by_id`]
#[derive(Clone, Debug)]
pub struct GetCompanyByIdParams {
    /// The organization id to use for the request
    pub organization: String,
    /// The id of the company you want to fetch.
    pub company_id: String
}

/// struct for passing parameters to the method [`update_company`]
#[derive(Clone, Debug)]
pub struct UpdateCompanyParams {
    /// The organization id to use for the request
    pub organization: String,
    /// The id of the note you want to update.
    pub company_id: String,
    /// JSON request payload to rename the company
    pub update_company_req_payload: models::UpdateCompanyReqPayload
}


/// struct for typed successes of method [`create_company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompanySuccess {
    Status201(models::Company),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`delete_company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCompanySuccess {
    Status204(),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_companies_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompaniesForOrgSuccess {
    Status200(Vec<models::Company>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`get_company_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompanyByIdSuccess {
    Status200(models::Company),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`update_company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCompanySuccess {
    Status200(models::Company),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompanyError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCompanyError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_companies_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompaniesForOrgError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_company_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCompanyByIdError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_company`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCompanyError {
    Status401(models::ErrorRespPayload),
    UnknownValue(serde_json::Value),
}


pub async fn create_company(configuration: &configuration::Configuration, params: CreateCompanyParams) -> Result<ResponseContent<CreateCompanySuccess>, Error<CreateCompanyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let create_company_req_payload = params.create_company_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/companies", local_var_configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&create_company_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<CreateCompanySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<CreateCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_company(configuration: &configuration::Configuration, params: DeleteCompanyParams) -> Result<ResponseContent<DeleteCompanySuccess>, Error<DeleteCompanyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let company_id = params.company_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/companies/{company_id}", local_var_configuration.base_path, company_id=crate::apis::urlencode(company_id));
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
        let local_var_entity: Option<DeleteCompanySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<DeleteCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_companies_for_org(configuration: &configuration::Configuration, params: GetCompaniesForOrgParams) -> Result<ResponseContent<GetCompaniesForOrgSuccess>, Error<GetCompaniesForOrgError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let limit = params.limit;
    let offset = params.offset;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/companies", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetCompaniesForOrgSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCompaniesForOrgError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_company_by_id(configuration: &configuration::Configuration, params: GetCompanyByIdParams) -> Result<ResponseContent<GetCompanyByIdSuccess>, Error<GetCompanyByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let company_id = params.company_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/companies/{company_id}", local_var_configuration.base_path, company_id=crate::apis::urlencode(company_id));
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
        let local_var_entity: Option<GetCompanyByIdSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<GetCompanyByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_company(configuration: &configuration::Configuration, params: UpdateCompanyParams) -> Result<ResponseContent<UpdateCompanySuccess>, Error<UpdateCompanyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let organization = params.organization;
    let company_id = params.company_id;
    let update_company_req_payload = params.update_company_req_payload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/companies/{company_id}", local_var_configuration.base_path, company_id=crate::apis::urlencode(company_id));
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
    local_var_req_builder = local_var_req_builder.json(&update_company_req_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateCompanySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateCompanyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

