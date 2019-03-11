
#[macro_use] extern crate prettytable;
use prettytable::{Table, format};

mod ns;
use ns::Namespaced;
use ns::WithNetns;
use ns::ns_names;

mod link;
use link::{Links, links};

fn with_ns_name(name: &str) -> Result<(), ns::NsError> {
   let namespace = Namespaced::default();
   namespace.name(name)?.with_netns();
   Ok(())
}

fn pprint(link: &Links) {
   println!("Net Namespace name: {}", link.get_ns_name());

   let mut table = Table::new();
   table.add_row(
       row![bFg -> "Device Name", bFg -> "If-Index", bFg -> "If-Type", bFg -> "Veth-Peer-Index"],
   );

   for l in link.get_links() {
       table.add_row(row![l.get_name(), l.get_if_index(), 
                          l.get_link_type(), l.get_veth_peer()]);
   }

   table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
   table.printstd();
   println!();
}


fn main() {

    for name in ns_names() {
        if let Ok(_) = with_ns_name(&name){
            //println!("{:#?}", links(&name));
            pprint(&links(&name));
        }
    }
}

