use hcloud::apis::configuration::Configuration;
use hcloud::apis::zones_api;
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

    // get list of all existing zones from zones API
    // Note: This only requests the first page (max 25) of zones,
    //       see `list_isos.rs` for an example of using pagination.
    let zones = zones_api::list_zones(&configuration, Default::default())
        .await
        .map_err(|err| format!("API call to list_zones failed: {:?}", err))?
        .zones;

    println!("Found {} zone(s), dumping zone data:", zones.len());
    for zone in zones {
        println!("{:?}", zone);
    }

    Ok(())
}
