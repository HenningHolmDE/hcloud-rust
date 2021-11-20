use hcloud::apis::configuration::Configuration;
use hcloud::apis::{servers_api, ssh_keys_api};
use hcloud::models;
use rand::{distributions, thread_rng, Rng};
use std::{env, thread, time};

// server type and location to be used for creating the servers
const SERVER_TYPE: &str = "cx11";
const LOCATION: &str = "fsn1";
const IMAGE: &str = "ubuntu-18.04";

// number of servers to create during the example
const NUMBER_OF_SERVERS: u32 = 2;

// struct to track some information about our servers
struct ServerInfo {
    id: i32,
    name: String,
    ipv4: String,
    ipv6: String,
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
            &thread_rng()
                .sample_iter(distributions::Alphanumeric)
                .take(8)
                .collect::<String>(),
        );
        println!(" Creating server \"{}\"...", name);
        let request = models::CreateServerRequest {
            name,
            server_type: SERVER_TYPE.to_string(),
            start_after_create: Some(true),
            image: IMAGE.to_string(),
            ssh_keys: Some(ssh_keys.clone()),
            volumes: None,
            networks: None,
            firewalls: None,
            user_data: None,
            labels: None,
            automount: None,
            location: Some(LOCATION.to_string()),
            datacenter: None,
        };

        // execute request and store server ID
        let params = servers_api::CreateServerParams {
            create_server_request: Some(request),
        };
        let server = servers_api::create_server(&configuration, params)
            .await
            .map_err(|err| format!("API call to create_server failed: {:?}", err))?
            .server;

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
        });
    }
    println!();
    println!("Server info of created servers:");
    for server_info in &created_servers {
        println!(
            " id: {}, name: {}, ipv4: {}, ipv6: {}",
            server_info.id, server_info.name, server_info.ipv4, server_info.ipv6
        );
    }
    println!();

    println!("Wait for servers to be ready by polling corresponding actions...");
    loop {
        let mut any_running = false;
        for server_info in &created_servers {
            let params = servers_api::ListActionsForServerParams {
                id: server_info.id,
                ..Default::default()
            };
            let actions = servers_api::list_actions_for_server(&configuration, params)
                .await
                .map_err(|err| format!("API call to list_actions_for_server failed: {:?}", err))?
                .actions;

            println!(" Actions for server {}:", server_info.name);
            for action in actions {
                println!(
                    "  id: {}, command: {}, status: {:?}, progress: {}",
                    action.id, action.command, action.status, action.progress
                );
                if action.status == models::action::Status::Running {
                    any_running = true;
                }
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
    for server_info in &created_servers {
        println!(" Deleting server {}...", server_info.name);
        let params = servers_api::DeleteServerParams { id: server_info.id };
        servers_api::delete_server(&configuration, params)
            .await
            .map_err(|err| format!("API call to delete_server failed: {:?}", err))?;
    }
    println!("The servers should be deleted now!");

    Ok(())
}
