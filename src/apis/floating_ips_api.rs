/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

pub struct FloatingIpsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl FloatingIpsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> FloatingIpsApiClient {
        FloatingIpsApiClient {
            configuration,
        }
    }
}

/// struct for passing parameters to the method `assign_floating_ip_to_server`
#[derive(Clone, Debug, Default)]
pub struct AssignFloatingIpToServerParams {
    /// ID of the Floating IP
    pub id: String,
    pub assign_floating_ip_to_server_request: Option<crate::models::AssignFloatingIpToServerRequest>
}

/// struct for passing parameters to the method `change_floating_ip_protection`
#[derive(Clone, Debug, Default)]
pub struct ChangeFloatingIpProtectionParams {
    /// ID of the Floating IP
    pub id: String,
    pub change_floating_ip_protection_request: Option<crate::models::ChangeFloatingIpProtectionRequest>
}

/// struct for passing parameters to the method `change_reverse_dns_entry_for_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct ChangeReverseDnsEntryForFloatingIpParams {
    /// ID of the Floating IP
    pub id: String,
    pub change_reverse_dns_entry_for_floating_ip_request: Option<crate::models::ChangeReverseDnsEntryForFloatingIpRequest>
}

/// struct for passing parameters to the method `create_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct CreateFloatingIpParams {
    pub create_floating_ip_request: Option<crate::models::CreateFloatingIpRequest>
}

/// struct for passing parameters to the method `delete_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct DeleteFloatingIpParams {
    /// ID of the Floating IP
    pub id: String
}

/// struct for passing parameters to the method `get_action_for_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct GetActionForFloatingIpParams {
    /// ID of the Floating IP
    pub id: String,
    /// ID of the Action
    pub action_id: String
}

/// struct for passing parameters to the method `get_specific_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct GetSpecificFloatingIpParams {
    /// ID of the Floating IP
    pub id: String
}

/// struct for passing parameters to the method `list_actions_for_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct ListActionsForFloatingIpParams {
    /// ID of the Floating IP
    pub id: String,
    /// Can be used multiple times, the response will contain only Actions with specified statuses Choices: running success error
    pub status: Option<String>,
    /// Can be used multiple times Choices: id id:asc id:desc command command:asc command:desc status status:asc status:desc progress progress:asc progress:desc started started:asc started:desc finished finished:asc finished:desc
    pub sort: Option<String>
}

/// struct for passing parameters to the method `list_floating_ips`
#[derive(Clone, Debug, Default)]
pub struct ListFloatingIpsParams {
    /// Can be used multiple times. Choices: id id:asc id:desc created created:asc created:desc
    pub sort: Option<String>,
    /// Can be used to filter Floating IPs by labels. The response will only contain Floating IPs matching the label selector.
    pub label_selector: Option<String>,
    /// Can be used to filter Floating IPs by their name. The response will only contain the Floating IP matching the specified name.
    pub name: Option<String>
}

/// struct for passing parameters to the method `replace_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct ReplaceFloatingIpParams {
    /// ID of the Floating IP
    pub id: String,
    pub replace_floating_ip_request: Option<crate::models::ReplaceFloatingIpRequest>
}

/// struct for passing parameters to the method `unassign_floating_ip`
#[derive(Clone, Debug, Default)]
pub struct UnassignFloatingIpParams {
    /// ID of the Floating IP
    pub id: String
}


/// struct for typed errors of method `assign_floating_ip_to_server`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssignFloatingIpToServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `change_floating_ip_protection`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeFloatingIpProtectionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `change_reverse_dns_entry_for_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeReverseDnsEntryForFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_action_for_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_specific_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSpecificFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_actions_for_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_floating_ips`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFloatingIpsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceFloatingIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `unassign_floating_ip`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnassignFloatingIpError {
    UnknownValue(serde_json::Value),
}


pub trait FloatingIpsApi {
    fn assign_floating_ip_to_server(&self, params: AssignFloatingIpToServerParams) -> Result<crate::models::AssignFloatingIpToServerResponse, Error<AssignFloatingIpToServerError>>;
    fn change_floating_ip_protection(&self, params: ChangeFloatingIpProtectionParams) -> Result<crate::models::ChangeFloatingIpProtectionResponse, Error<ChangeFloatingIpProtectionError>>;
    fn change_reverse_dns_entry_for_floating_ip(&self, params: ChangeReverseDnsEntryForFloatingIpParams) -> Result<crate::models::ChangeReverseDnsEntryForFloatingIpResponse, Error<ChangeReverseDnsEntryForFloatingIpError>>;
    fn create_floating_ip(&self, params: CreateFloatingIpParams) -> Result<crate::models::CreateFloatingIpResponse, Error<CreateFloatingIpError>>;
    fn delete_floating_ip(&self, params: DeleteFloatingIpParams) -> Result<(), Error<DeleteFloatingIpError>>;
    fn get_action_for_floating_ip(&self, params: GetActionForFloatingIpParams) -> Result<crate::models::GetActionForFloatingIpResponse, Error<GetActionForFloatingIpError>>;
    fn get_specific_floating_ip(&self, params: GetSpecificFloatingIpParams) -> Result<crate::models::GetSpecificFloatingIpResponse, Error<GetSpecificFloatingIpError>>;
    fn list_actions_for_floating_ip(&self, params: ListActionsForFloatingIpParams) -> Result<crate::models::ListActionsForFloatingIpResponse, Error<ListActionsForFloatingIpError>>;
    fn list_floating_ips(&self, params: ListFloatingIpsParams) -> Result<crate::models::ListFloatingIpsResponse, Error<ListFloatingIpsError>>;
    fn replace_floating_ip(&self, params: ReplaceFloatingIpParams) -> Result<crate::models::ReplaceFloatingIpResponse, Error<ReplaceFloatingIpError>>;
    fn unassign_floating_ip(&self, params: UnassignFloatingIpParams) -> Result<crate::models::UnassignFloatingIpResponse, Error<UnassignFloatingIpError>>;
}

