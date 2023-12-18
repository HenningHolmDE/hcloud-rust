/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.19.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`assign_floating_ip_to_server`]
#[derive(Clone, Debug, Default)]
pub struct AssignFloatingIpToServerParams {
    /// ID of the Floating IP
    pub id: i64,
    pub assign_floating_ip_to_server_request:
        Option<crate::models::AssignFloatingIpToServerRequest>,
}

/// struct for passing parameters to the method [`change_floating_ip_protection`]
#[derive(Clone, Debug, Default)]
pub struct ChangeFloatingIpProtectionParams {
    /// ID of the Floating IP
    pub id: i64,
    pub change_floating_ip_protection_request:
        Option<crate::models::ChangeFloatingIpProtectionRequest>,
}

/// struct for passing parameters to the method [`change_reverse_dns_entry_for_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct ChangeReverseDnsEntryForFloatingIpParams {
    /// ID of the Floating IP
    pub id: i64,
    /// Select the IP address for which to change the DNS entry by passing `ip`. For a Floating IP of type `ipv4` this must exactly match the IP address of the Floating IP. For a Floating IP of type `ipv6` this must be a single IP within the IPv6 /64 range that belongs to this Floating IP. You can add up to 100 IPv6 reverse DNS entries.  The target hostname is set by passing `dns_ptr`.
    pub change_reverse_dns_entry_for_floating_ip_request:
        Option<crate::models::ChangeReverseDnsEntryForFloatingIpRequest>,
}

/// struct for passing parameters to the method [`create_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct CreateFloatingIpParams {
    /// The `type` argument is required while `home_location` and `server` are mutually exclusive.
    pub create_floating_ip_request: Option<crate::models::CreateFloatingIpRequest>,
}

/// struct for passing parameters to the method [`delete_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFloatingIpParams {
    /// ID of the Floating IP
    pub id: i64,
}

/// struct for passing parameters to the method [`get_action_for_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct GetActionForFloatingIpParams {
    /// ID of the Floating IP
    pub id: i64,
    /// ID of the Action
    pub action_id: i64,
}

