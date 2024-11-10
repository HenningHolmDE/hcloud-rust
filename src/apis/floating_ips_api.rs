/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.22.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`assign_floating_ip_to_server`]
#[derive(Clone, Debug, Default)]
pub struct AssignFloatingIpToServerParams {
    /// ID of the Floating IP.
    pub id: i64,
    pub assign_floating_ip_to_server_request: Option<models::AssignFloatingIpToServerRequest>,
}

/// struct for passing parameters to the method [`change_floating_ip_protection`]
#[derive(Clone, Debug, Default)]
pub struct ChangeFloatingIpProtectionParams {
    /// ID of the Floating IP.
    pub id: i64,
    pub body: Option<models::Protection>,
}

/// struct for passing parameters to the method [`change_reverse_dns_records_for_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct ChangeReverseDnsRecordsForFloatingIpParams {
    /// ID of the Floating IP.
    pub id: i64,
    /// The `ip` attributes specifies for which IP address the record is set. For IPv4 addresses this must be the exact address of the [Floating IP](#floating-ips). For IPv6 addresses this must be a single address within the `/64` subnet of the [Floating IP](#floating-ips).  The `dns_ptr` attribute specifies the hostname used for the IP address.  For IPv6 [Floating IPs](#floating-ips) up to 100 entries can be created.
    pub body: Option<models::DnsPtr>,
}

/// struct for passing parameters to the method [`create_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct CreateFloatingIpParams {
    /// The `type` argument is required while `home_location` and `server` are mutually exclusive.
    pub create_floating_ip_request: Option<models::CreateFloatingIpRequest>,
}

/// struct for passing parameters to the method [`delete_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFloatingIpParams {
    /// ID of the Floating IP.
    pub id: i64,
}

/// struct for passing parameters to the method [`get_action_for_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct GetActionForFloatingIpParams {
    /// ID of the Floating IP.
    pub id: i64,
    /// ID of the Action.
    pub action_id: i64,
}

/// struct for passing parameters to the method [`get_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct GetFloatingIpParams {
    /// ID of the Floating IP.
    pub id: i64,
}

/// struct for passing parameters to the method [`get_floating_ip_action`]
#[derive(Clone, Debug, Default)]
pub struct GetFloatingIpActionParams {
    /// ID of the Action
    pub id: i64,
}

/// struct for passing parameters to the method [`list_actions_for_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct ListActionsForFloatingIpParams {
    /// ID of the Floating IP.
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

