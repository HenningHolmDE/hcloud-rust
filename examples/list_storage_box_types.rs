use hcloud::apis::configuration::Configuration;
use hcloud::apis::storage_box_types_api;
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

    // get list of storage box types from storage_boxes API
    // Note: This only requests the first page (max 25) of entries,
    //       see `list_isos.rs` for an example of using pagination.
    let storage_box_types =
        storage_box_types_api::list_storage_box_types(&configuration, Default::default())
            .await
            .map_err(|err| format!("API call to list_storage_box_types failed: {:?}", err))?
            .storage_box_types;

    println!(
        "Found {} storage box type(s), dumping data:",
        storage_box_types.len()
    );
    for storage_box_type in storage_box_types {
        println!("{:?}", storage_box_type);
    }

    Ok(())
}
