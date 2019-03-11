

use futures::{Stream, Future};
use rtnetlink::new_connection;
use tokio_core::reactor::Core;
use rtnetlink::packet::{LinkInfo, LinkInfoKind, LinkNla};

#[derive(Debug, Clone)]
pub struct Links {
    ns_name: String,
    links: Vec<LinkDevice>,
}

impl Links {
    fn new() -> Self {
        Links::default()
    }

    pub fn get_ns_name(&self) -> String {
        self.ns_name.clone()
    }

    pub fn get_links(&self) -> Vec<LinkDevice> {
        self.links.clone()
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
    pub veth_peer: Option<u32>
}

impl Default for LinkDevice {
    fn default() -> Self {
        LinkDevice{
            name: String::new(),
            if_index: 0,
            link_type: None,
            veth_peer: None,
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
}


pub fn links(ns_name: &str) -> Links {
    let (connection, handle) = new_connection().unwrap();
    let mut core = Core::new().unwrap();
    core.handle().spawn(connection.map_err(|_| ()));

    let mut links = Links::new();
    links.ns_name = String::from(ns_name);

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

        links.links.push(ld);
        Ok(())

    }); 

    core.run(request).unwrap();
    links
}
