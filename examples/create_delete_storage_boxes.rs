use hcloud::apis::storage_boxes_api;
use hcloud::models;
use hcloud::{apis::configuration::Configuration, models::CreateStorageBoxResponse};
use rand::prelude::*;
use std::{env, thread, time};

// type and location to be used for creating the storage boxes
const STORAGE_BOX_TYPE: &str = "bx11";
const STORAGE_BOX_LOCATION: &str = "fsn1";

// number of storage boxes to create during the example,
// as storage boxes have a low resource limit, we will only create one box here
const NUMBER_OF_STORAGE_BOXES: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), String> {
    // use API token from command line
    let api_token = env::args()
        .nth(1)
        .ok_or("Please provide API token as command line parameter.")?;

    // set up basic configuration using provided API token
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(api_token);

    // create a random password to be used for the storage boxes
    // (the prefix ensures all character classes to be present)
    let password = format!(
        "C!0ud{}",
        rand::rng()
            .sample_iter(rand::distr::Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>()
    );

    // create requested number of storage boxes and store responses
    println!(
        "Creating {} example storage boxes...",
        NUMBER_OF_STORAGE_BOXES
    );
    let mut create_responses = Vec::new();
    for _ in 0..NUMBER_OF_STORAGE_BOXES {
        let mut name = "example-".to_string();
        name.push_str(
            // append random postfix
            &rand::rng()
                .sample_iter(rand::distr::Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
        );
        let name = format!(
            "example-{}",
            // append random postfix
            rand::rng()
                .sample_iter(rand::distr::Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>()
        );
        println!(" Creating storage box \"{}\"...", name);
        let params = storage_boxes_api::CreateStorageBoxParams {
            create_storage_box_request: models::CreateStorageBoxRequest {
                location: STORAGE_BOX_LOCATION.to_owned(),
                name,
                password: password.clone(),
                storage_box_type: STORAGE_BOX_TYPE.to_owned(),
                ..Default::default()
            },
        };
        let response = storage_boxes_api::create_storage_box(&configuration, params)
            .await
            .map_err(|err| format!("API call to create_storage_box failed: {:?}", err))?;

        create_responses.push(response);
    }
    println!();

    // collect IDs of create storage box actions
    let create_action_ids = create_responses
        .iter()
        .map(|r| r.action.id)
        .collect::<Vec<_>>();

    println!("Wait for storage boxes to be ready by polling corresponding actions...");
    wait_for_completion_of_actions(&configuration, &create_action_ids).await?;
    println!("All actions have finished. The storage boxes should be available now!");
    println!();

    // extract storage box IDs from created boxes
    let created_storage_box_ids = create_responses
        .iter()
        .map(|r| r.storage_box.id)
        .collect::<Vec<_>>();

    // get list of all existing storage boxes from storage_boxes API
    // Note: This only requests the first page (max 25) of entries,
    //       see `list_isos.rs` for an example of using pagination.
    let storage_boxes = storage_boxes_api::list_storage_boxes(&configuration, Default::default())
        .await
        .map_err(|err| format!("API call to list_storage_boxes failed: {:?}", err))?
        .storage_boxes;

    println!("Listing {} storage box(es):", storage_boxes.len());
    for storage_box in storage_boxes {
        println!(
            " {}id: {}, name: {}, server: {}",
            if created_storage_box_ids.contains(&storage_box.id) {
                "*"
            } else {
                ""
            },
            storage_box.id,
            storage_box.name,
            storage_box.server.as_deref().unwrap_or("NONE")
        );
    }
    println!(" *: Created by this example.");
    println!();

    println!("Deleting storage boxes...");
    // Collect IDs of delete server actions to wait for completion
    let mut delete_action_ids = Vec::new();
    for CreateStorageBoxResponse { storage_box, .. } in &create_responses {
        println!(" Deleting storage box {}...", storage_box.name);
        let params = storage_boxes_api::DeleteStorageBoxParams { id: storage_box.id };
        let action = storage_boxes_api::delete_storage_box(&configuration, params)
            .await
            .map_err(|err| format!("API call to delete_storage_box failed: {:?}", err))?
            .action;
        delete_action_ids.push(action.id);
    }
    println!();

    println!("Wait for storage boxes to be deleted by polling corresponding actions...");
    wait_for_completion_of_actions(&configuration, &delete_action_ids).await?;
    println!("All actions have finished. The storage boxes should be deleted now!");

    Ok(())
}

async fn wait_for_completion_of_actions(
    configuration: &Configuration,
    action_ids: &[i64],
) -> Result<(), String> {
    // We use the `storage_boxes_api::list_storage_box_actions` for this to only have one API call per iteration.
    loop {
        let mut any_running = false;
        let params = storage_boxes_api::ListStorageBoxActionsParams {
            id: Some(action_ids.to_owned()),
            ..Default::default()
        };
        let actions = storage_boxes_api::list_storage_box_actions(configuration, params)
            .await
            .map_err(|err| format!("API call to list_storage_box_actions failed: {:?}", err))?
            .actions;

        println!(" Waiting for the following actions to be completed:");
        for action in actions {
            println!(
                "  id: {}, command: {}, status: {:?}, progress: {}",
                action.id, action.command, action.status, action.progress
            );
            if action.status == models::action::Status::Running {
                any_running = true;
            }
        }
        if !any_running {
            break;
        }
        println!("Some actions are still running, let's wait some time and check again...");
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
