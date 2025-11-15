use hcloud::apis::configuration::Configuration;
use hcloud::apis::{actions_api, servers_api, ssh_keys_api};
use hcloud::models;
use rand::prelude::*;
use std::{env, thread, time};

// server type and location to be used for creating the servers
const SERVER_TYPE: &str = "cx22";
const LOCATION: &str = "fsn1";
const IMAGE: &str = "ubuntu-22.04";

// number of servers to create during the example
const NUMBER_OF_SERVERS: u32 = 2;

// struct to track some information about our servers
struct ServerInfo {
    id: i64,
    name: String,
    ipv4: String,
    ipv6: String,
    action_id: i64,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // use API token from command line
    let api_token = env::args()
        .nth(1)
        .ok_or("Please provide API token as command line parameter.")?;

    // set up basic configuration using provided API token
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(api_token);

    // collect all available SSH keys to be added to the server
    // Note: This only requests the first page (max 25) of SSH keys,
    //       see `list_isos.rs` for an example of using pagination.
    let ssh_keys: Vec<String> = ssh_keys_api::list_ssh_keys(&configuration, Default::default())
        .await
        .map_err(|err| format!("API call to list_ssh_keys failed: {:?}", err))?
        .ssh_keys
        .into_iter()
        .map(|ssh_key| ssh_key.name)
        .collect();

    // store server info of created servers for querying
    // corresponding actions and finally deleting the servers
    let mut created_servers = Vec::new();

    println!("Creating {} example servers...", NUMBER_OF_SERVERS);
    for _ in 0..NUMBER_OF_SERVERS {
        let mut name = "example-server-".to_string();
        name.push_str(
            // append random postfix
            &rand::rng()
                .sample_iter(rand::distr::Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
        );
        println!(" Creating server \"{}\"...", name);
        let create_server_request = models::CreateServerRequest {
            name,
            server_type: SERVER_TYPE.to_string(),
            start_after_create: Some(true),
            image: IMAGE.to_string(),
            ssh_keys: Some(ssh_keys.clone()),
            location: Some(LOCATION.to_string()),
            public_net: Some(Box::new(models::CreateServerRequestPublicNet {
                enable_ipv4: Some(false),
                ..Default::default()
            })),
            ..Default::default()
        };

        // execute request and store server ID
        let params = servers_api::CreateServerParams {
            create_server_request,
        };
        let models::CreateServerResponse { action, server, .. } =
            servers_api::create_server(&configuration, params)
                .await
                .map_err(|err| format!("API call to create_server failed: {:?}", err))?;

        created_servers.push(ServerInfo {
            id: server.id,
            name: server.name,
            ipv4: server
                .public_net
                .ipv4
                .map(|ipv4| ipv4.ip)
                .unwrap_or("None".to_string()),
            ipv6: server
                .public_net
                .ipv6
                .map(|ipv6| ipv6.ip)
                .unwrap_or("None".to_string()),
            action_id: action.id,
        });
    }
    println!();
    println!("Server info of created servers:");
    for server_info in &created_servers {
        println!(
            " id: {}, name: {}, ipv4: {}, ipv6: {}, action_id: {}",
            server_info.id,
            server_info.name,
            server_info.ipv4,
            server_info.ipv6,
            server_info.action_id
        );
    }
    println!();

    // Collect IDs of create server actions to wait for completion
    let create_action_ids = created_servers
        .iter()
        .map(|server_info| server_info.action_id)
        .collect::<Vec<_>>();

    println!("Wait for servers to be ready by polling corresponding actions...");
    // We use the `servers_api::list_server_actions` for this to only have one API call per iteration.
    loop {
        let mut any_running = false;
        let params = servers_api::ListServerActionsParams {
            id: Some(create_action_ids.clone()),
            ..Default::default()
        };
        let actions = servers_api::list_server_actions(&configuration, params)
            .await
            .map_err(|err| format!("API call to get_multiple_actions failed: {:?}", err))?
            .actions;

        println!(" Actions for creating servers:");
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
    println!("All actions have finished. The servers should be up and running now!");
    println!();

    // maybe do something useful with the servers here?

    println!("Deleting servers...");
    // Collect IDs of delete server actions to wait for completion
    let mut delete_action_ids = Vec::new();
    for server_info in &created_servers {
        println!(" Deleting server {}...", server_info.name);
        let params = servers_api::DeleteServerParams { id: server_info.id };
        let action_option = servers_api::delete_server(&configuration, params)
            .await
            .map_err(|err| format!("API call to delete_server failed: {:?}", err))?
            .action;
        if let Some(action) = action_option {
            delete_action_ids.push(action.id);
        }
    }

    println!("Wait for servers to be deleted by polling corresponding actions...");
    // For a change, we use the more generic `actions_api::get_multiple_actions` for this.
    loop {
        let mut any_running = false;
        let params = actions_api::GetMultipleActionsParams {
            id: delete_action_ids.clone(),
        };
        let actions = actions_api::get_multiple_actions(&configuration, params)
            .await
            .map_err(|err| format!("API call to get_multiple_actions failed: {:?}", err))?
            .actions;

        println!(" Actions for deleting servers:");
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
    println!("All actions have finished. The servers should be deleted now!");

    Ok(())
}
