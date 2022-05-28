/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.9.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`add_route_to_network`]
#[derive(Clone, Debug, Default)]
pub struct AddRouteToNetworkParams {
    /// ID of the Network
    pub id: i32,
    pub body: Option<crate::models::Route>,
}

/// struct for passing parameters to the method [`add_subnet_to_network`]
#[derive(Clone, Debug, Default)]
pub struct AddSubnetToNetworkParams {
    /// ID of the Network
    pub id: i32,
    pub body: Option<crate::models::Subnet>,
}

/// struct for passing parameters to the method [`change_ip_range_of_network`]
#[derive(Clone, Debug, Default)]
pub struct ChangeIpRangeOfNetworkParams {
    /// ID of the Network
    pub id: i32,
    pub change_ip_range_of_network_request: Option<crate::models::ChangeIpRangeOfNetworkRequest>,
}

/// struct for passing parameters to the method [`change_network_protection`]
#[derive(Clone, Debug, Default)]
pub struct ChangeNetworkProtectionParams {
    /// ID of the Network
    pub id: i32,
    pub change_network_protection_request: Option<crate::models::ChangeNetworkProtectionRequest>,
}

/// struct for passing parameters to the method [`create_network`]
#[derive(Clone, Debug, Default)]
pub struct CreateNetworkParams {
    pub create_network_request: Option<crate::models::CreateNetworkRequest>,
}

/// struct for passing parameters to the method [`delete_network`]
#[derive(Clone, Debug, Default)]
pub struct DeleteNetworkParams {
    /// ID of the network
    pub id: i32,
}

/// struct for passing parameters to the method [`delete_route_from_network`]
#[derive(Clone, Debug, Default)]
pub struct DeleteRouteFromNetworkParams {
    /// ID of the Network
    pub id: i32,
    pub body: Option<crate::models::Route>,
}

/// struct for passing parameters to the method [`delete_subnet_from_network`]
#[derive(Clone, Debug, Default)]
pub struct DeleteSubnetFromNetworkParams {
    /// ID of the Network
    pub id: i32,
    pub delete_subnet_from_network_request: Option<crate::models::DeleteSubnetFromNetworkRequest>,
}

/// struct for passing parameters to the method [`get_action_for_network`]
#[derive(Clone, Debug, Default)]
pub struct GetActionForNetworkParams {
    /// ID of the Network
    pub id: i32,
    /// ID of the Action
    pub action_id: i32,
}

/// struct for passing parameters to the method [`get_network`]
#[derive(Clone, Debug, Default)]
pub struct GetNetworkParams {
    /// ID of the network
    pub id: i32,
}

/// struct for passing parameters to the method [`list_actions_for_network`]
#[derive(Clone, Debug, Default)]
pub struct ListActionsForNetworkParams {
    /// ID of the Network
    pub id: i32,
    /// Can be used multiple times.
    pub sort: Option<String>,
    /// Can be used multiple times, the response will contain only Actions with specified statuses
    pub status: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>,
}

/// struct for passing parameters to the method [`list_networks`]
#[derive(Clone, Debug, Default)]
pub struct ListNetworksParams {
    /// Can be used to filter networks by their name. The response will only contain the networks matching the specified name.
    pub name: Option<String>,
    /// Can be used to filter networks by labels. The response will only contain networks with a matching label selector pattern.
    pub label_selector: Option<String>,
    /// Specifies the page to fetch. The number of the first page is 1
    pub page: Option<i32>,
    /// Specifies the number of items returned per page. The default value is 25, the maximum value is 50 except otherwise specified in the documentation.
    pub per_page: Option<i32>,
}

/// struct for passing parameters to the method [`replace_network`]
#[derive(Clone, Debug, Default)]
pub struct ReplaceNetworkParams {
    /// ID of the network
    pub id: i32,
    pub replace_network_request: Option<crate::models::ReplaceNetworkRequest>,
}

/// struct for typed errors of method [`add_route_to_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddRouteToNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_subnet_to_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSubnetToNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_ip_range_of_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeIpRangeOfNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`change_network_protection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeNetworkProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_route_from_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRouteFromNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_subnet_from_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSubnetFromNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_action_for_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_actions_for_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForNetworkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_networks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNetworksError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceNetworkError {
    UnknownValue(serde_json::Value),
}

/// Adds a route entry to a Network.  Note: if the Network object changes during the request, the response will be a “conflict” error.
pub async fn add_route_to_network(
    configuration: &configuration::Configuration,
    params: AddRouteToNetworkParams,
) -> Result<crate::models::AddRouteToNetworkResponse, Error<AddRouteToNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/add_route",
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
        let local_var_entity: Option<AddRouteToNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds a new subnet object to the Network. If you do not specify an `ip_range` for the subnet we will automatically pick the first available /24 range for you if possible.  Note: if the parent Network object changes during the request, the response will be a “conflict” error.
pub async fn add_subnet_to_network(
    configuration: &configuration::Configuration,
    params: AddSubnetToNetworkParams,
) -> Result<crate::models::AddSubnetToNetworkResponse, Error<AddSubnetToNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/add_subnet",
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
        let local_var_entity: Option<AddSubnetToNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the IP range of a Network. IP ranges can only be extended and never shrunk. You can only add IPs at the end of an existing IP range. This means that the IP part of your existing range must stay the same and you can only change its netmask.  For example if you have a range `10.0.0.0/16` you want to extend then your new range must also start with the IP `10.0.0.0`. Your CIDR netmask `/16` may change to a number that is smaller than `16` thereby increasing the IP range. So valid entries would be `10.0.0.0/15`, `10.0.0.0/14`, `10.0.0.0/13` and so on.  After changing the IP range you will have to adjust the routes on your connected Servers by either rebooting them or manually changing the routes to your private Network interface.  Note: if the Network object changes during the request, the response will be a “conflict” error.
