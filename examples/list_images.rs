use hcloud::apis::configuration::Configuration;
use hcloud::apis::images_api;
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

    let mut images = Vec::new();

    // start with page 1
    let mut next_page = Some(1);
    while next_page.is_some() {
        // query images API for a page of images
        let params = images_api::ListImagesParams {
            page: next_page,
            ..Default::default()
        };
        let response = images_api::list_images(&configuration, params)
            .await
            .map_err(|err| format!("API call to list_images failed: {:?}", err))?;

        // add images to list
        images.extend(response.images);

        // proceed with next page (if existing)
        next_page = response.meta.pagination.next_page
    }

    println!("Found {} images. Listing IDs and names:", images.len());
    for image in images {
        println!(
            "{}: {} ({})",
            image.id,
            image.name.unwrap_or("<unnamed>".to_string()),
            image.description,
        );
    }

    Ok(())
}
