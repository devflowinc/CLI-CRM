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
pub struct TaskResourceOneOf {
    #[serde(rename = "Link")]
    pub link: Box<models::TaskLink>,
}

impl TaskResourceOneOf {
    pub fn new(link: models::TaskLink) -> TaskResourceOneOf {
        TaskResourceOneOf {
            link: Box::new(link),
        }
    }
}

