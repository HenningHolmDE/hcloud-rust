use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    actions_api: Box<dyn crate::apis::ActionsApi>,
    datacenters_api: Box<dyn crate::apis::DatacentersApi>,
    floating_ips_api: Box<dyn crate::apis::FloatingIpsApi>,
    images_api: Box<dyn crate::apis::ImagesApi>,
    isos_api: Box<dyn crate::apis::IsosApi>,
    locations_api: Box<dyn crate::apis::LocationsApi>,
    networks_api: Box<dyn crate::apis::NetworksApi>,
    pricing_api: Box<dyn crate::apis::PricingApi>,
    server_types_api: Box<dyn crate::apis::ServerTypesApi>,
    servers_api: Box<dyn crate::apis::ServersApi>,
    ssh_keys_api: Box<dyn crate::apis::SshKeysApi>,
    volumes_api: Box<dyn crate::apis::VolumesApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            actions_api: Box::new(crate::apis::ActionsApiClient::new(rc.clone())),
            datacenters_api: Box::new(crate::apis::DatacentersApiClient::new(rc.clone())),
            floating_ips_api: Box::new(crate::apis::FloatingIpsApiClient::new(rc.clone())),
            images_api: Box::new(crate::apis::ImagesApiClient::new(rc.clone())),
            isos_api: Box::new(crate::apis::IsosApiClient::new(rc.clone())),
            locations_api: Box::new(crate::apis::LocationsApiClient::new(rc.clone())),
            networks_api: Box::new(crate::apis::NetworksApiClient::new(rc.clone())),
            pricing_api: Box::new(crate::apis::PricingApiClient::new(rc.clone())),
            server_types_api: Box::new(crate::apis::ServerTypesApiClient::new(rc.clone())),
            servers_api: Box::new(crate::apis::ServersApiClient::new(rc.clone())),
            ssh_keys_api: Box::new(crate::apis::SshKeysApiClient::new(rc.clone())),
            volumes_api: Box::new(crate::apis::VolumesApiClient::new(rc.clone())),
        }
    }

    pub fn actions_api(&self) -> &dyn crate::apis::ActionsApi{
        self.actions_api.as_ref()
    }

    pub fn datacenters_api(&self) -> &dyn crate::apis::DatacentersApi{
        self.datacenters_api.as_ref()
    }

    pub fn floating_ips_api(&self) -> &dyn crate::apis::FloatingIpsApi{
        self.floating_ips_api.as_ref()
    }

    pub fn images_api(&self) -> &dyn crate::apis::ImagesApi{
        self.images_api.as_ref()
    }

    pub fn isos_api(&self) -> &dyn crate::apis::IsosApi{
        self.isos_api.as_ref()
    }

    pub fn locations_api(&self) -> &dyn crate::apis::LocationsApi{
        self.locations_api.as_ref()
    }

    pub fn networks_api(&self) -> &dyn crate::apis::NetworksApi{
        self.networks_api.as_ref()
    }

    pub fn pricing_api(&self) -> &dyn crate::apis::PricingApi{
        self.pricing_api.as_ref()
    }

    pub fn server_types_api(&self) -> &dyn crate::apis::ServerTypesApi{
        self.server_types_api.as_ref()
    }

    pub fn servers_api(&self) -> &dyn crate::apis::ServersApi{
        self.servers_api.as_ref()
    }

    pub fn ssh_keys_api(&self) -> &dyn crate::apis::SshKeysApi{
        self.ssh_keys_api.as_ref()
    }

    pub fn volumes_api(&self) -> &dyn crate::apis::VolumesApi{
        self.volumes_api.as_ref()
    }

}
