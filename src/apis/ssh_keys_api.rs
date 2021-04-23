/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `create_ssh_key`
#[derive(Clone, Debug, Default)]
pub struct CreateSshKeyParams {
    pub create_ssh_key_request: Option<crate::models::CreateSshKeyRequest>
}

/// struct for passing parameters to the method `delete_ssh_key`
#[derive(Clone, Debug, Default)]
pub struct DeleteSshKeyParams {
    /// ID of the SSH key
    pub id: String
}

/// struct for passing parameters to the method `get_ssh_key`
#[derive(Clone, Debug, Default)]
pub struct GetSshKeyParams {
    /// ID of the SSH key
    pub id: i32
}

/// struct for passing parameters to the method `list_ssh_keys`
#[derive(Clone, Debug, Default)]
pub struct ListSshKeysParams {
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used to filter resources by their name. The response will only contain the resources matching the specified name
    pub name: Option<String>,
    /// Can be used to filter SSH keys by their fingerprint. The response will only contain the SSH key matching the specified fingerprint.
    pub fingerprint: Option<String>,
    /// Can be used to filter resources by labels. The response will only contain resources matching the label selector.
    pub label_selector: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>
}

/// struct for passing parameters to the method `replace_ssh_key`
#[derive(Clone, Debug, Default)]
pub struct ReplaceSshKeyParams {
    /// ID of the SSH key
    pub id: String,
    pub replace_ssh_key_request: Option<crate::models::ReplaceSshKeyRequest>
}


/// struct for typed errors of method `create_ssh_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSshKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_ssh_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSshKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_ssh_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSshKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_ssh_keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSshKeysError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_ssh_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceSshKeyError {
    UnknownValue(serde_json::Value),
}


/// Creates a new SSH key with the given `name` and `public_key`. Once an SSH key is created, it can be used in other calls such as creating Servers.
pub async fn create_ssh_key(configuration: &configuration::Configuration, params: CreateSshKeyParams) -> Result<crate::models::CreateSshKeyResponse, Error<CreateSshKeyError>> {
    // unbox the parameters
    let create_ssh_key_request = params.create_ssh_key_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ssh_keys", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_ssh_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSshKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes an SSH key. It cannot be used anymore.
pub async fn delete_ssh_key(configuration: &configuration::Configuration, params: DeleteSshKeyParams) -> Result<(), Error<DeleteSshKeyError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ssh_keys/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteSshKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific SSH key object.
pub async fn get_ssh_key(configuration: &configuration::Configuration, params: GetSshKeyParams) -> Result<crate::models::GetSshKeyResponse, Error<GetSshKeyError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ssh_keys/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSshKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all SSH key objects.
pub async fn list_ssh_keys(configuration: &configuration::Configuration, params: ListSshKeysParams) -> Result<crate::models::ListSshKeysResponse, Error<ListSshKeysError>> {
    // unbox the parameters
    let sort = params.sort;
    let name = params.name;
    let fingerprint = params.fingerprint;
    let label_selector = params.label_selector;
    let page = params.page;
    let per_page = params.per_page;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ssh_keys", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fingerprint {
        local_var_req_builder = local_var_req_builder.query(&[("fingerprint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("label_selector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListSshKeysError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an SSH key. You can update an SSH key name and an SSH key labels.  Please note that when updating labels, the SSH key current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body. 
pub async fn replace_ssh_key(configuration: &configuration::Configuration, params: ReplaceSshKeyParams) -> Result<crate::models::ReplaceSshKeyResponse, Error<ReplaceSshKeyError>> {
    // unbox the parameters
    let id = params.id;
    let replace_ssh_key_request = params.replace_ssh_key_request;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ssh_keys/{id}", configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_ssh_key_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceSshKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

