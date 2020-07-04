use hcloud::apis::client::APIClient;
use hcloud::apis::configuration::Configuration;
use hcloud::models::ServerType;
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

    // query server types API for a list of all server types
    let server_types = api_client
        .server_types_api()
        .list_server_types(Default::default())
        .map_err(|err| format!("API call to list_server_types failed: {:?}", err))?
        .server_types;

    // use helper functions below for finding the cheapest server type
    let cheapest_server_type = server_types
        .into_iter()
        // map server type to iterator over ServerTypePriceInfo,
        // flatten to get rid of outer iterator
        .flat_map(map_server_type)
        .min_by(compare_prices)
        .ok_or("Could not find cheapest server type")?;

    println!("Cheapest server type:");
    println!(" Server type name: {}", cheapest_server_type.name);
    println!(" Location: {}", cheapest_server_type.location);
    println!(" Hourly gross price: {}", cheapest_server_type.gross_price);

    Ok(())
}

// helper type for processing server type data
struct ServerTypePriceInfo {
    name: String,
    location: String,
    gross_price: f32,
}

// helper function to map server type to iterator over ServerTypePriceInfo
// of all available locations and hourly gross prices
// - gross price strings are parsed into f32 for later comparison
// - locations with unparsable prices are filtered out
fn map_server_type(server_type: ServerType) -> impl Iterator<Item = ServerTypePriceInfo> {
    let name = server_type.name;
    server_type
        .prices
        .into_iter()
        .filter_map(move |price_per_time| {
            price_per_time
                .price_hourly
                .gross
                .parse::<f32>()
                .ok()
                .map(|gross_price| ServerTypePriceInfo {
                    name: name.clone(),
                    location: price_per_time.location,
                    gross_price,
                })
        })
}

// helper function for price comparison
fn compare_prices(x: &ServerTypePriceInfo, y: &ServerTypePriceInfo) -> std::cmp::Ordering {
    x.gross_price
        .partial_cmp(&y.gross_price)
        .unwrap_or(std::cmp::Ordering::Equal)
}
