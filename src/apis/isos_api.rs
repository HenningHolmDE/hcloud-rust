/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.4.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_iso`]
#[derive(Clone, Debug, Default)]
pub struct GetIsoParams {
    /// ID of the ISO
    pub id: i32
}

/// struct for passing parameters to the method [`list_isos`]
#[derive(Clone, Debug, Default)]
pub struct ListIsosParams {
    /// Can be used to filter ISOs by their name. The response will only contain the ISO matching the specified name.
    pub name: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>
}


/// struct for typed errors of method [`get_iso`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIsoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_isos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIsosError {
    UnknownValue(serde_json::Value),
}


/// Returns a specific ISO object.
pub async fn get_iso(configuration: &configuration::Configuration, params: GetIsoParams) -> Result<crate::models::GetIsoResponse, Error<GetIsoError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/isos/{id}", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetIsoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all available ISO objects.
pub async fn list_isos(configuration: &configuration::Configuration, params: ListIsosParams) -> Result<crate::models::ListIsosResponse, Error<ListIsosError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let page = params.page;
    let per_page = params.per_page;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/isos", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListIsosError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

