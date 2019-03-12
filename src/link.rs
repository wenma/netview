
use std::collections::HashMap;

use futures::{Stream, Future};
use rtnetlink::new_connection;
use tokio_core::reactor::Core;
use rtnetlink::packet::{LinkInfo, LinkInfoKind, LinkNla};

use super::containers::Container;
use super::containers::containers;

#[derive(Debug, Clone)]
pub struct Links {
    ns_name: String,
    links: Vec<LinkDevice>,
}

impl Links {
    fn new() -> Self {
        Links::default()
    }

    fn ns_name(&mut self, name: String) {
        self.ns_name = name;
    }

    pub fn get_ns_name(&self) -> String {
        self.ns_name.clone()
    }

    pub fn get_links(&self) -> Vec<LinkDevice> {
        self.links.clone()
    }

    fn get_links_mut(&mut self) -> &mut Vec<LinkDevice> {
        &mut self.links
    }
}

impl Default for Links {
    fn default() -> Self {
        Links {
            ns_name: String::new(),
            links: Vec::<LinkDevice>::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkDevice {
    pub name: String,
    pub if_index: u32,
    pub link_type: Option<LinkInfoKind>,
    pub veth_peer: Option<u32>,
    pub container: Option<Container>,
}

impl Default for LinkDevice {
    fn default() -> Self {
        LinkDevice{
            name: String::new(),
            if_index: 0,
            link_type: None,
            veth_peer: None,
            container: None
        }
    }
}

pub trait KindToString {
    fn to_string(&self) -> String;
}

impl KindToString for LinkInfoKind {
    fn to_string(&self) -> String {
        let name = match self {
            LinkInfoKind::Dummy => "Dummy",
            LinkInfoKind::Ifb => "Ifb",
            LinkInfoKind::Bridge => "Bridge",
            LinkInfoKind::Tun => "Tun",
            LinkInfoKind::Nlmon => "Nlmon",
            LinkInfoKind::Vlan => "Vlan",
            LinkInfoKind::Veth => "Veth",
            LinkInfoKind::Vxlan => "Vxlan",
            LinkInfoKind::Bond => "Bond",
            LinkInfoKind::IpVlan => "IpVlan",
            LinkInfoKind::MacVlan => "MacVlan",
            LinkInfoKind::MacVtap => "MacVtap",
            LinkInfoKind::GreTap => "GreTap",
            LinkInfoKind::GreTap6 => "GreTap6",
            LinkInfoKind::IpTun => "IpTun",
            LinkInfoKind::SitTun => "SitTun",
            LinkInfoKind::GreTun => "GreTun",
            LinkInfoKind::GreTun6 => "GreTun6",
            LinkInfoKind::Vti => "Vti",
            LinkInfoKind::Vrf => "Vrf",
            LinkInfoKind::Gtp => "Gtp",
            LinkInfoKind::Other(s) => s,
        };
        String::from(name)
    }
}

impl LinkDevice {
    fn new() -> Self {
        LinkDevice::default()
    }

    fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn if_index(&mut self, index: u32) {
        self.if_index = index;
    }

    pub fn get_if_index(&self) -> String {
        format!("{}", self.if_index)
    }

    fn link_type(&mut self, link_type: LinkInfoKind) {
        self.link_type = Some(link_type);
    }

    pub fn get_link_type(&self) -> String {
        match self.link_type {
            Some(ref s) => s.to_string(),
            None => String::new(),
        }
    }

    fn veth_peer(&mut self, peer: u32) {
        self.veth_peer = Some(peer);
    }

    pub fn get_veth_peer(&self) -> String {
        match self.veth_peer {
            Some(ref p) => p.to_string(),
            None => String::new()
        }
    }

    fn container(&mut self, c: Container) {
        self.container = Some(c);
    }

    pub fn get_container(&self) -> String {
        match self.container.clone() {
            Some(c) => c.to_string(),
            None => String::new(),
        }
    }
}

pub fn links<'a>(ns_name: &'a str) -> Links {

    let mut links = Links::new();
    links.ns_name(String::from(ns_name));

    let (connection, handle) = new_connection().unwrap();
    let mut core = Core::new().unwrap();
    core.handle().spawn(connection.map_err(|_| ()));

    let request = handle.link().get().execute().for_each(|link| {
        let mut ld = LinkDevice::new();
        ld.if_index(link.header().index());
        for nla in link.nlas() {
            if let LinkNla::IfName(ref name) = nla {
                ld.name(name.clone());
            }

            if let LinkNla::LinkInfo(ref infos) = nla {
                for info in infos {
                    if let LinkInfo::Kind(ref kind) = info {
                        ld.link_type(kind.clone());
                    }
                }
            }

            if let LinkNla::Link(peer_id) = nla {
                ld.veth_peer(*peer_id);
            }
        }

        links.get_links_mut().push(ld);
        Ok(())

    }); 

    core.run(request).unwrap();
    links
}

pub fn with_containers<'a>(links: &'a mut Vec<Links>) {
    let cs: Vec<Container> = containers();
    let mut if_index_map: HashMap<String, &mut LinkDevice> = HashMap::new();
    let mut container_map: HashMap<String, Option<Container>> = HashMap::new();

    for link in links {
        
         let ns_name = link.get_ns_name();
         for device in link.get_links_mut() {
            
            let if_index = device.get_if_index();
            if_index_map.insert(if_index.clone(), device);

            let mut container: Option<Container> = None;
            for c in &cs {
                if c.get_config().get_sandbox_key().ends_with(&ns_name) && 
                    ns_name != "default" {
                    container = Some(c.clone());
                }
            }

            if if_index != "1" {
                container_map.insert(if_index, container);
            }
        }
    }
 
    for (if_index, link_device) in if_index_map {
        let peer_index = link_device.get_veth_peer();

        let _ = [container_map.get(&if_index), 
                 container_map.get(&peer_index)]
                .iter()
                .map(|o|{

            if let Some(container_ref) = o {
                if let Some(c) = container_ref {
                    link_device.container((*c).clone());
                }
            }
         }).collect::<()>();
    }
}
