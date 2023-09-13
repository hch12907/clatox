use libc::*;

pub const RTPROT_GATED: u8 = 8;
pub const RTPROT_RA: u8 = 9;
pub const RTPROT_MRT: u8 = 10;
pub const RTPROT_ZEBRA: u8 = 11;
pub const RTPROT_BIRD: u8 = 12;
pub const RTPROT_DNROUTED: u8 = 13;
pub const RTPROT_XORP: u8 = 14;
pub const RTPROT_NTK: u8 = 15;
pub const RTPROT_DHCP: u8 = 16;
pub const RTPROT_MROUTED: u8 = 17;
pub const RTPROT_KEEPALIVED: u8 = 18;
pub const RTPROT_BABEL: u8 = 42;
pub const RTPROT_OPENR: u8 = 99;
pub const RTPROT_BGP: u8 = 186;
pub const RTPROT_ISIS: u8 = 187;
pub const RTPROT_OSPF: u8 = 188;
pub const RTPROT_RIP: u8 = 189;
pub const RTPROT_EIGRP: u8 = 192;

#[doc(alias("rtmsg", "RTPROT_"))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RouteProtocol {
    /// `RTPROT_UNSPEC`
    Unspecified,
    
    /// `RTPROT_REDIRECT`
    Redirect,
    
    /// `RTPROT_KERNEL`
    Kernel,
    
    /// `RTPROT_BOOT`
    Boot,
    
    /// `RTPROT_STATIC`
    Static,
    
    /// `RTPROT_GATED`
    Gated,
    
    /// `RTPROT_RA`
    RouteAdvertisement,
    
    /// `RTPROT_MRT`
    MeritMrt,
    
    /// `RTPROT_ZEBRA`
    Zebra,
    
    /// `RTPROT_BIRD`
    Bird,
    
    /// `RTPROT_DNROUTED`
    Dnrouted,
    
    /// `RTPROT_XORP`
    Xorp,
    
    /// `RTPROT_NTK`
    Netsukuku,
    
    /// `RTPROT_DHCP`
    Dhcp,
    
    /// `RTPROT_MROUTED`
    Mrouted,
    
    /// `RTPROT_KEEPALIVED`
    KeepAlived,
    
    /// `RTPROT_BABEL`
    Babel,
    
    /// `RTPROT_OPENR`
    OpenR,
    
    /// `RTPROT_BGP`
    Bgp,
    
    /// `RTPROT_ISIS`
    Isis,
    
    /// `RTPROT_OSPF`
    Ospf,
    
    /// `RTPROT_RIP`
    Rip,
    
    /// `RTPROT_EIGRP`
    EIgrp,
}

impl RouteProtocol {
    pub fn from_raw_value(value: u8) -> Option<Self> {
        use RouteProtocol::*;

        Some(match value {
            RTPROT_UNSPEC => Unspecified,
            RTPROT_REDIRECT => Redirect,
            RTPROT_KERNEL => Kernel,
            RTPROT_BOOT => Boot,
            RTPROT_STATIC => Static,
            RTPROT_GATED => Gated,
            RTPROT_RA => RouteAdvertisement,
            RTPROT_MRT => MeritMrt,
            RTPROT_ZEBRA => Zebra,
            RTPROT_BIRD => Bird,
            RTPROT_DNROUTED => Dnrouted,
            RTPROT_XORP => Xorp,
            RTPROT_NTK => Netsukuku,
            RTPROT_DHCP => Dhcp,
            RTPROT_MROUTED => Mrouted,
            RTPROT_KEEPALIVED => KeepAlived,
            RTPROT_BABEL => Babel,
            RTPROT_OPENR => OpenR,
            RTPROT_BGP => Bgp,
            RTPROT_ISIS => Isis,
            RTPROT_OSPF => Ospf,
            RTPROT_RIP => Rip,
            RTPROT_EIGRP => EIgrp,
            _ => None?
        })
    }

    pub fn raw_value(&self) -> u8 {
        use RouteProtocol::*;

        match self {
            Unspecified => RTPROT_UNSPEC,
            Redirect => RTPROT_REDIRECT,
            Kernel => RTPROT_KERNEL,
            Boot => RTPROT_BOOT,
            Static => RTPROT_STATIC,
            Gated => RTPROT_GATED,
            RouteAdvertisement => RTPROT_RA,
            MeritMrt => RTPROT_MRT,
            Zebra => RTPROT_ZEBRA,
            Bird => RTPROT_BIRD,
            Dnrouted => RTPROT_DNROUTED,
            Xorp => RTPROT_XORP,
            Netsukuku => RTPROT_NTK,
            Dhcp => RTPROT_DHCP,
            Mrouted => RTPROT_MROUTED,
            KeepAlived => RTPROT_KEEPALIVED,
            Babel => RTPROT_BABEL,
            OpenR => RTPROT_OPENR,
            Bgp => RTPROT_BGP,
            Isis => RTPROT_ISIS,
            Ospf => RTPROT_OSPF,
            Rip => RTPROT_RIP,
            EIgrp => RTPROT_EIGRP,
        }
    }
}
