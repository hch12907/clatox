use libc::*;

pub const ARPHRD_MCTP: u16 = 290;
pub const ARPHRD_RAWIP: u16 = 519;
pub const ARPHRD_IEEE802154_MONITOR: u16 = 805;
pub const ARPHRD_PHONET: u16 = 820;
pub const ARPHRD_PHONET_PIPE: u16 = 821;
pub const ARPHRD_CAIF: u16 = 822;
pub const ARPHRD_IP6GRE: u16 = 823;
pub const ARPHRD_NETLINK: u16 = 824;
pub const ARPHRD_6LOWPAN: u16 = 825;
pub const ARPHRD_VSOCKMON: u16 = 826;

/// Those enum values correspond to `ARPHRD_*` constants in libc.
///
/// The term *ARP Hardware* has historical origins and should not be taken
/// literally. It is better to understand the term as *Data Link (OSI layer 2)
/// devices*.
///
/// It is also notable that most common interfaces will be of the Ethernet type,
/// even though e.g. Ieee80211 may seem more intuitive for wireless cards. Use
/// the `ip` tool on Linux to examine the interface device type.
///
/// Naming conventions
/// ======
/// Unlike many other enums in this library which try to establish a human-readable
/// name for its variants, the variants in this enum are given deliberately
/// names that are similar to their C `ARPHRD_*` counterparts, only that they
/// are adapted to camel case. The reasoning is that those names are not
/// semantical - they are merely names of various technologies, and changing
/// them would actually lead to confusions.
/// 
/// However, a limited number of names are still modified. `ARPHRD_EETHER` and
/// `ARPHRD_HWX25`, for example, as parts of their names *are* semantic.
#[doc(alias("ifinfomsg", "ifi_type", "ARPHRD_"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
#[non_exhaustive]
pub enum ArpHardware {
    NetRom = ARPHRD_NETROM,
    Ethernet = ARPHRD_ETHER,
    ExperimentalEthernet = ARPHRD_EETHER,
    Ax25 = ARPHRD_AX25,
    Pronet = ARPHRD_PRONET,
    Chaos = ARPHRD_CHAOS,
    Ieee802 = ARPHRD_IEEE802,
    ArcNet = ARPHRD_ARCNET,
    AppleTalk = ARPHRD_APPLETLK,
    Dlci = ARPHRD_DLCI,
    Atm = ARPHRD_ATM,
    Metricom = ARPHRD_METRICOM,
    Ieee1394 = ARPHRD_IEEE1394,
    Eui64 = ARPHRD_EUI64,
    InfiniBand = ARPHRD_INFINIBAND,

    /// Serial Line Internet Protocol
    Slip = ARPHRD_SLIP,
    /// Compressed Serial Line Internet Protocol
    CSlip = ARPHRD_CSLIP,
    Slip6 = ARPHRD_SLIP6,
    CSlip6 = ARPHRD_CSLIP6,
    /// Reserved
    Rsrvd = ARPHRD_RSRVD,
    Adapt = ARPHRD_ADAPT,
    Rose = ARPHRD_ROSE,
    X25 = ARPHRD_X25,
    HardwareX25 = ARPHRD_HWX25,
    Can = ARPHRD_CAN,
    Mctp = ARPHRD_MCTP,
    Ppp = ARPHRD_PPP,
    /// Cisco HDLC, equivalent to `ARPHRD_CISCO`
    Hdlc = ARPHRD_HDLC,
    Lapd = ARPHRD_LAPB,
    Ddcmp = ARPHRD_DDCMP,
    RawHdlc = ARPHRD_RAWHDLC,
    RawIp = 519, //ARPHRD_RAWIP,