impl FloatingIpsApi for FloatingIpsApiClient {
    fn assign_floating_ip_to_server(&self, params: AssignFloatingIpToServerParams) -> Result<crate::models::AssignFloatingIpToServerResponse, Error<AssignFloatingIpToServerError>> {
        // unbox the parameters
        let id = params.id;
        let assign_floating_ip_to_server_request = params.assign_floating_ip_to_server_request;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}/actions/assign", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&assign_floating_ip_to_server_request);

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<AssignFloatingIpToServerError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn change_floating_ip_protection(&self, params: ChangeFloatingIpProtectionParams) -> Result<crate::models::ChangeFloatingIpProtectionResponse, Error<ChangeFloatingIpProtectionError>> {
        // unbox the parameters
        let id = params.id;
        let change_floating_ip_protection_request = params.change_floating_ip_protection_request;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}/actions/change_protection", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&change_floating_ip_protection_request);

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ChangeFloatingIpProtectionError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn change_reverse_dns_entry_for_floating_ip(&self, params: ChangeReverseDnsEntryForFloatingIpParams) -> Result<crate::models::ChangeReverseDnsEntryForFloatingIpResponse, Error<ChangeReverseDnsEntryForFloatingIpError>> {
        // unbox the parameters
        let id = params.id;
        let change_reverse_dns_entry_for_floating_ip_request = params.change_reverse_dns_entry_for_floating_ip_request;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}/actions/change_dns_ptr", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&change_reverse_dns_entry_for_floating_ip_request);

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ChangeReverseDnsEntryForFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn create_floating_ip(&self, params: CreateFloatingIpParams) -> Result<crate::models::CreateFloatingIpResponse, Error<CreateFloatingIpError>> {
        // unbox the parameters
        let create_floating_ip_request = params.create_floating_ip_request;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&create_floating_ip_request);

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<CreateFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn delete_floating_ip(&self, params: DeleteFloatingIpParams) -> Result<(), Error<DeleteFloatingIpError>> {
        // unbox the parameters
        let id = params.id;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            Ok(())
        } else {
            let entity: Option<DeleteFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn get_action_for_floating_ip(&self, params: GetActionForFloatingIpParams) -> Result<crate::models::GetActionForFloatingIpResponse, Error<GetActionForFloatingIpError>> {
        // unbox the parameters
        let id = params.id;
        let action_id = params.action_id;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}/actions/{action_id}", configuration.base_path, id=crate::apis::urlencode(id), action_id=crate::apis::urlencode(action_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetActionForFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn get_specific_floating_ip(&self, params: GetSpecificFloatingIpParams) -> Result<crate::models::GetSpecificFloatingIpResponse, Error<GetSpecificFloatingIpError>> {
        // unbox the parameters
        let id = params.id;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<GetSpecificFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn list_actions_for_floating_ip(&self, params: ListActionsForFloatingIpParams) -> Result<crate::models::ListActionsForFloatingIpResponse, Error<ListActionsForFloatingIpError>> {
        // unbox the parameters
        let id = params.id;
        let status = params.status;
        let sort = params.sort;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}/actions", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = status {
            req_builder = req_builder.query(&[("status", &s.to_string())]);
        }
        if let Some(ref s) = sort {
            req_builder = req_builder.query(&[("sort", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ListActionsForFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn list_floating_ips(&self, params: ListFloatingIpsParams) -> Result<crate::models::ListFloatingIpsResponse, Error<ListFloatingIpsError>> {
        // unbox the parameters
        let sort = params.sort;
        let label_selector = params.label_selector;
        let name = params.name;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = sort {
            req_builder = req_builder.query(&[("sort", &s.to_string())]);
        }
        if let Some(ref s) = label_selector {
            req_builder = req_builder.query(&[("label_selector", &s.to_string())]);
        }
        if let Some(ref s) = name {
            req_builder = req_builder.query(&[("name", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ListFloatingIpsError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn replace_floating_ip(&self, params: ReplaceFloatingIpParams) -> Result<crate::models::ReplaceFloatingIpResponse, Error<ReplaceFloatingIpError>> {
        // unbox the parameters
        let id = params.id;
        let replace_floating_ip_request = params.replace_floating_ip_request;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&replace_floating_ip_request);

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<ReplaceFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn unassign_floating_ip(&self, params: UnassignFloatingIpParams) -> Result<crate::models::UnassignFloatingIpResponse, Error<UnassignFloatingIpError>> {
        // unbox the parameters
        let id = params.id;

        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/floating_ips/{id}/actions/unassign", configuration.base_path, id=crate::apis::urlencode(id));
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            serde_json::from_str(&content).map_err(Error::from)
        } else {
            let entity: Option<UnassignFloatingIpError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

}
