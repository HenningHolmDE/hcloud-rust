use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    actions_api: Box<dyn crate::apis::actions_api::ActionsApi>,
    datacenters_api: Box<dyn crate::apis::datacenters_api::DatacentersApi>,
    floating_ips_api: Box<dyn crate::apis::floating_ips_api::FloatingIpsApi>,
    images_api: Box<dyn crate::apis::images_api::ImagesApi>,
    isos_api: Box<dyn crate::apis::isos_api::IsosApi>,
    locations_api: Box<dyn crate::apis::locations_api::LocationsApi>,
    networks_api: Box<dyn crate::apis::networks_api::NetworksApi>,
    pricing_api: Box<dyn crate::apis::pricing_api::PricingApi>,
    server_types_api: Box<dyn crate::apis::server_types_api::ServerTypesApi>,
    servers_api: Box<dyn crate::apis::servers_api::ServersApi>,
    ssh_keys_api: Box<dyn crate::apis::ssh_keys_api::SshKeysApi>,
    volumes_api: Box<dyn crate::apis::volumes_api::VolumesApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            actions_api: Box::new(crate::apis::actions_api::ActionsApiClient::new(rc.clone())),
            datacenters_api: Box::new(crate::apis::datacenters_api::DatacentersApiClient::new(rc.clone())),
            floating_ips_api: Box::new(crate::apis::floating_ips_api::FloatingIpsApiClient::new(rc.clone())),
            images_api: Box::new(crate::apis::images_api::ImagesApiClient::new(rc.clone())),
            isos_api: Box::new(crate::apis::isos_api::IsosApiClient::new(rc.clone())),
            locations_api: Box::new(crate::apis::locations_api::LocationsApiClient::new(rc.clone())),
            networks_api: Box::new(crate::apis::networks_api::NetworksApiClient::new(rc.clone())),
            pricing_api: Box::new(crate::apis::pricing_api::PricingApiClient::new(rc.clone())),
            server_types_api: Box::new(crate::apis::server_types_api::ServerTypesApiClient::new(rc.clone())),
            servers_api: Box::new(crate::apis::servers_api::ServersApiClient::new(rc.clone())),
            ssh_keys_api: Box::new(crate::apis::ssh_keys_api::SshKeysApiClient::new(rc.clone())),
            volumes_api: Box::new(crate::apis::volumes_api::VolumesApiClient::new(rc.clone())),
        }
    }

    pub fn actions_api(&self) -> &dyn crate::apis::actions_api::ActionsApi{
        self.actions_api.as_ref()
    }

    pub fn datacenters_api(&self) -> &dyn crate::apis::datacenters_api::DatacentersApi{
        self.datacenters_api.as_ref()
    }

    pub fn floating_ips_api(&self) -> &dyn crate::apis::floating_ips_api::FloatingIpsApi{
        self.floating_ips_api.as_ref()
    }

    pub fn images_api(&self) -> &dyn crate::apis::images_api::ImagesApi{
        self.images_api.as_ref()
    }

    pub fn isos_api(&self) -> &dyn crate::apis::isos_api::IsosApi{
        self.isos_api.as_ref()
    }

    pub fn locations_api(&self) -> &dyn crate::apis::locations_api::LocationsApi{
        self.locations_api.as_ref()
    }

    pub fn networks_api(&self) -> &dyn crate::apis::networks_api::NetworksApi{
        self.networks_api.as_ref()
    }

    pub fn pricing_api(&self) -> &dyn crate::apis::pricing_api::PricingApi{
        self.pricing_api.as_ref()
    }

    pub fn server_types_api(&self) -> &dyn crate::apis::server_types_api::ServerTypesApi{
        self.server_types_api.as_ref()
    }

    pub fn servers_api(&self) -> &dyn crate::apis::servers_api::ServersApi{
        self.servers_api.as_ref()
    }

    pub fn ssh_keys_api(&self) -> &dyn crate::apis::ssh_keys_api::SshKeysApi{
        self.ssh_keys_api.as_ref()
    }

    pub fn volumes_api(&self) -> &dyn crate::apis::volumes_api::VolumesApi{
        self.volumes_api.as_ref()
    }

}
