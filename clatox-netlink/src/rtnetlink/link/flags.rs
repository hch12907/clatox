use libc::*;

use bitflags::bitflags;

bitflags! {
    /// A bitfield of all interface flags.
    ///
    /// It corresponds to `IFF_*` in libc.
    #[doc(alias("ifinfomsg", "ifi_flags", "IFF_"))]
    #[derive(Copy, Debug, Clone, PartialEq, Eq)]
    pub struct InterfaceFlags: u32 {
        const Up = IFF_UP as u32;
        const Broadcast = IFF_BROADCAST as u32;
        const Debug = IFF_DEBUG as u32;
        const Loopback = IFF_LOOPBACK as u32;
        const PointToPoint = IFF_POINTOPOINT as u32;
        const Running = IFF_RUNNING as u32;
        const NoArp = IFF_NOARP as u32;
        const Promiscuous = IFF_PROMISC as u32;
        const NoTrailers = IFF_NOTRAILERS as u32;
        const AllMulticast = IFF_ALLMULTI as u32;
        const Master = IFF_MASTER as u32;
        const Slave = IFF_SLAVE as u32;
        const Multicast = IFF_MULTICAST as u32;
        const PortSelect = IFF_PORTSEL as u32;
        const AutoMedia = IFF_AUTOMEDIA as u32;
        const Dynamic = IFF_DYNAMIC as u32;
        const LowerUp = IFF_LOWER_UP as u32;
        const Dormant = IFF_DORMANT as u32;
        const Echo = IFF_ECHO as u32;
    }
}
