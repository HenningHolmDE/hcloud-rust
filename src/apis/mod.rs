use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(e) => fmt::Display::fmt(e, f),
            Error::Serde(e) => fmt::Display::fmt(e, f),
            Error::Io(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(err) => Some(err),
            Error::Serde(err) => Some(err),
            Error::Io(err) => Some(err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod actions_api;
pub use self::actions_api::{ ActionsApi, ActionsApiClient };
mod datacenters_api;
pub use self::datacenters_api::{ DatacentersApi, DatacentersApiClient };
mod floating_ips_api;
pub use self::floating_ips_api::{ FloatingIpsApi, FloatingIpsApiClient };
mod images_api;
pub use self::images_api::{ ImagesApi, ImagesApiClient };
mod isos_api;
pub use self::isos_api::{ IsosApi, IsosApiClient };
mod locations_api;
pub use self::locations_api::{ LocationsApi, LocationsApiClient };
mod networks_api;
pub use self::networks_api::{ NetworksApi, NetworksApiClient };
mod pricing_api;
pub use self::pricing_api::{ PricingApi, PricingApiClient };
mod server_types_api;
pub use self::server_types_api::{ ServerTypesApi, ServerTypesApiClient };
mod servers_api;
pub use self::servers_api::{ ServersApi, ServersApiClient };
mod ssh_keys_api;
pub use self::ssh_keys_api::{ SshKeysApi, SshKeysApiClient };
mod volumes_api;
pub use self::volumes_api::{ VolumesApi, VolumesApiClient };

pub mod configuration;
pub mod client;