    Tunnel = ARPHRD_TUNNEL,
    Tunnel6 = ARPHRD_TUNNEL6,
    Frad = ARPHRD_FRAD,
    Skip = ARPHRD_SKIP,
    Loopback = ARPHRD_LOOPBACK,
    Localtalk = ARPHRD_LOCALTLK,
    Fddi = ARPHRD_FDDI,
    Bif = ARPHRD_BIF,
    Sit = ARPHRD_SIT,
    IpDdp = ARPHRD_IPDDP,
    IpGre = ARPHRD_IPGRE,
    PimReg = ARPHRD_PIMREG,
    HiPPI = ARPHRD_HIPPI,
    Ash = ARPHRD_ASH,
    Econet = ARPHRD_ECONET,
    IrDA = ARPHRD_IRDA,
    Fcpp = ARPHRD_FCPP,
    Fcal = ARPHRD_FCAL,
    Fcpl = ARPHRD_FCPL,
    FcFabric = ARPHRD_FCFABRIC,
    Ieee802TR = ARPHRD_IEEE802_TR,
    Ieee80211 = ARPHRD_IEEE80211,
    Ieee80211Prism = ARPHRD_IEEE80211_PRISM,
    Ieee80211RadioTap = ARPHRD_IEEE80211_RADIOTAP,
    Ieee802154 = ARPHRD_IEEE802154,
    Ieee802154Monitor = ARPHRD_IEEE802154_MONITOR,
    PhoNet = ARPHRD_PHONET,
    PhoNetPipe = ARPHRD_PHONET_PIPE,
    Caif = ARPHRD_CAIF,
    Ipv6Gre = ARPHRD_IP6GRE,
    Netlink = ARPHRD_NETLINK,
    Ipv6OverLowpan = ARPHRD_6LOWPAN,
    VsockMonitor = ARPHRD_VSOCKMON,

    Void = ARPHRD_VOID,
    None = ARPHRD_NONE,
}

impl ArpHardware {
    pub fn raw_value(&self) -> u16 {
        use ArpHardware::*;

        match self {
            NetRom => ARPHRD_NETROM,
            Ethernet => ARPHRD_ETHER,
            ExperimentalEthernet => ARPHRD_EETHER,
            Ax25 => ARPHRD_AX25,
            Pronet => ARPHRD_PRONET,
            Chaos => ARPHRD_CHAOS,
            Ieee802 => ARPHRD_IEEE802,
            ArcNet => ARPHRD_ARCNET,
            AppleTalk => ARPHRD_APPLETLK,
            Dlci => ARPHRD_DLCI,
            Atm => ARPHRD_ATM,
            Metricom => ARPHRD_METRICOM,
            Ieee1394 => ARPHRD_IEEE1394,
            Eui64 => ARPHRD_EUI64,
            InfiniBand => ARPHRD_INFINIBAND,
            Slip => ARPHRD_SLIP,
            CSlip => ARPHRD_CSLIP,
            Slip6 => ARPHRD_SLIP6,
            CSlip6 => ARPHRD_CSLIP6,
            Rsrvd => ARPHRD_RSRVD,
            Adapt => ARPHRD_ADAPT,
            Rose => ARPHRD_ROSE,
            X25 => ARPHRD_X25,
            HardwareX25 => ARPHRD_HWX25,
            Can => ARPHRD_CAN,
            Mctp => 290, //ARPHRD_MCTP,
            Ppp => ARPHRD_PPP,
            Hdlc => ARPHRD_HDLC,
            Lapd => ARPHRD_LAPB,
            Ddcmp => ARPHRD_DDCMP,
            RawHdlc => ARPHRD_RAWHDLC,
            RawIp => 519, //ARPHRD_RAWIP,
            Tunnel => ARPHRD_TUNNEL,
            Tunnel6 => ARPHRD_TUNNEL6,
            Frad => ARPHRD_FRAD,
            Skip => ARPHRD_SKIP,
            Loopback => ARPHRD_LOOPBACK,
            Localtalk => ARPHRD_LOCALTLK,
            Fddi => ARPHRD_FDDI,
            Bif => ARPHRD_BIF,
            Sit => ARPHRD_SIT,
            IpDdp => ARPHRD_IPDDP,
            IpGre => ARPHRD_IPGRE,
            PimReg => ARPHRD_PIMREG,
            HiPPI => ARPHRD_HIPPI,
            Ash => ARPHRD_ASH,
            Econet => ARPHRD_ECONET,
            IrDA => ARPHRD_IRDA,
            Fcpp => ARPHRD_FCPP,
            Fcal => ARPHRD_FCAL,
            Fcpl => ARPHRD_FCPL,
            FcFabric => ARPHRD_FCFABRIC,
            Ieee802TR => ARPHRD_IEEE802_TR,
            Ieee80211 => ARPHRD_IEEE80211,
            Ieee80211Prism => ARPHRD_IEEE80211_PRISM,
            Ieee80211RadioTap => ARPHRD_IEEE80211_RADIOTAP,
            Ieee802154 => ARPHRD_IEEE802154,
            Ieee802154Monitor =>ARPHRD_IEEE802154_MONITOR,
            PhoNet => ARPHRD_PHONET,
            PhoNetPipe => ARPHRD_PHONET_PIPE,
            Caif => ARPHRD_CAIF,
            Ipv6Gre => ARPHRD_IP6GRE,
            Netlink => ARPHRD_NETLINK,
            Ipv6OverLowpan => ARPHRD_6LOWPAN,
            VsockMonitor => ARPHRD_VSOCKMON,
            Void => ARPHRD_VOID,
            None => ARPHRD_NONE,
        }
    }

