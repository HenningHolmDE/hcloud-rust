use hcloud::apis::configuration::Configuration;
use hcloud::apis::isos_api;
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

    let mut isos = Vec::new();

    // start with page 1
    let mut next_page = Some(1);
    while next_page.is_some() {
        // query ISOs API for a page of ISOs
        let params = isos_api::ListIsosParams {
            page: next_page,
            per_page: Some(10), // arbitrary value for this example, Hetzner's default is 25, maximum is 50
            ..Default::default()
        };
        let response = isos_api::list_isos(&configuration, params)
            .await
            .map_err(|err| format!("API call to list_isos failed: {:?}", err))?;

        // add ISOs to list
        isos.extend(response.isos);

        // proceed with next page (if existing)
        next_page = response
            .meta
            .map(|meta| meta.pagination)
            .and_then(|pagination| pagination.next_page);
    }

    println!("Found {} ISOs. Listing IDs and names:", isos.len());
    for iso in isos {
        println!(
            "{}: {}",
            iso.id,
            iso.name.unwrap_or("<unnamed>".to_string())
        );
    }

    Ok(())
}
