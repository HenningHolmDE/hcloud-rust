/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.21.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`apply_to_resources`]
#[derive(Clone, Debug, Default)]
pub struct ApplyToResourcesParams {
    /// ID of the Firewall.
    pub id: i64,
    pub apply_to_resources_request: Option<models::ApplyToResourcesRequest>,
}

/// struct for passing parameters to the method [`create_firewall`]
#[derive(Clone, Debug, Default)]
pub struct CreateFirewallParams {
    pub create_firewall_request: Option<models::CreateFirewallRequest>,
}

/// struct for passing parameters to the method [`delete_firewall`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFirewallParams {
    /// ID of the Firewall.
    pub id: i64,
}

/// struct for passing parameters to the method [`get_action_for_firewall`]
#[derive(Clone, Debug, Default)]
pub struct GetActionForFirewallParams {
    /// ID of the Firewall.
    pub id: i64,
    /// ID of the Action.
    pub action_id: i64,
}

/// struct for passing parameters to the method [`get_firewall`]
#[derive(Clone, Debug, Default)]
pub struct GetFirewallParams {
    /// ID of the Firewall.
    pub id: i64,
}

/// struct for passing parameters to the method [`get_firewall_action`]
#[derive(Clone, Debug, Default)]
pub struct GetFirewallActionParams {
    /// ID of the Action
    pub id: i64,
}

/// struct for passing parameters to the method [`list_actions_for_firewall`]
#[derive(Clone, Debug, Default)]
pub struct ListActionsForFirewallParams {
    /// ID of the Firewall
    pub id: i64,
    /// Sort actions by field and direction. Can be used multiple times. For more information, see \"[Sorting](#sorting)\".
    pub sort: Option<String>,
    /// Filter the actions by status. Can be used multiple times. The response will only contain actions matching the specified statuses.
    pub status: Option<String>,
    /// Page number to return. For more information, see \"[Pagination](#pagination)\".
    pub page: Option<i64>,
    /// Maximum number of entries returned per page. For more information, see \"[Pagination](#pagination)\".
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`list_firewall_actions`]
#[derive(Clone, Debug, Default)]
pub struct ListFirewallActionsParams {
    /// Filter the actions by ID. Can be used multiple times. The response will only contain actions matching the specified IDs.
    pub id: Option<i64>,
    /// Sort actions by field and direction. Can be used multiple times. For more information, see \"[Sorting](#sorting)\".
    pub sort: Option<String>,
    /// Filter the actions by status. Can be used multiple times. The response will only contain actions matching the specified statuses.
    pub status: Option<String>,
    /// Page number to return. For more information, see \"[Pagination](#pagination)\".
    pub page: Option<i64>,
    /// Maximum number of entries returned per page. For more information, see \"[Pagination](#pagination)\".
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`list_firewalls`]
#[derive(Clone, Debug, Default)]
pub struct ListFirewallsParams {
    /// Sort resources by field and direction. Can be used multiple times. For more information, see \"[Sorting](#sorting)\".
    pub sort: Option<String>,
    /// Filter resources by their name. The response will only contain the resources matching the specified name.
    pub name: Option<String>,
    /// Filter resources by labels. The response will only contain resources matching the label selector. For more information, see \"[Label Selector](#label-selector)\".
    pub label_selector: Option<String>,
    /// Page number to return. For more information, see \"[Pagination](#pagination)\".
    pub page: Option<i64>,
    /// Maximum number of entries returned per page. For more information, see \"[Pagination](#pagination)\".
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`remove_from_resources`]
#[derive(Clone, Debug, Default)]
pub struct RemoveFromResourcesParams {
    /// ID of the Firewall.
    pub id: i64,
    pub remove_from_resources_request: Option<models::RemoveFromResourcesRequest>,
}

/// struct for passing parameters to the method [`replace_firewall`]
#[derive(Clone, Debug, Default)]
pub struct ReplaceFirewallParams {
    /// ID of the Firewall.
    pub id: i64,
    pub replace_firewall_request: Option<models::ReplaceFirewallRequest>,
}

/// struct for passing parameters to the method [`set_rules`]
#[derive(Clone, Debug, Default)]
pub struct SetRulesParams {
    /// ID of the Firewall.
    pub id: i64,
    pub set_rules_request: Option<models::SetRulesRequest>,
}

/// struct for typed errors of method [`apply_to_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplyToResourcesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_action_for_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_firewall_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFirewallActionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_actions_for_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_firewall_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFirewallActionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_firewalls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFirewallsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_from_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveFromResourcesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_firewall`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceFirewallError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetRulesError {
    UnknownValue(serde_json::Value),
}

