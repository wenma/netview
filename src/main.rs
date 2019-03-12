
#[macro_use] extern crate prettytable;
use prettytable::{Table, format};

mod ns;
use ns::Namespaced;
use ns::WithNetns;
use ns::ns_names;

mod link;
use link::{Links, links, with_containers};

mod containers;


fn with_ns_name(name: &str) -> Result<(), ns::NsError> {
   let namespace = Namespaced::default();
   namespace.name(name)?.with_netns();
   Ok(())
}

fn pprint(link: &Links) {

   println!("Net-Namespace Name: `{}`", link.get_ns_name());
   let mut table = Table::new();
   table.add_row(
       row![bFg -> "Device Name", bFg -> "If-Index", 
            bFg -> "If-Type", bFg -> "Veth-Peer-Index",
            bFg -> "Container"],
   );

   for l in link.get_links() {
       table.add_row(row![l.get_name(), l.get_if_index(), 
                          l.get_link_type(), l.get_veth_peer(),
                          l.get_container()]);
   }

   table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
   table.printstd();
   println!();
}


fn main() {

    let mut ls: Vec<Links> = Vec::new();
    for name in ns_names() {
        if let Ok(_) = with_ns_name(&name){
            ls.push(links(&name));
        }
    }

    with_containers(&mut ls);

    for link in &ls {
        pprint(link);
    }
}

