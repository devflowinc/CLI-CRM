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
pub struct ListTaskResourceResponseOneOf2 {
    #[serde(rename = "user")]
    pub user: Vec<models::User>,
}

impl ListTaskResourceResponseOneOf2 {
    pub fn new(user: Vec<models::User>) -> ListTaskResourceResponseOneOf2 {
        ListTaskResourceResponseOneOf2 {
            user,
        }
    }
}