pub async fn change_ip_range_of_network(
    configuration: &configuration::Configuration,
    params: ChangeIpRangeOfNetworkParams,
) -> Result<crate::models::ChangeIpRangeOfNetworkResponse, Error<ChangeIpRangeOfNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let change_ip_range_of_network_request = params.change_ip_range_of_network_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/change_ip_range",
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
    local_var_req_builder = local_var_req_builder.json(&change_ip_range_of_network_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeIpRangeOfNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Changes the protection configuration of a Network.  Note: if the Network object changes during the request, the response will be a “conflict” error.
pub async fn change_network_protection(
    configuration: &configuration::Configuration,
    params: ChangeNetworkProtectionParams,
) -> Result<crate::models::ChangeNetworkProtectionResponse, Error<ChangeNetworkProtectionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let change_network_protection_request = params.change_network_protection_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/change_protection",
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
    local_var_req_builder = local_var_req_builder.json(&change_network_protection_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeNetworkProtectionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a network with the specified `ip_range`.  You may specify one or more `subnets`. You can also add more Subnets later by using the [add subnet action](https://docs.hetzner.cloud/#network-actions-add-a-subnet-to-a-network). If you do not specify an `ip_range` in the subnet we will automatically pick the first available /24 range for you.  You may specify one or more routes in `routes`. You can also add more routes later by using the [add route action](https://docs.hetzner.cloud/#network-actions-add-a-route-to-a-network).
pub async fn create_network(
    configuration: &configuration::Configuration,
    params: CreateNetworkParams,
) -> Result<crate::models::CreateNetworkResponse, Error<CreateNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_network_request = params.create_network_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/networks", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_network_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a network. If there are Servers attached they will be detached in the background.  Note: if the network object changes during the request, the response will be a “conflict” error.
pub async fn delete_network(
    configuration: &configuration::Configuration,
    params: DeleteNetworkParams,
) -> Result<(), Error<DeleteNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}",
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
        let local_var_entity: Option<DeleteNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a route entry from a Network.  Note: if the Network object changes during the request, the response will be a “conflict” error.
pub async fn delete_route_from_network(
    configuration: &configuration::Configuration,
    params: DeleteRouteFromNetworkParams,
) -> Result<crate::models::DeleteRouteFromNetworkResponse, Error<DeleteRouteFromNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/delete_route",
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
        let local_var_entity: Option<DeleteRouteFromNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a single subnet entry from a Network. You cannot delete subnets which still have Servers attached. If you have Servers attached you first need to detach all Servers that use IPs from this subnet before you can delete the subnet.  Note: if the Network object changes during the request, the response will be a “conflict” error.
pub async fn delete_subnet_from_network(
    configuration: &configuration::Configuration,
    params: DeleteSubnetFromNetworkParams,
) -> Result<crate::models::DeleteSubnetFromNetworkResponse, Error<DeleteSubnetFromNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let delete_subnet_from_network_request = params.delete_subnet_from_network_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/delete_subnet",
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
    local_var_req_builder = local_var_req_builder.json(&delete_subnet_from_network_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteSubnetFromNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific Action for a Network.
pub async fn get_action_for_network(
    configuration: &configuration::Configuration,
    params: GetActionForNetworkParams,
) -> Result<crate::models::GetActionForNetworkResponse, Error<GetActionForNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let action_id = params.action_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions/{action_id}",
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
        let local_var_entity: Option<GetActionForNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a specific network object.
pub async fn get_network(
    configuration: &configuration::Configuration,
    params: GetNetworkParams,
) -> Result<crate::models::GetNetworkResponse, Error<GetNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}",
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
        let local_var_entity: Option<GetNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Action objects for a Network. You can sort the results by using the `sort` URI parameter, and filter them with the `status` parameter.
pub async fn list_actions_for_network(
    configuration: &configuration::Configuration,
    params: ListActionsForNetworkParams,
) -> Result<crate::models::ListActionsForNetworkResponse, Error<ListActionsForNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}/actions",
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
        let local_var_entity: Option<ListActionsForNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets all existing networks that you have available.
pub async fn list_networks(
    configuration: &configuration::Configuration,
    params: ListNetworksParams,
) -> Result<crate::models::ListNetworksResponse, Error<ListNetworksError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let label_selector = params.label_selector;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/networks", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListNetworksError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the network properties.  Note that when updating labels, the network’s current set of labels will be replaced with the labels provided in the request body. So, for example, if you want to add a new label, you have to provide all existing labels plus the new label in the request body.  Note: if the network object changes during the request, the response will be a “conflict” error.
pub async fn replace_network(
    configuration: &configuration::Configuration,
    params: ReplaceNetworkParams,
) -> Result<crate::models::ReplaceNetworkResponse, Error<ReplaceNetworkError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let replace_network_request = params.replace_network_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/networks/{id}",
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
    local_var_req_builder = local_var_req_builder.json(&replace_network_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceNetworkError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
