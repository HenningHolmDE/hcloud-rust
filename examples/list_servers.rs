use hcloud::apis::client::APIClient;
use hcloud::apis::configuration::Configuration;
use std::env;

fn main() -> Result<(), String> {
    // use API token from command line
    let api_token = env::args()
        .nth(1)
        .ok_or("Please provide API token as command line parameter.")?;

    // set up basic configuration using provided API token
    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(api_token);

    // create API client handle from configuration
    let api_client = APIClient::new(configuration);

    // get list of all existing servers from servers API
    let servers = api_client
        .servers_api()
        .list_servers(None, None, None, None)
        .map_err(|err| format!("API call to list_servers failed: {:?}", err))?
        .servers;

    println!("Found {} server(s), dumping server data:", servers.len());
    for server in servers {
        println!("{:?}", server);
    }

    Ok(())
}