    pub fn from_raw_value(value: u16) -> Option<Self> {
        use ArpHardware::*;

        Some(match value {
            ARPHRD_NETROM => NetRom,
            ARPHRD_ETHER => Ethernet,
            ARPHRD_EETHER => ExperimentalEthernet,
            ARPHRD_AX25 => Ax25,
            ARPHRD_PRONET => Pronet,
            ARPHRD_CHAOS => Chaos,
            ARPHRD_IEEE802 => Ieee802,
            ARPHRD_ARCNET => ArcNet,
            ARPHRD_APPLETLK => AppleTalk,
            ARPHRD_DLCI => Dlci,
            ARPHRD_ATM => Atm,
            ARPHRD_METRICOM => Metricom,
            ARPHRD_IEEE1394 => Ieee1394,
            ARPHRD_EUI64 => Eui64,
            ARPHRD_INFINIBAND => InfiniBand,
            ARPHRD_LOOPBACK => Loopback,
            ARPHRD_SLIP => Slip,
            ARPHRD_CSLIP => CSlip,
            ARPHRD_SLIP6 => Slip6,
            ARPHRD_CSLIP6 => CSlip6,
            ARPHRD_RSRVD => Rsrvd,
            ARPHRD_ADAPT => Adapt,
            ARPHRD_ROSE => Rose,
            ARPHRD_X25 => X25,
            ARPHRD_HWX25 => HardwareX25,
            ARPHRD_CAN => Can,
            ARPHRD_MCTP => Mctp,
            ARPHRD_PPP => Ppp,
            ARPHRD_HDLC => Hdlc,
            ARPHRD_LAPB => Lapd,
            ARPHRD_DDCMP => Ddcmp,
            ARPHRD_RAWHDLC => RawHdlc,
            ARPHRD_RAWIP => RawIp,
            ARPHRD_TUNNEL => Tunnel,
            ARPHRD_TUNNEL6 => Tunnel6,
            ARPHRD_FRAD => Frad,
            ARPHRD_SKIP => Skip,
            ARPHRD_LOCALTLK => Localtalk,
            ARPHRD_FDDI => Fddi,
            ARPHRD_BIF => Bif,
            ARPHRD_SIT => Sit,
            ARPHRD_IPDDP => IpDdp,
            ARPHRD_IPGRE => IpGre,
            ARPHRD_PIMREG => PimReg,
            ARPHRD_HIPPI => HiPPI,
            ARPHRD_ASH => Ash,
            ARPHRD_ECONET => Econet,
            ARPHRD_IRDA => IrDA,
            ARPHRD_FCPP => Fcpp,
            ARPHRD_FCAL => Fcal,
            ARPHRD_FCPL => Fcpl,
            ARPHRD_FCFABRIC => FcFabric,
            ARPHRD_IEEE802_TR => Ieee802TR,
            ARPHRD_IEEE80211 => Ieee80211,
            ARPHRD_IEEE80211_PRISM => Ieee80211Prism,
            ARPHRD_IEEE80211_RADIOTAP => Ieee80211RadioTap,
            ARPHRD_IEEE802154 => Ieee802154,
            ARPHRD_IEEE802154_MONITOR => Ieee802154Monitor,
            ARPHRD_PHONET => PhoNet,
            ARPHRD_PHONET_PIPE => PhoNetPipe,
            ARPHRD_CAIF => Caif,
            ARPHRD_IP6GRE => Ipv6Gre,
            ARPHRD_NETLINK => Netlink,
            ARPHRD_6LOWPAN => Ipv6OverLowpan,
            ARPHRD_VSOCKMON => VsockMonitor,
            ARPHRD_VOID => Void,
            ARPHRD_NONE => None,
            _ => std::option::Option::None?,
        })
    }
}
