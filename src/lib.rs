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
//!- supported API endpoints (complete as of July 2020): `actions`, `certificates`, `datacenters`, `floating_ips`, `images`, `isos`, `load_balancer_types`, `load_balancers`, `locations`, `networks`, `pricing`, `server_types`, `servers`, `ssh_keys`, `volumes`
//!- asynchronous API functions
//!- documentation and tests are still WIP
//!
//!# Example
//!
//!A very basic example for listing all existing servers:
//!
//!```rust,no_run
//!use hcloud::apis::configuration::Configuration;
//!use hcloud::apis::servers_api;
//!
//!# #[tokio::main]
//!# async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!// set up basic configuration using API token
//!let mut configuration = Configuration::new();
//!configuration.bearer_access_token =
//!    Some("YOUR_HCLOUD_API_TOKEN".to_string());
//!
//!// get list of all existing servers from servers API
//!let servers = servers_api::list_servers(&configuration, Default::default())
//!    .await?
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
