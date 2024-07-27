/*
 * geoprox-server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RemoveKey : Arguments for removing a key
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveKey {
    /// resource key
    #[serde(rename = "key")]
    pub key: String,
}

impl RemoveKey {
    /// Arguments for removing a key
    pub fn new(key: String) -> RemoveKey {
        RemoveKey {
            key,
        }
    }
}
