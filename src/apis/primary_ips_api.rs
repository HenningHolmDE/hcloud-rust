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

/// struct for passing parameters to the method [`assign_primary_ip_to_resource`]
#[derive(Clone, Debug, Default)]
pub struct AssignPrimaryIpToResourceParams {
    /// ID of the Primary IP
    pub id: i64,
    pub assign_primary_ip_to_resource_request:
        Option<crate::models::AssignPrimaryIpToResourceRequest>,
}

/// struct for passing parameters to the method [`change_primary_ip_protection`]
#[derive(Clone, Debug, Default)]
pub struct ChangePrimaryIpProtectionParams {
    /// ID of the Primary IP
    pub id: i64,
    pub change_primary_ip_protection_request:
        Option<crate::models::ChangePrimaryIpProtectionRequest>,
}

/// struct for passing parameters to the method [`change_reverse_dns_entry_for_primary_ip`]
#[derive(Clone, Debug, Default)]
pub struct ChangeReverseDnsEntryForPrimaryIpParams {
    /// ID of the Primary IP
    pub id: i64,
    /// Select the IP address for which to change the DNS entry by passing `ip`. For a Primary IP of type `ipv4` this must exactly match the IP address of the Primary IP. For a Primary IP of type `ipv6` this must be a single IP within the IPv6 /64 range that belongs to this Primary IP. You can add up to 100 IPv6 reverse DNS entries.  The target hostname is set by passing `dns_ptr`.
    pub change_reverse_dns_entry_for_primary_ip_request:
        Option<crate::models::ChangeReverseDnsEntryForPrimaryIpRequest>,
}

/// struct for passing parameters to the method [`create_primary_ip`]
#[derive(Clone, Debug, Default)]
pub struct CreatePrimaryIpParams {
    /// The `type` argument is required while `datacenter` and `assignee_id` are mutually exclusive.
    pub create_primary_ip_request: Option<crate::models::CreatePrimaryIpRequest>,
}

/// struct for passing parameters to the method [`delete_primary_ip`]
#[derive(Clone, Debug, Default)]
pub struct DeletePrimaryIpParams {
    /// ID of the resource
    pub id: i64,
}

/// struct for passing parameters to the method [`get_primary_ip`]
#[derive(Clone, Debug, Default)]
pub struct GetPrimaryIpParams {
    /// ID of the resource
    pub id: i64,
}

/// struct for passing parameters to the method [`get_primary_ip_action`]
#[derive(Clone, Debug, Default)]
pub struct GetPrimaryIpActionParams {
    /// ID of the Action
    pub id: i64,
}

/// struct for passing parameters to the method [`list_primary_ip_actions`]
#[derive(Clone, Debug, Default)]
pub struct ListPrimaryIpActionsParams {
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

/// struct for passing parameters to the method [`list_primary_ips`]
#[derive(Clone, Debug, Default)]
pub struct ListPrimaryIpsParams {
    /// Can be used to filter resources by their name. The response will only contain the resources matching the specified name
    pub name: Option<String>,
    /// Can be used to filter resources by labels. The response will only contain resources matching the label selector.
    pub label_selector: Option<String>,
    /// Can be used to filter resources by their ip. The response will only contain the resources matching the specified ip.
    pub ip: Option<String>,
    /// Page to load.
    pub page: Option<i64>,
    /// Items to load per page.
    pub per_page: Option<i64>,
    /// Can be used multiple times. Choices id id:asc id:desc created created:asc created:desc
    pub sort: Option<String>,
}

/// struct for passing parameters to the method [`replace_primary_ip`]
#[derive(Clone, Debug, Default)]
pub struct ReplacePrimaryIpParams {
    /// ID of the resource
    pub id: i64,
    pub replace_primary_ip_request: Option<crate::models::ReplacePrimaryIpRequest>,
}

/// struct for passing parameters to the method [`unassign_primary_ip_from_resource`]
#[derive(Clone, Debug, Default)]
pub struct UnassignPrimaryIpFromResourceParams {
    /// ID of the Primary IP
    pub id: i64,
}

/// struct for typed errors of method [`assign_primary_ip_to_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssignPrimaryIpToResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_primary_ip_protection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangePrimaryIpProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_reverse_dns_entry_for_primary_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeReverseDnsEntryForPrimaryIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_primary_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePrimaryIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_primary_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePrimaryIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_primary_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrimaryIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_primary_ip_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrimaryIpActionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_primary_ip_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrimaryIpActionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_primary_ips`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrimaryIpsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_primary_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplacePrimaryIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unassign_primary_ip_from_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnassignPrimaryIpFromResourceError {
    UnknownValue(serde_json::Value),
}

