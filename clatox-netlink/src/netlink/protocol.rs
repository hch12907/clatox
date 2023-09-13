use libc::*;

pub enum Protocol {
    Route,

    // Other protocols are TODO
}

impl Protocol {
    pub const fn raw_value(&self) -> c_int {
        use Protocol::*;

        match self {
            Route => NETLINK_ROUTE,
        }
    }
}
