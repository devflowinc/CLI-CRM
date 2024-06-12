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
pub struct ContactList {
    #[serde(rename = "contacts")]
    pub contacts: Vec<models::Contact>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl ContactList {
    pub fn new(contacts: Vec<models::Contact>, total: i64) -> ContactList {
        ContactList {
            contacts,
            total,
        }
    }
}

