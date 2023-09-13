use libc::*;

use bitflags::bitflags;

bitflags! {
    #[doc(alias("rtmsg", "RTM_F_"))]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RouteFlags: u32 {
        const Notify = RTM_F_NOTIFY;
        const Cloned = RTM_F_CLONED;
        const Equalize = RTM_F_EQUALIZE;
        const Prefix = RTM_F_PREFIX;
    }
}
