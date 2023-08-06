use libc::*;

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
/// TODO: The enum is incomplete.
#[doc(alias("ifinfomsg", "ifi_type", "ARPHRD_"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpHardware {
    NetRom = ARPHRD_NETROM,
    Ethernet = ARPHRD_ETHER,
    ExperimentalEthernet = ARPHRD_EETHER,
    Ax25 = ARPHRD_AX25,
    Pronet = ARPHRD_PRONET,
    Chaos = ARPHRD_CHAOS,
    Ieee802 = ARPHRD_IEEE802,
    ARCnet = ARPHRD_ARCNET,
    AppleTalk = ARPHRD_APPLETLK,
    Dlci = ARPHRD_DLCI,
    Atm = ARPHRD_ATM,
    Metricom = ARPHRD_METRICOM,
    Ieee1394 = ARPHRD_IEEE1394,
    Eui64 = ARPHRD_EUI64,
    InfiniBand = ARPHRD_INFINIBAND,
    Loopback = ARPHRD_LOOPBACK,
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
            ARCnet => ARPHRD_ARCNET,
            AppleTalk => ARPHRD_APPLETLK,
            Dlci => ARPHRD_DLCI,
            Atm => ARPHRD_ATM,
            Metricom => ARPHRD_METRICOM,
            Ieee1394 => ARPHRD_IEEE1394,
            Eui64 => ARPHRD_EUI64,
            InfiniBand => ARPHRD_INFINIBAND,
            Loopback => ARPHRD_LOOPBACK,
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
             ARPHRD_ARCNET => ARCnet,
             ARPHRD_APPLETLK => AppleTalk,
             ARPHRD_DLCI => Dlci,
             ARPHRD_ATM => Atm,
             ARPHRD_METRICOM => Metricom,
             ARPHRD_IEEE1394 => Ieee1394,
             ARPHRD_EUI64 => Eui64,
             ARPHRD_INFINIBAND => InfiniBand,
             ARPHRD_LOOPBACK => Loopback,
             ARPHRD_VOID => Void,
             ARPHRD_NONE => None,
             _ => std::option::Option::None?
        })
    }
}
