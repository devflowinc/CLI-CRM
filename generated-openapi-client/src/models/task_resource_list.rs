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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskResourceList {
    TaskResourceListOneOf(Box<models::TaskResourceListOneOf>),
    TaskResourceListOneOf1(Box<models::TaskResourceListOneOf1>),
    TaskResourceListOneOf2(Box<models::TaskResourceListOneOf2>),
}

impl Default for TaskResourceList {
    fn default() -> Self {
        Self::TaskResourceListOneOf(Default::default())
    }
}
