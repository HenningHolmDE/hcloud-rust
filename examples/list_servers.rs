use hcloud::apis::configuration::Configuration;
use hcloud::apis::servers_api;
use std::env;

#[tokio::main]
async fn main() -> Result<(), String> {
    // use API token from command line
    let api_token = env::args()
        .nth(1)
        .ok_or("Please provide API token as command line parameter.")?;

    // set up basic configuration using provided API token
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(api_token);

    // get list of all existing servers from servers API
    let servers = servers_api::list_servers(&configuration, Default::default())
        .await
        .map_err(|err| format!("API call to list_servers failed: {:?}", err))?
        .servers;

    println!("Found {} server(s), dumping server data:", servers.len());
    for server in servers {
        println!("{:?}", server);
    }

    Ok(())
}
