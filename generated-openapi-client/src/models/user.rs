/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl User {
    pub fn new(created_at: String, email: String, id: String, updated_at: String) -> User {
        User {
            created_at,
            email,
            id,
            name: None,
            updated_at,
        }
    }
}

