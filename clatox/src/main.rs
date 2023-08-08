use std::process::exit;

use caps::{CapSet, Capability};

fn main() {
    let has_admin_cap = caps::has_cap(None, CapSet::Effective, Capability::CAP_NET_ADMIN)
        .ok()
        .unwrap_or(false);

    if !has_admin_cap {
        println!("No CAP_NET_ADMIN capability - try su or sudo.");
        exit(1);
    };

    todo!("clat implementation");
}
