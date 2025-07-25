/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 18354c8
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`create_certificate`]
#[derive(Clone, Debug, Default)]
pub struct CreateCertificateParams {
    pub create_certificate_request: Option<models::CreateCertificateRequest>,
}

/// struct for passing parameters to the method [`delete_certificate`]
#[derive(Clone, Debug, Default)]
pub struct DeleteCertificateParams {
    /// ID of the Certificate.
    pub id: i64,
}

/// struct for passing parameters to the method [`get_action_for_certificate`]
#[derive(Clone, Debug, Default)]
pub struct GetActionForCertificateParams {
    /// ID of the Certificate.
    pub id: i64,
    /// ID of the Action.
    pub action_id: i64,
}

/// struct for passing parameters to the method [`get_certificate`]
#[derive(Clone, Debug, Default)]
pub struct GetCertificateParams {
    /// ID of the Certificate.
    pub id: i64,
}

/// struct for passing parameters to the method [`get_certificate_action`]
#[derive(Clone, Debug, Default)]
pub struct GetCertificateActionParams {
    /// ID of the Action
    pub id: i64,
}

/// struct for passing parameters to the method [`list_actions_for_certificate`]
#[derive(Clone, Debug, Default)]
pub struct ListActionsForCertificateParams {
    /// ID of the Certificate
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

/// struct for passing parameters to the method [`list_certificate_actions`]
#[derive(Clone, Debug, Default)]
pub struct ListCertificateActionsParams {
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

/// struct for passing parameters to the method [`list_certificates`]
#[derive(Clone, Debug, Default)]
pub struct ListCertificatesParams {
    /// Sort resources by field and direction. Can be used multiple times. For more information, see \"[Sorting](#sorting)\".
    pub sort: Option<String>,
    /// Filter resources by their name. The response will only contain the resources matching exactly the specified name.
    pub name: Option<String>,
    /// Filter resources by labels. The response will only contain resources matching the label selector. For more information, see \"[Label Selector](#label-selector)\".
    pub label_selector: Option<String>,
    /// Filter resources by type. Can be used multiple times. The response will only contain the resources with the specified type.
    pub r#type: Option<String>,
    /// Page number to return. For more information, see \"[Pagination](#pagination)\".
    pub page: Option<i64>,
    /// Maximum number of entries returned per page. For more information, see \"[Pagination](#pagination)\".
    pub per_page: Option<i64>,
}

/// struct for passing parameters to the method [`replace_certificate`]
#[derive(Clone, Debug, Default)]
pub struct ReplaceCertificateParams {
    /// ID of the Certificate.
    pub id: i64,
    pub replace_certificate_request: Option<models::ReplaceCertificateRequest>,
}

/// struct for passing parameters to the method [`retry_issuance_or_renewal`]
#[derive(Clone, Debug, Default)]
pub struct RetryIssuanceOrRenewalParams {
    /// ID of the Certificate.
    pub id: i64,
}

/// struct for typed errors of method [`create_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_action_for_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionForCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_certificate_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCertificateActionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_actions_for_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionsForCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_certificate_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCertificateActionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_certificates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCertificatesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retry_issuance_or_renewal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetryIssuanceOrRenewalError {
    UnknownValue(serde_json::Value),
}

/// Creates a new Certificate.  The default type **uploaded** allows for uploading your existing `certificate` and `private_key` in PEM format. You have to monitor its expiration date and handle renewal yourself.  In contrast, type **managed** requests a new Certificate from *Let's Encrypt* for the specified `domain_names`. Only domains managed by *Hetzner DNS* are supported. We handle renewal and timely alert the project owner via email if problems occur.  For type `managed` Certificates the `action` key of the response contains the Action that allows for tracking the issuance process. For type `uploaded` Certificates the `action` is always null.
pub async fn create_certificate(
    configuration: &configuration::Configuration,
    params: CreateCertificateParams,
) -> Result<models::CreateCertificateResponse, Error<CreateCertificateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_certificate_request = params.create_certificate_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/certificates", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_certificate_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCertificateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a Certificate.
pub async fn delete_certificate(
    configuration: &configuration::Configuration,
    params: DeleteCertificateParams,
) -> Result<(), Error<DeleteCertificateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/{id}",
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
        let local_var_entity: Option<DeleteCertificateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a specific Action for a Certificate. Only type `managed` Certificates have Actions.
pub async fn get_action_for_certificate(
    configuration: &configuration::Configuration,
    params: GetActionForCertificateParams,
) -> Result<models::GetActionForCertificateResponse, Error<GetActionForCertificateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let action_id = params.action_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/{id}/actions/{action_id}",
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
        let local_var_entity: Option<GetActionForCertificateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a specific Certificate object.
pub async fn get_certificate(
    configuration: &configuration::Configuration,
    params: GetCertificateParams,
) -> Result<models::GetCertificateResponse, Error<GetCertificateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/{id}",
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
        let local_var_entity: Option<GetCertificateError> =
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
pub async fn get_certificate_action(
    configuration: &configuration::Configuration,
    params: GetCertificateActionParams,
) -> Result<models::GetActionResponse, Error<GetCertificateActionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/actions/{id}",
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
        let local_var_entity: Option<GetCertificateActionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Action objects for a Certificate. You can sort the results by using the `sort` URI parameter, and filter them with the `status` parameter.  Only type `managed` Certificates can have Actions. For type `uploaded` Certificates the `actions` key will always contain an empty array.
pub async fn list_actions_for_certificate(
    configuration: &configuration::Configuration,
    params: ListActionsForCertificateParams,
) -> Result<models::ListActionsForCertificateResponse, Error<ListActionsForCertificateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/{id}/actions",
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
        let local_var_entity: Option<ListActionsForCertificateError> =
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
pub async fn list_certificate_actions(
    configuration: &configuration::Configuration,
    params: ListCertificateActionsParams,
) -> Result<models::ListActionsResponse, Error<ListCertificateActionsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let sort = params.sort;
    let status = params.status;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/certificates/actions", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListCertificateActionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all Certificate objects.
pub async fn list_certificates(
    configuration: &configuration::Configuration,
    params: ListCertificatesParams,
) -> Result<models::ListCertificatesResponse, Error<ListCertificatesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    let name = params.name;
    let label_selector = params.label_selector;
    let r#type = params.r#type;
    let page = params.page;
    let per_page = params.per_page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/certificates", local_var_configuration.base_path);
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
    if let Some(ref local_var_str) = r#type {
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
        let local_var_entity: Option<ListCertificatesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the Certificate properties.  Note: if the Certificate object changes during the request, the response will be a “conflict” error.
pub async fn replace_certificate(
    configuration: &configuration::Configuration,
    params: ReplaceCertificateParams,
) -> Result<models::ReplaceCertificateResponse, Error<ReplaceCertificateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let replace_certificate_request = params.replace_certificate_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/{id}",
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
    local_var_req_builder = local_var_req_builder.json(&replace_certificate_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceCertificateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retry a failed Certificate issuance or renewal.  Only applicable if the type of the Certificate is `managed` and the issuance or renewal status is `failed`.  #### Call specific error codes  | Code                                                    | Description                                                               | |---------------------------------------------------------|---------------------------------------------------------------------------| | `caa_record_does_not_allow_ca`                          | CAA record does not allow certificate authority                           | | `ca_dns_validation_failed`                              | Certificate Authority: DNS validation failed                              | | `ca_too_many_authorizations_failed_recently`            | Certificate Authority: Too many authorizations failed recently            | | `ca_too_many_certificates_issued_for_registered_domain` | Certificate Authority: Too many certificates issued for registered domain | | `ca_too_many_duplicate_certificates`                    | Certificate Authority: Too many duplicate certificates                    | | `could_not_verify_domain_delegated_to_zone`             | Could not verify domain delegated to zone                                 | | `dns_zone_not_found`                                    | DNS zone not found                                                        | | `dns_zone_is_secondary_zone`                            | DNS zone is a secondary zone                                              |
pub async fn retry_issuance_or_renewal(
    configuration: &configuration::Configuration,
    params: RetryIssuanceOrRenewalParams,
) -> Result<models::RetryIssuanceOrRenewalResponse, Error<RetryIssuanceOrRenewalError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/certificates/{id}/actions/retry",
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
        let local_var_entity: Option<RetryIssuanceOrRenewalError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
