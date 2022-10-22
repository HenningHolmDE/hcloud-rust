/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`create_placementgroup`]
#[derive(Clone, Debug, Default)]
pub struct CreatePlacementgroupParams {
    pub create_placementgroup_request: Option<crate::models::CreatePlacementgroupRequest>,
}

/// struct for passing parameters to the method [`delete_placementgroup`]
#[derive(Clone, Debug, Default)]
pub struct DeletePlacementgroupParams {
    /// ID of the resource
    pub id: i32,
}

/// struct for passing parameters to the method [`get_placementgroup`]
#[derive(Clone, Debug, Default)]
pub struct GetPlacementgroupParams {
    /// ID of the resource
    pub id: i32,
}

/// struct for passing parameters to the method [`list_placementgroups`]
#[derive(Clone, Debug, Default)]
pub struct ListPlacementgroupsParams {
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used to filter resources by their name. The response will only contain the resources matching the specified name
    pub name: Option<String>,
    /// Can be used to filter resources by labels. The response will only contain resources matching the label selector.
    pub label_selector: Option<String>,
    /// Can be used multiple times. The response will only contain PlacementGroups matching the type.
    pub _type: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>,
}

/// struct for passing parameters to the method [`replace_placementgroup`]
#[derive(Clone, Debug, Default)]
pub struct ReplacePlacementgroupParams {
    /// ID of the resource
    pub id: i32,
    pub replace_placementgroup_request: Option<crate::models::ReplacePlacementgroupRequest>,
}

/// struct for typed errors of method [`create_placementgroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePlacementgroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_placementgroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePlacementgroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_placementgroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPlacementgroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_placementgroups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPlacementgroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_placementgroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplacePlacementgroupError {
    UnknownValue(serde_json::Value),
}

/// Creates a new PlacementGroup.
pub async fn create_placementgroup(
    configuration: &configuration::Configuration,
    params: CreatePlacementgroupParams,
) -> Result<crate::models::CreatePlacementgroupResponse, Error<CreatePlacementgroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_placementgroup_request = params.create_placementgroup_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/placement_groups", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_placementgroup_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreatePlacementgroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a PlacementGroup.
pub async fn delete_placementgroup(
    configuration: &configuration::Configuration,
    params: DeletePlacementgroupParams,
) -> Result<(), Error<DeletePlacementgroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/placement_groups/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeletePlacementgroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a specific PlacementGroup object.
pub async fn get_placementgroup(
    configuration: &configuration::Configuration,
    params: GetPlacementgroupParams,
) -> Result<crate::models::GetPlacementgroupResponse, Error<GetPlacementgroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/placement_groups/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<GetPlacementgroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all PlacementGroup objects.
pub async fn list_placementgroups(
    configuration: &configuration::Configuration,
    params: ListPlacementgroupsParams,
) -> Result<crate::models::ListPlacementgroupsResponse, Error<ListPlacementgroupsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    let name = params.name;
    let label_selector = params.label_selector;
    let _type = params._type;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/placement_groups", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_selector {
        local_var_req_builder =
            local_var_req_builder.query(&[("label_selector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<ListPlacementgroupsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the PlacementGroup properties.  Note that when updating labels, the PlacementGroup’s current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body.  Note: if the PlacementGroup object changes during the request, the response will be a “conflict” error.
pub async fn replace_placementgroup(
    configuration: &configuration::Configuration,
    params: ReplacePlacementgroupParams,
) -> Result<crate::models::ReplacePlacementgroupResponse, Error<ReplacePlacementgroupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let replace_placementgroup_request = params.replace_placementgroup_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/placement_groups/{id}",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_placementgroup_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplacePlacementgroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
