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
pub struct TaskResourceListWithPagination {
    #[serde(rename = "data")]
    pub data: Box<models::TaskResourceList>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl TaskResourceListWithPagination {
    pub fn new(data: models::TaskResourceList, total: i64) -> TaskResourceListWithPagination {
        TaskResourceListWithPagination {
            data: Box::new(data),
            total,
        }
    }
}