/// Assigns a Primary IP to a Server.  A Server can only have one Primary IP of type `ipv4` and one of type `ipv6` assigned. If you need more IPs use Floating IPs.  The Server must be powered off (status `off`) in order for this operation to succeed.  #### Call specific error codes  | Code                          | Description                                                   | |------------------------------ |-------------------------------------------------------------- | | `server_not_stopped`          | The server is running, but needs to be powered off            | | `primary_ip_already_assigned` | Primary ip is already assigned to a different server          | | `server_has_ipv4`             | The server already has an ipv4 address                        | | `server_has_ipv6`             | The server already has an ipv6 address                        |
pub async fn assign_primary_ip_to_resource(
    configuration: &configuration::Configuration,
    params: AssignPrimaryIpToResourceParams,
) -> Result<crate::models::AssignPrimaryIpToResourceResponse, Error<AssignPrimaryIpToResourceError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let assign_primary_ip_to_resource_request = params.assign_primary_ip_to_resource_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}/actions/assign",
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
    local_var_req_builder = local_var_req_builder.json(&assign_primary_ip_to_resource_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AssignPrimaryIpToResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the protection configuration of a Primary IP.  A Primary IP can only be delete protected if its `auto_delete` property is set to `false`.
pub async fn change_primary_ip_protection(
    configuration: &configuration::Configuration,
    params: ChangePrimaryIpProtectionParams,
) -> Result<crate::models::ChangePrimaryIpProtectionResponse, Error<ChangePrimaryIpProtectionError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let change_primary_ip_protection_request = params.change_primary_ip_protection_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}/actions/change_protection",
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
    local_var_req_builder = local_var_req_builder.json(&change_primary_ip_protection_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangePrimaryIpProtectionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the hostname that will appear when getting the hostname belonging to this Primary IP.
pub async fn change_reverse_dns_entry_for_primary_ip(
    configuration: &configuration::Configuration,
    params: ChangeReverseDnsEntryForPrimaryIpParams,
) -> Result<
    crate::models::ChangeReverseDnsEntryForPrimaryIpResponse,
    Error<ChangeReverseDnsEntryForPrimaryIpError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let change_reverse_dns_entry_for_primary_ip_request =
        params.change_reverse_dns_entry_for_primary_ip_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}/actions/change_dns_ptr",
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
        local_var_req_builder.json(&change_reverse_dns_entry_for_primary_ip_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeReverseDnsEntryForPrimaryIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new Primary IP, optionally assigned to a Server.  If you want to create a Primary IP that is not assigned to a Server, you need to provide the `datacenter` key instead of `assignee_id`. This can be either the ID or the name of the Datacenter this Primary IP shall be created in.  Note that a Primary IP can only be assigned to a Server in the same Datacenter later on.  #### Call specific error codes  | Code                          | Description                                                   | |------------------------------ |-------------------------------------------------------------- | | `server_not_stopped`          | The specified server is running, but needs to be powered off  | | `server_has_ipv4`             | The server already has an ipv4 address                        | | `server_has_ipv6`             | The server already has an ipv6 address                        |
pub async fn create_primary_ip(
    configuration: &configuration::Configuration,
    params: CreatePrimaryIpParams,
) -> Result<crate::models::CreatePrimaryIpResponse, Error<CreatePrimaryIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_primary_ip_request = params.create_primary_ip_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/primary_ips", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_primary_ip_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreatePrimaryIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a Primary IP.  The Primary IP may be assigned to a Server. In this case it is unassigned automatically. The Server must be powered off (status `off`) in order for this operation to succeed.
pub async fn delete_primary_ip(
    configuration: &configuration::Configuration,
    params: DeletePrimaryIpParams,
) -> Result<(), Error<DeletePrimaryIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}",
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
        let local_var_entity: Option<DeletePrimaryIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific Primary IP object.
pub async fn get_primary_ip(
    configuration: &configuration::Configuration,
    params: GetPrimaryIpParams,
) -> Result<crate::models::GetPrimaryIpResponse, Error<GetPrimaryIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}",
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
        let local_var_entity: Option<GetPrimaryIpError> =
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
pub async fn get_primary_ip_action(
    configuration: &configuration::Configuration,
    params: GetPrimaryIpActionParams,
) -> Result<crate::models::GetActionResponse, Error<GetPrimaryIpActionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/actions/{id}",
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
        let local_var_entity: Option<GetPrimaryIpActionError> =
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
pub async fn list_primary_ip_actions(
    configuration: &configuration::Configuration,
    params: ListPrimaryIpActionsParams,
) -> Result<crate::models::ListActionsResponse, Error<ListPrimaryIpActionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/primary_ips/actions", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListPrimaryIpActionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Primary IP objects.
pub async fn list_primary_ips(
    configuration: &configuration::Configuration,
    params: ListPrimaryIpsParams,
) -> Result<crate::models::ListPrimaryIpsResponse, Error<ListPrimaryIpsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let label_selector = params.label_selector;
    let ip = params.ip;
    let page = params.page;
    let per_page = params.per_page;
    let sort = params.sort;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/primary_ips", local_var_configuration.base_path);
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
    if let Some(ref local_var_str) = ip {
        local_var_req_builder = local_var_req_builder.query(&[("ip", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListPrimaryIpsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the Primary IP.  Note that when updating labels, the Primary IP's current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body.  If the Primary IP object changes during the request, the response will be a “conflict” error.
pub async fn replace_primary_ip(
    configuration: &configuration::Configuration,
    params: ReplacePrimaryIpParams,
) -> Result<crate::models::ReplacePrimaryIpResponse, Error<ReplacePrimaryIpError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let replace_primary_ip_request = params.replace_primary_ip_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}",
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
    local_var_req_builder = local_var_req_builder.json(&replace_primary_ip_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplacePrimaryIpError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unassigns a Primary IP from a Server.  The Server must be powered off (status `off`) in order for this operation to succeed.  Note that only Servers that have at least one network interface (public or private) attached can be powered on.  #### Call specific error codes  | Code                              | Description                                                   | |---------------------------------- |-------------------------------------------------------------- | | `server_not_stopped`              | The server is running, but needs to be powered off            | | `server_is_load_balancer_target`  | The server ipv4 address is a loadbalancer target              |
pub async fn unassign_primary_ip_from_resource(
    configuration: &configuration::Configuration,
    params: UnassignPrimaryIpFromResourceParams,
) -> Result<
    crate::models::UnassignPrimaryIpFromResourceResponse,
    Error<UnassignPrimaryIpFromResourceError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/primary_ips/{id}/actions/unassign",
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
        let local_var_entity: Option<UnassignPrimaryIpFromResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
