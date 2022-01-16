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
//!- supported API endpoints (complete as of January 2022): `actions`, `certificates`, `datacenters`, `firewalls`, `floating_ips`, `images`, `isos`, `load_balancer_types`, `load_balancers`, `locations`, `networks`, `pricing`, `placement_groups`, `server_types`, `servers`, `ssh_keys`, `volumes`
//!- asynchronous API functions
//!- pagination support
//!- documentation and tests are still WIP
//!
//!# Selecting TLS implementation
//!
//!The underlying TLS implementation for `reqwest` can be selected using [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section):
//!- **default-tls** *(enabled by default)*: Provides TLS support to connect over HTTPS.
//!- **native-tls**: Enables TLS functionality provided by `native-tls`.
//!- **native-tls-vendored**: Enables the `vendored` feature of `native-tls`.
//!- **rustls-tls**: Enables TLS functionality provided by `rustls`.
//!
//!(Refer to [Optional Features](https://docs.rs/reqwest/latest/reqwest/#optional-features) in the `reqwest` documentation.)
//!
//!Example for using the TLS functionality provided by `rustls`:
//!```toml
//![dependencies]
//!hcloud = { version = "*", default-features = false, features = ["rustls-tls"] }
//!```
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