/// Applies one Firewall to multiple resources.  Currently servers (public network interface) and label selectors are supported.  #### Call specific error codes  | Code                          | Description                                                   | |-------------------------------|---------------------------------------------------------------| | `firewall_already_applied`    | Firewall was already applied on resource                      | | `incompatible_network_type`   | The Network type is incompatible for the given resource       | | `firewall_resource_not_found` | The resource the Firewall should be attached to was not found |
pub async fn apply_to_resources(
    configuration: &configuration::Configuration,
    params: ApplyToResourcesParams,
) -> Result<models::ApplyToResourcesResponse, Error<ApplyToResourcesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let apply_to_resources_request = params.apply_to_resources_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}/actions/apply_to_resources",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&apply_to_resources_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApplyToResourcesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new Firewall.  #### Call specific error codes  | Code                          | Description                                                   | |------------------------------ |-------------------------------------------------------------- | | `server_already_added`        | Server added more than one time to resource                   | | `incompatible_network_type`   | The Network type is incompatible for the given resource       | | `firewall_resource_not_found` | The resource the Firewall should be attached to was not found |
pub async fn create_firewall(
    configuration: &configuration::Configuration,
    params: CreateFirewallParams,
) -> Result<models::CreateFirewallResponse, Error<CreateFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_firewall_request = params.create_firewall_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/firewalls", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_firewall_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateFirewallError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a Firewall.  #### Call specific error codes  | Code                 | Description                               | |--------------------- |-------------------------------------------| | `resource_in_use`    | Firewall must not be in use to be deleted |
pub async fn delete_firewall(
    configuration: &configuration::Configuration,
    params: DeleteFirewallParams,
) -> Result<(), Error<DeleteFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}",
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
        let local_var_entity: Option<DeleteFirewallError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific Action for a Firewall.
pub async fn get_action_for_firewall(
    configuration: &configuration::Configuration,
    params: GetActionForFirewallParams,
) -> Result<models::GetActionForFirewallResponse, Error<GetActionForFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let action_id = params.action_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}/actions/{action_id}",
        local_var_configuration.base_path,
        id = id,
        action_id = action_id
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
        let local_var_entity: Option<GetActionForFirewallError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a specific Firewall object.
pub async fn get_firewall(
    configuration: &configuration::Configuration,
    params: GetFirewallParams,
) -> Result<models::GetFirewallResponse, Error<GetFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}",
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
        let local_var_entity: Option<GetFirewallError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific Action object.
pub async fn get_firewall_action(
    configuration: &configuration::Configuration,
    params: GetFirewallActionParams,
) -> Result<models::GetActionResponse, Error<GetFirewallActionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/actions/{id}",
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
        let local_var_entity: Option<GetFirewallActionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Action objects for a Firewall. You can sort the results by using the `sort` URI parameter, and filter them with the `status` parameter.
pub async fn list_actions_for_firewall(
    configuration: &configuration::Configuration,
    params: ListActionsForFirewallParams,
) -> Result<models::ListActionsForFirewallResponse, Error<ListActionsForFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}/actions",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder =
            local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListActionsForFirewallError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Action objects. You can `sort` the results by using the sort URI parameter, and filter them with the `status` and `id` parameter.
pub async fn list_firewall_actions(
    configuration: &configuration::Configuration,
    params: ListFirewallActionsParams,
) -> Result<models::ListActionsResponse, Error<ListFirewallActionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/firewalls/actions", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder =
            local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListFirewallActionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Firewall objects.
pub async fn list_firewalls(
    configuration: &configuration::Configuration,
    params: ListFirewallsParams,
) -> Result<models::ListFirewallsResponse, Error<ListFirewallsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    let name = params.name;
    let label_selector = params.label_selector;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/firewalls", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListFirewallsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes one Firewall from multiple resources.  Currently only Servers (and their public network interfaces) are supported.  #### Call specific error codes  | Code                                  | Description                                                            | |---------------------------------------|------------------------------------------------------------------------| | `firewall_already_removed`            | Firewall was already removed from the resource                         | | `firewall_resource_not_found`         | The resource the Firewall should be attached to was not found          | | `firewall_managed_by_label_selector`  | Firewall was applied via label selector and cannot be removed manually |
pub async fn remove_from_resources(
    configuration: &configuration::Configuration,
    params: RemoveFromResourcesParams,
) -> Result<models::RemoveFromResourcesResponse, Error<RemoveFromResourcesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let remove_from_resources_request = params.remove_from_resources_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}/actions/remove_from_resources",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&remove_from_resources_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RemoveFromResourcesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the Firewall.  Note that when updating labels, the Firewall's current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body.  Note: if the Firewall object changes during the request, the response will be a “conflict” error.
pub async fn replace_firewall(
    configuration: &configuration::Configuration,
    params: ReplaceFirewallParams,
) -> Result<models::ReplaceFirewallResponse, Error<ReplaceFirewallError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let replace_firewall_request = params.replace_firewall_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}",
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
    local_var_req_builder = local_var_req_builder.json(&replace_firewall_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceFirewallError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the rules of a Firewall.  All existing rules will be overwritten. Pass an empty `rules` array to remove all rules. The maximum amount of rules that can be defined is 50.  #### Call specific error codes  | Code                          | Description                                                   | |-------------------------------|---------------------------------------------------------------| | `firewall_resource_not_found` | The resource the Firewall should be attached to was not found |
pub async fn set_rules(
    configuration: &configuration::Configuration,
    params: SetRulesParams,
) -> Result<models::SetRulesResponse, Error<SetRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let set_rules_request = params.set_rules_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/firewalls/{id}/actions/set_rules",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&set_rules_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