/// struct for passing parameters to the method [`list_floating_ip_actions`]
#[derive(Clone, Debug, Default)]
pub struct ListFloatingIpActionsParams {
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

/// struct for passing parameters to the method [`list_floating_ips`]
#[derive(Clone, Debug, Default)]
pub struct ListFloatingIpsParams {
    /// Filter resources by their name. The response will only contain the resources matching the specified name.
    pub name: Option<String>,
    /// Filter resources by labels. The response will only contain resources matching the label selector. For more information, see \"[Label Selector](#label-selector)\".
    pub label_selector: Option<String>,
    /// Sort resources by field and direction. Can be used multiple times. For more information, see \"[Sorting](#sorting)\".
    pub sort: Option<String>,
    /// Page number to return. For more information, see \"[Pagination](#pagination)\".
    pub page: Option<i64>,
    /// Maximum number of entries returned per page. For more information, see \"[Pagination](#pagination)\".
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`replace_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct ReplaceFloatingIpParams {
    /// ID of the Floating IP.
    pub id: i64,
    pub replace_floating_ip_request: Option<models::ReplaceFloatingIpRequest>,
}

/// struct for passing parameters to the method [`unassign_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct UnassignFloatingIpParams {
    /// ID of the Floating IP.
    pub id: i64,
}

/// struct for typed errors of method [`assign_floating_ip_to_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssignFloatingIpToServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_floating_ip_protection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeFloatingIpProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_reverse_dns_records_for_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeReverseDnsRecordsForFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_action_for_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_floating_ip_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFloatingIpActionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_actions_for_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_floating_ip_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFloatingIpActionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_floating_ips`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFloatingIpsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unassign_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnassignFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// Assigns a [Floating IP](#floating-ips) to a [Server](#servers).
pub async fn assign_floating_ip_to_server(
    configuration: &configuration::Configuration,
    params: AssignFloatingIpToServerParams,
) -> Result<models::AssignFloatingIpToServerResponse, Error<AssignFloatingIpToServerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let assign_floating_ip_to_server_request = params.assign_floating_ip_to_server_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}/actions/assign",
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
    local_var_req_builder = local_var_req_builder.json(&assign_floating_ip_to_server_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AssignFloatingIpToServerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the protection settings configured for the [Floating IP](#floating-ips).
pub async fn change_floating_ip_protection(
    configuration: &configuration::Configuration,
    params: ChangeFloatingIpProtectionParams,
) -> Result<models::ChangeFloatingIpProtectionResponse, Error<ChangeFloatingIpProtectionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}/actions/change_protection",
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
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeFloatingIpProtectionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Change the reverse DNS records for this [Floating IP](#floating-ips).  Allows to modify the PTR records set for the IP address.
pub async fn change_reverse_dns_records_for_floating_ip(
    configuration: &configuration::Configuration,
    params: ChangeReverseDnsRecordsForFloatingIpParams,
) -> Result<
    models::ChangeReverseDnsRecordsForFloatingIpResponse,
    Error<ChangeReverseDnsRecordsForFloatingIpError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}/actions/change_dns_ptr",
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
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeReverseDnsRecordsForFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a [Floating IP](#floating-ips).  Provide the `server` attribute to assign the [Floating IP](#floating-ips) to that server or provide a `home_location` to locate the [Floating IP](#floating-ips) at. Note that the [Floating IP](#floating-ips) can be assigned to a [Server](#servers) in any [Location](#locations) later on. For optimal routing it is advised to use the [Floating IP](#floating-ips) in the same [Location](#locations) it was created in.
pub async fn create_floating_ip(
    configuration: &configuration::Configuration,
    params: CreateFloatingIpParams,
) -> Result<models::CreateFloatingIpResponse, Error<CreateFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_floating_ip_request = params.create_floating_ip_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/floating_ips", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_floating_ip_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a [Floating IP](#floating-ips).  If the IP is assigned to a resource it will be unassigned.
pub async fn delete_floating_ip(
    configuration: &configuration::Configuration,
    params: DeleteFloatingIpParams,
) -> Result<(), Error<DeleteFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}",
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
        let local_var_entity: Option<DeleteFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific [Action](#actions) for a [Floating IP](#floating-ips).
pub async fn get_action_for_floating_ip(
    configuration: &configuration::Configuration,
    params: GetActionForFloatingIpParams,
) -> Result<models::GetActionForFloatingIpResponse, Error<GetActionForFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let action_id = params.action_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}/actions/{action_id}",
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
        let local_var_entity: Option<GetActionForFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a single [Floating IP](#floating-ips).
pub async fn get_floating_ip(
    configuration: &configuration::Configuration,
    params: GetFloatingIpParams,
) -> Result<models::GetFloatingIpResponse, Error<GetFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}",
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
        let local_var_entity: Option<GetFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a single [Action](#actions).
pub async fn get_floating_ip_action(
    configuration: &configuration::Configuration,
    params: GetFloatingIpActionParams,
) -> Result<models::GetActionResponse, Error<GetFloatingIpActionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/actions/{id}",
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
        let local_var_entity: Option<GetFloatingIpActionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists [Actions](#actions) for a [Floating IP](#floating-ips).  Use the provided URI parameters to modify the result.
pub async fn list_actions_for_floating_ip(
    configuration: &configuration::Configuration,
    params: ListActionsForFloatingIpParams,
) -> Result<models::ListActionsForFloatingIpResponse, Error<ListActionsForFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}/actions",
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
        let local_var_entity: Option<ListActionsForFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists multiple [Actions](#actions).  Use the provided URI parameters to modify the result.
pub async fn list_floating_ip_actions(
    configuration: &configuration::Configuration,
    params: ListFloatingIpActionsParams,
) -> Result<models::ListActionsResponse, Error<ListFloatingIpActionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/floating_ips/actions", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListFloatingIpActionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List multiple [Floating IPs](#floating-ips).  Use the provided URI parameters to modify the result.
pub async fn list_floating_ips(
    configuration: &configuration::Configuration,
    params: ListFloatingIpsParams,
) -> Result<models::ListFloatingIpsResponse, Error<ListFloatingIpsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let label_selector = params.label_selector;
    let sort = params.sort;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/floating_ips", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_selector {
        local_var_req_builder =
            local_var_req_builder.query(&[("label_selector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListFloatingIpsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the description or [Labels](#labels) of a [Floating IP](#floating-ips). Note that when updating [Labels](#labels), the [Floating IPs](#floating-ips) current set of [Labels](#labels) will be replaced with the [Labels](#labels) provided with the request. So, for example, if you want to add a new [Label](#labels), you have to provide all existing [Labels](#labels) plus the new [Label](#labels) in the request body.
pub async fn replace_floating_ip(
    configuration: &configuration::Configuration,
    params: ReplaceFloatingIpParams,
) -> Result<models::ReplaceFloatingIpResponse, Error<ReplaceFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let replace_floating_ip_request = params.replace_floating_ip_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}",
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
    local_var_req_builder = local_var_req_builder.json(&replace_floating_ip_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unassigns a [Floating IP](#floating-ips).  Results in the IP being unreachable. Can be assigned to another resource again.
pub async fn unassign_floating_ip(
    configuration: &configuration::Configuration,
    params: UnassignFloatingIpParams,
) -> Result<models::UnassignFloatingIpResponse, Error<UnassignFloatingIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/floating_ips/{id}/actions/unassign",
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UnassignFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
