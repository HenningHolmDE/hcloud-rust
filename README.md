# hcloud for Rust

[![Crates.io](https://img.shields.io/crates/v/hcloud.svg)](https://crates.io/crates/hcloud)
[![Documentation](https://docs.rs/hcloud/badge.svg)](https://docs.rs/hcloud/)
![Build and test](https://github.com/HenningHolmDE/hcloud-rust/workflows/Build%20and%20test/badge.svg)

Unofficial Rust crate for accessing the [Hetzner Cloud API](https://docs.hetzner.cloud/)

## Overview

The `hcloud` crate can be used for managing the endpoints provided by the Hetzner Cloud API in your Rust project.

The API client code of this crate has been auto-generated from the [Unofficial OpenAPI Description for the Hetzner Cloud API](https://github.com/MaximilianKoestler/hcloud-openapi) using [OpenAPI Generator](https://openapi-generator.tech/).

## Current state of development

- supported API endpoints (complete as of July 2020): `actions`, `certificates`, `datacenters`, `floating_ips`, `images`, `isos`, `load_balancer_types`, `load_balancers`, `locations`, `networks`, `pricing`, `server_types`, `servers`, `ssh_keys`, `volumes`
- asynchronous API functions
- documentation and tests are still WIP

## Example

A very basic example for listing all existing servers:

```rust
use hcloud::apis::configuration::Configuration;
use hcloud::apis::servers_api;

// set up basic configuration using API token
let mut configuration = Configuration::new();
configuration.bearer_access_token =
   Some("YOUR_HCLOUD_API_TOKEN".to_string());

// get list of all existing servers from servers API
let servers = servers_api::list_servers(&configuration, Default::default())
   .await?
   .servers;

// handle server data
for server in servers {
   println!("{:?}", server);
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