/// struct for passing parameters to the method [`get_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct GetFloatingIpParams {
    /// ID of the Floating IP
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
    /// ID of the Floating IP
    pub id: i64,
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used multiple times, the response will contain only Actions with specified statuses
    pub status: Option<String>,
    /// Page to load.
    pub page: Option<i64>,
    /// Items to load per page.
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`list_floating_ip_actions`]
#[derive(Clone, Debug, Default)]
pub struct ListFloatingIpActionsParams {
    /// Can be used multiple times, the response will contain only Actions with specified IDs.
    pub id: Option<i64>,
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used multiple times, the response will contain only Actions with specified statuses
    pub status: Option<String>,
    /// Page to load.
    pub page: Option<i64>,
    /// Items to load per page.
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`list_floating_ips`]
#[derive(Clone, Debug, Default)]
pub struct ListFloatingIpsParams {
    /// Can be used to filter Floating IPs by their name. The response will only contain the Floating IP matching the specified name.
    pub name: Option<String>,
    /// Can be used to filter Floating IPs by labels. The response will only contain Floating IPs matching the label selector.
    pub label_selector: Option<String>,
    /// Can be used multiple times. Choices id id:asc id:desc created created:asc created:desc
    pub sort: Option<String>,
    /// Page to load.
    pub page: Option<i64>,
    /// Items to load per page.
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`replace_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct ReplaceFloatingIpParams {
    /// ID of the Floating IP
    pub id: i64,
    pub replace_floating_ip_request: Option<crate::models::ReplaceFloatingIpRequest>,
}

/// struct for passing parameters to the method [`unassign_floating_ip`]
#[derive(Clone, Debug, Default)]
pub struct UnassignFloatingIpParams {
    /// ID of the Floating IP
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

/// struct for typed errors of method [`change_reverse_dns_entry_for_floating_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeReverseDnsEntryForFloatingIpError {
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

/// Assigns a Floating IP to a Server.
pub async fn assign_floating_ip_to_server(
    configuration: &configuration::Configuration,
    params: AssignFloatingIpToServerParams,
) -> Result<crate::models::AssignFloatingIpToServerResponse, Error<AssignFloatingIpToServerError>> {
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

/// Changes the protection configuration of the Floating IP.
pub async fn change_floating_ip_protection(
    configuration: &configuration::Configuration,
    params: ChangeFloatingIpProtectionParams,
) -> Result<crate::models::ChangeFloatingIpProtectionResponse, Error<ChangeFloatingIpProtectionError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let change_floating_ip_protection_request = params.change_floating_ip_protection_request;

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
    local_var_req_builder = local_var_req_builder.json(&change_floating_ip_protection_request);

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

/// Changes the hostname that will appear when getting the hostname belonging to this Floating IP.
pub async fn change_reverse_dns_entry_for_floating_ip(
    configuration: &configuration::Configuration,
    params: ChangeReverseDnsEntryForFloatingIpParams,
) -> Result<
    crate::models::ChangeReverseDnsEntryForFloatingIpResponse,
    Error<ChangeReverseDnsEntryForFloatingIpError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let change_reverse_dns_entry_for_floating_ip_request =
        params.change_reverse_dns_entry_for_floating_ip_request;

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
    local_var_req_builder =
        local_var_req_builder.json(&change_reverse_dns_entry_for_floating_ip_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeReverseDnsEntryForFloatingIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new Floating IP assigned to a Server. If you want to create a Floating IP that is not bound to a Server, you need to provide the `home_location` key instead of `server`. This can be either the ID or the name of the Location this IP shall be created in. Note that a Floating IP can be assigned to a Server in any Location later on. For optimal routing it is advised to use the Floating IP in the same Location it was created in.
pub async fn create_floating_ip(
    configuration: &configuration::Configuration,
    params: CreateFloatingIpParams,
) -> Result<crate::models::CreateFloatingIpResponse, Error<CreateFloatingIpError>> {
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

/// Deletes a Floating IP. If it is currently assigned to a Server it will automatically get unassigned.
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

/// Returns a specific Action object for a Floating IP.
pub async fn get_action_for_floating_ip(
    configuration: &configuration::Configuration,
    params: GetActionForFloatingIpParams,
) -> Result<crate::models::GetActionForFloatingIpResponse, Error<GetActionForFloatingIpError>> {
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

/// Returns a specific Floating IP object.
pub async fn get_floating_ip(
    configuration: &configuration::Configuration,
    params: GetFloatingIpParams,
) -> Result<crate::models::GetFloatingIpResponse, Error<GetFloatingIpError>> {
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

/// Returns a specific Action object.
pub async fn get_floating_ip_action(
    configuration: &configuration::Configuration,
    params: GetFloatingIpActionParams,
) -> Result<crate::models::GetActionResponse, Error<GetFloatingIpActionError>> {
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

/// Returns all Action objects for a Floating IP. You can sort the results by using the `sort` URI parameter, and filter them with the `status` parameter.
pub async fn list_actions_for_floating_ip(
    configuration: &configuration::Configuration,
    params: ListActionsForFloatingIpParams,
) -> Result<crate::models::ListActionsForFloatingIpResponse, Error<ListActionsForFloatingIpError>> {
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

/// Returns all Action objects. You can `sort` the results by using the sort URI parameter, and filter them with the `status` and `id` parameter.
pub async fn list_floating_ip_actions(
    configuration: &configuration::Configuration,
    params: ListFloatingIpActionsParams,
) -> Result<crate::models::ListActionsResponse, Error<ListFloatingIpActionsError>> {
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

/// Returns all Floating IP objects.
pub async fn list_floating_ips(
    configuration: &configuration::Configuration,
    params: ListFloatingIpsParams,
) -> Result<crate::models::ListFloatingIpsResponse, Error<ListFloatingIpsError>> {
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

/// Updates the description or labels of a Floating IP. Also note that when updating labels, the Floating IP’s current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body.
pub async fn replace_floating_ip(
    configuration: &configuration::Configuration,
    params: ReplaceFloatingIpParams,
) -> Result<crate::models::ReplaceFloatingIpResponse, Error<ReplaceFloatingIpError>> {
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

/// Unassigns a Floating IP, resulting in it being unreachable. You may assign it to a Server again at a later time.
pub async fn unassign_floating_ip(
    configuration: &configuration::Configuration,
    params: UnassignFloatingIpParams,
) -> Result<crate::models::UnassignFloatingIpResponse, Error<UnassignFloatingIpError>> {
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
