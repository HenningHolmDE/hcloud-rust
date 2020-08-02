use reqwest;
use serde_json;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

impl<T> fmt::Display for ResponseContent<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.content)
    }
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(e) => fmt::Display::fmt(e, f),
            Error::Serde(e) => fmt::Display::fmt(e, f),
            Error::Io(e) => fmt::Display::fmt(e, f),
            Error::ResponseError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(err) => Some(err),
            Error::Serde(err) => Some(err),
            Error::Io(err) => Some(err),
            Error::ResponseError(_) => None,
        }
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod actions_api;
pub mod datacenters_api;
pub mod floating_ips_api;
pub mod images_api;
pub mod isos_api;
pub mod locations_api;
pub mod networks_api;
pub mod pricing_api;
pub mod server_types_api;
pub mod servers_api;
pub mod ssh_keys_api;
pub mod volumes_api;

pub mod configuration;
