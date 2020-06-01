//!Unofficial Rust crate for accessing the [Hetzner Cloud API](https://docs.hetzner.cloud/)
//!
//!# Overview
//!
//!The `hcloud` crate can be used for managing the endpoints provided by the Hetzner Cloud API in your Rust project.
//!
//!The API client code of this crate has been auto-generated from the [Unofficial OpenAPI Description for the Hetzner Cloud API](https://github.com/MaximilianKoestler/hcloud-openapi) using [OpenAPI Generator](https://openapi-generator.tech/).
//!
//!# Current state of development
//!
//!- supported API endpoints (complete as of May 2020): `actions`, `datacenters`, `floating_ips`, `images`, `isos`, `locations`, `networks`, `pricing`, `server_types`, `servers`, `ssh_keys`, `volumes`
//!- `async`/`await` is not supported yet: The crate currently uses the blocking version 0.9 of the `reqwest` crate for underlying HTTP access and is therefore not asynchronous. This is planned to be implemented after the new OpenAPI Generator version 5.x has been released, in which generating asynchronous Rust client code is supported.
//!- documentation and tests are still WIP
//!
//!# Example
//!
//!A very basic example for listing all existing servers:
//!
//!```rust,no_run
//!use hcloud::apis::client::APIClient;
//!use hcloud::apis::configuration::Configuration;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!// set up basic configuration using API token
//!let mut configuration = Configuration::new();
//!configuration.bearer_access_token =
//!    Some("YOUR_HCLOUD_API_TOKEN".to_string());
//!
//!// create API client handle from configuration
//!let api_client = APIClient::new(configuration);
//!
//!// get list of all existing servers from servers API
//!let servers = api_client
//!    .servers_api()
//!    .list_servers(None, None, None, None)?
//!    .servers;
//!
//!// handle server data
//!for server in servers {
//!    println!("{:?}", server);
//!}
//!# Ok(())
//!# }
//!```

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
