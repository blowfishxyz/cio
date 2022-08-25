use chrono::{DateTime, Utc};
use reqwest::{Method, Url};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::{to_client_response, ScimError, ScimListResponse};
use crate::Inner;

/// A client for making requests to the Airtable Enterprise SCIM Group endpoints. An [AirtableScimUserClient]
/// can be retrieved from an [AirtableScimClient][crate::AirtableScimClient]. Supports listing, reading, creating,
/// and updating users as defined by the Airtable SCIM Users API. Patching users is not currently supported.
pub struct AirtableScimUserClient {
    inner: Inner,
}

impl AirtableScimUserClient {
    pub(super) fn new(inner: Inner) -> Self {
        Self { inner }
    }

    fn base_endpoint() -> &'static str {
        "https://airtable.com/scim/v2/Users"
    }

    fn url(base: &str, path: Option<&str>) -> Result<Url, ScimError> {
        let url = Url::parse(base)?;

        if let Some(path) = path {
            Ok(url.join("/")?.join(path)?)
        } else {
            Ok(url)
        }
    }

    /// Lists users as [SCIM User](https://datatracker.ietf.org/doc/html/rfc7643#section-4.1) objects
    ///
    /// From: <https://airtable.com/api/enterprise#scimUsersGet>
    pub async fn list(&self, filter: Option<&ScimListUserOptions>) -> Result<ScimListResponse<ScimUser>, ScimError> {
        let query_args = filter.map(|options| options.to_query_args());

        let req = self
            .inner
            .request(Method::GET, Self::url(Self::base_endpoint(), None)?, query_args)?
            .body("")
            .build()?;
        let resp = self.inner.execute(req).await?;

        to_client_response(resp).await
    }

    /// Get a single user as a [SCIM User](https://datatracker.ietf.org/doc/html/rfc7643#section-4.1) object
    ///
    /// From: <https://airtable.com/api/enterprise#scimUsersGetById>
    pub async fn get<T: AsRef<str>>(&self, id: T) -> Result<Option<ScimUser>, ScimError> {
        let req = self
            .inner
            .request(Method::GET, Self::url(Self::base_endpoint(), Some(id.as_ref()))?, None)?
            .body("")
            .build()?;
        let resp = self.inner.execute(req).await?;

        to_client_response(resp).await
    }

    /// Create a new user from a [SCIM User](https://datatracker.ietf.org/doc/html/rfc7643#section-4.1) object
    ///
    /// From: <https://airtable.com/api/enterprise#scimUserCreate>
    pub async fn create(&self, new_user: &ScimCreateUser) -> Result<ScimUser, ScimError> {
        let req = self
            .inner
            .request(Method::POST, Self::url(Self::base_endpoint(), None)?, None)?
            .body(serde_json::to_string(new_user)?)
            .build()?;
        let resp = self.inner.execute(req).await?;

        to_client_response(resp).await
    }

    /// Replace a user with a new a [SCIM User](https://datatracker.ietf.org/doc/html/rfc7643#section-4.1) object. Additionally during an update
    /// the `active` flag should be set to determine if the user is activated.
    ///
    /// From: <https://airtable.com/api/enterprise#scimUserUpdate>
    pub async fn update<T: AsRef<str>>(&self, id: T, user: &ScimUpdateUser) -> Result<ScimUser, ScimError> {
        let req = self
            .inner
            .request(Method::PUT, Self::url(Self::base_endpoint(), Some(id.as_ref()))?, None)?
            .body(serde_json::to_string(user)?)
            .build()?;
        let resp = self.inner.execute(req).await?;

        to_client_response(resp).await
    }

    // /// From: <https://airtable.com/api/enterprise#scimUserPatch>
    // pub async fn patch<T: AsRef<str>>(&self, id: T, operation: ScimPatchOp) -> Result<ScimUser, ScimError> {
    //     unimplemented!()
    // }
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimListUserOptions {
    pub start_index: Option<u32>,
    pub count: Option<u32>,
    pub filter: Option<ScimListUserFilter>,
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimListUserFilter {
    pub user_name: Option<String>,
}

impl ScimListUserOptions {
    pub fn to_query_args(&self) -> Vec<(&str, String)> {
        let mut args = vec![];

        if let Some(start_index) = self.start_index {
            args.push(("startIndex", start_index.to_string()));
        }

        if let Some(count) = self.count {
            args.push(("count", count.to_string()));
        }

        if let Some(filter) = &self.filter {
            let mut filter_conditions = String::default();

            if let Some(user_name) = &filter.user_name {
                filter_conditions.push_str(&format!(r#"userName+eq+"{}""#, user_name));
            }

            if !filter_conditions.is_empty() {
                args.push(("fitler", filter_conditions));
            }
        }

        args
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimUser {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "userName")]
    pub username: String,
    pub name: ScimName,
    pub active: bool,
    pub meta: ScimUserMeta,
    pub emails: Vec<ScimUserEmail>,
    #[serde(flatten)]
    pub extensions: HashMap<String, HashMap<String, Value>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimName {
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimUserMeta {
    pub created: DateTime<Utc>,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub location: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimUserEmail {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimCreateUser {
    pub schemas: Vec<String>,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: ScimName,
    /// The title field is available in create and update requests, but it is not returned in
    /// retrieval responses
    /// See: https://airtable.com/api/enterprise#scimUserFieldTypes
    pub title: String,
    #[serde(flatten)]
    pub extensions: HashMap<String, HashMap<String, Value>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema, Deserialize)]
pub struct ScimUpdateUser {
    pub schemas: Vec<String>,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: ScimName,
    /// The title field is available in create and update requests, but it is not returned in
    /// retrieval responses
    /// See: https://airtable.com/api/enterprise#scimUserFieldTypes
    pub title: String,
    pub active: bool,
    #[serde(flatten)]
    pub extensions: HashMap<String, HashMap<String, Value>>,
}

#[cfg(test)]
mod tests {
    use super::{ScimListUserFilter, ScimListUserOptions};

    #[test]
    fn test_serialize_list_options() {
        let options = ScimListUserOptions {
            start_index: Some(5),
            count: Some(10),
            filter: Some(ScimListUserFilter {
                user_name: Some("foo@bar.com".to_string()),
            }),
        };

        let expected = vec![
            ("startIndex", "5".to_string()),
            ("count", "10".to_string()),
            ("fitler", r#"userName+eq+"foo@bar.com""#.to_string()),
        ];

        assert_eq!(expected, options.to_query_args());
    }
}
