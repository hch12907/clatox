use libc::*;

/// Address families used in network communication.
///
/// Corresponds to `AF_*` in libc.
#[doc(alias("AF_"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressFamily {
    Unspecified,

    /// Same as `AF_LOCAL`.
    Unix,

    /// IPv4
    Inet,

    /// Amateur X.25 protocol
    Ax25,

    /// Novell IPX
    Ipx,

    AppleTalk,

    /// AX.25 point-to-point protocol
    NetRom,

    Bridge,

    /// ATM Permanent Virtual Circuit
    AtmPvc,

    /// ITU-T X.25 (ISO-8208)
    X25,

    /// IPv6
    Inet6,

    Rose,

    DecNet,

    /// NetBIOS Extended User Interface
    NetBEUI,

    Security,
    Key,
    Netlink,
    Packet,
    Ash,
    Econet,

    /// ATM Switched Virtual Circuits
    AtmSvc,

    /// Reliable Datagram Sockets
    Rds,

    Sna,
    IrDA,
    PPPoX,
    WanPipe,
    Llc,
    Can,
    Tipc,
    Bluetooth,
    Iucv,
    RxRPC,
    Isdn,
    Phonet,
    Ieee802154,
    Caif,
    Alg,
    Nfc,
    Vsock,
    Xdp,
}

impl AddressFamily {
    pub fn raw_value(&self) -> u8 {
        use AddressFamily::*;

        let value = match self {
            Unspecified => AF_UNSPEC,
            Unix => AF_UNIX,
            Inet => AF_INET,
            Ax25 => AF_AX25,
            Ipx => AF_IPX,
            AppleTalk => AF_APPLETALK,
            NetRom => AF_NETROM,
            Bridge => AF_BRIDGE,
            AtmPvc => AF_ATMPVC,
            X25 => AF_X25,
            Inet6 => AF_INET6,
            Rose => AF_ROSE,
            DecNet => AF_DECnet,
            NetBEUI => AF_NETBEUI,
            Security => AF_SECURITY,
            Key => AF_KEY,
            Netlink => AF_NETLINK,
            Packet => AF_PACKET,
            Ash => AF_ASH,
            Econet => AF_ECONET,
            AtmSvc => AF_ATMSVC,
            Rds => AF_RDS,
            Sna => AF_SNA,
            IrDA => AF_IRDA,
            PPPoX => AF_PPPOX,
            WanPipe => AF_WANPIPE,
            Llc => AF_LLC,
            Can => AF_CAN,
            Tipc => AF_TIPC,
            Bluetooth => AF_BLUETOOTH,
            Iucv => AF_IUCV,
            RxRPC => AF_RXRPC,
            Isdn => AF_ISDN,
            Phonet => AF_PHONET,
            Ieee802154 => AF_IEEE802154,
            Caif => AF_CAIF,
            Alg => AF_ALG,
            Nfc => AF_NFC,
            Vsock => AF_VSOCK,
            Xdp => AF_XDP,
        };

        value as u8
    }

    pub fn from_raw_value(value: u8) -> Option<Self> {
        use AddressFamily::*;

        #[allow(non_upper_case_globals)]
        Some(match value as i32 {
            AF_UNSPEC => Unspecified,
            AF_UNIX => Unix,
            AF_INET => Inet,
            AF_AX25 => Ax25,
            AF_IPX => Ipx,
            AF_APPLETALK => AppleTalk,
            AF_NETROM => NetRom,
            AF_BRIDGE => Bridge,
            AF_ATMPVC => AtmPvc,
            AF_X25 => X25,
            AF_INET6 => Inet6,
            AF_ROSE => Rose,
            AF_DECnet => DecNet,
            AF_NETBEUI => NetBEUI,
            AF_SECURITY => Security,
            AF_KEY => Key,
            AF_NETLINK => Netlink,
            AF_PACKET => Packet,
            AF_ASH => Ash,
            AF_ECONET => Econet,
            AF_ATMSVC => AtmSvc,
            AF_RDS => Rds,
            AF_SNA => Sna,
            AF_IRDA => IrDA,
            AF_PPPOX => PPPoX,
            AF_WANPIPE => WanPipe,
            AF_LLC => Llc,
            AF_CAN => Can,
            AF_TIPC => Tipc,
            AF_BLUETOOTH => Bluetooth,
            AF_IUCV => Iucv,
            AF_RXRPC => RxRPC,
            AF_ISDN => Isdn,
            AF_PHONET => Phonet,
            AF_IEEE802154 => Ieee802154,
            AF_CAIF => Caif,
            AF_ALG => Alg,
            AF_NFC => Nfc,
            AF_VSOCK => Vsock,
            AF_XDP => Xdp,
            _ => None?
        })
    }
}
