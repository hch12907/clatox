use libc::*;

use bitflags::bitflags;

bitflags! {
    /// This is a bitfield consisting of multiple Netlink flags.
    /// It corresponds to `nlmsg_flags`.
    #[doc(alias("nlmsghdr", "nlmsg_flags", "NLM_F_"))]
    #[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
    pub struct Flags: u16 {
        const Request = NLM_F_REQUEST as u16;
        const Multi = NLM_F_MULTI as u16;
        const Ack = NLM_F_ACK as u16;
        const Echo = NLM_F_ECHO as u16;
        
        // Get flags
        const Root = NLM_F_ROOT as u16;
        const Match = NLM_F_MATCH as u16;
        const Atomic = NLM_F_ATOMIC as u16;
        const Dump = NLM_F_DUMP as u16;

        // New flags
        const Replace = NLM_F_REPLACE as u16;
        const Excl = NLM_F_EXCL as u16;
        const Create = NLM_F_CREATE as u16;
        const Append = NLM_F_APPEND as u16;
    }
}
