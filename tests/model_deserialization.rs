//! Regression tests for backwards-compatible deserialization of Hetzner responses.

use hcloud::models;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

fn model_json<T: Serialize + Default>() -> Value {
    serde_json::to_value(T::default()).expect("serialize model fixture")
}

fn assert_missing_is_none<T>(field: &str)
where
    T: Serialize + DeserializeOwned + Default,
{
    let mut value = model_json::<T>();
    assert!(
        value.as_object_mut().unwrap().remove(field).is_some(),
        "field {field} is missing from the fixture"
    );
    let model: T = serde_json::from_value(value).expect("omitted nullable field must deserialize");
    assert_eq!(
        serde_json::to_value(model).unwrap().get(field),
        Some(&Value::Null),
        "omitted {field} should default to None"
    );
}

fn assert_value_accepted<T>(field: &str, value: Value)
where
    T: Serialize + DeserializeOwned + Default,
{
    let mut payload = model_json::<T>();
    payload[field] = value.clone();
    let model: T = serde_json::from_value(payload).expect("nullable field value must deserialize");
    assert_eq!(serde_json::to_value(model).unwrap().get(field), Some(&value));
}

#[test]
fn removed_deprecated_fields_default_to_none() {
    assert_missing_is_none::<models::Image>("deprecated");
    assert_missing_is_none::<models::LoadBalancerType>("deprecated");
    assert_missing_is_none::<models::ServerType>("deprecated");
}

#[test]
fn omitted_and_null_deprecation_objects_are_supported() {
    assert_missing_is_none::<models::LoadBalancerType>("deprecation");
    assert_missing_is_none::<models::Iso>("deprecation");
    assert_missing_is_none::<models::ServerTypeLocation>("deprecation");
    assert_missing_is_none::<models::StorageBoxType>("deprecation");

    assert_value_accepted::<models::LoadBalancerType>("deprecation", Value::Null);
    assert_value_accepted::<models::ServerTypeLocation>("deprecation", Value::Null);
    assert_value_accepted::<models::StorageBoxType>("deprecation", Value::Null);
    assert_value_accepted::<models::LoadBalancerType>(
        "deprecation",
        model_json::<models::DeprecationInfo>(),
    );
}

#[test]
fn legacy_deprecated_values_are_still_accepted() {
    let timestamp = json!("2026-11-02T00:00:00Z");
    assert_value_accepted::<models::Image>("deprecated", timestamp.clone());
    assert_value_accepted::<models::LoadBalancerType>("deprecated", timestamp);
    assert_value_accepted::<models::ServerType>("deprecated", json!(true));
    assert_value_accepted::<models::Image>("deprecated", Value::Null);
    assert_value_accepted::<models::ServerType>("deprecated", Value::Null);
}

#[test]
fn other_nullable_response_fields_can_be_omitted() {
    assert_missing_is_none::<models::Action>("finished");
    assert_missing_is_none::<models::Server>("image");
    assert_missing_is_none::<models::PrimaryIp>("assignee_id");
}

#[test]
fn nested_server_and_load_balancer_payloads_deserialize_without_deprecated() {
    let mut server = model_json::<models::Server>();
    server["server_type"]
        .as_object_mut()
        .unwrap()
        .remove("deprecated");
    server["image"] = model_json::<models::Image>();
    server["image"]
        .as_object_mut()
        .unwrap()
        .remove("deprecated");
    let parsed: models::Server = serde_json::from_value(server).unwrap();
    assert!(parsed.server_type.deprecated.is_none());
    assert!(parsed.image.unwrap().deprecated.is_none());

    let mut lb = model_json::<models::LoadBalancer>();
    lb["load_balancer_type"]
        .as_object_mut()
        .unwrap()
        .remove("deprecated");
    let parsed: models::LoadBalancer = serde_json::from_value(lb).unwrap();
    assert!(parsed.load_balancer_type.deprecated.is_none());
}

#[test]
fn unassigned_primary_ip_is_supported() {
    use models::primary_ip::AssigneeType;
    let parsed: AssigneeType = serde_json::from_value(json!("unassigned")).unwrap();
    assert_eq!(parsed, AssigneeType::Unassigned);
    assert_eq!(serde_json::to_value(parsed).unwrap(), json!("unassigned"));
}
