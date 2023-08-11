use libc::*;

#[doc(alias("rtmsg", "RTN_"))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RouteType {
    Unspecified,
    Unicast,
    Local,
    Broadcast,
    Anycast,
    Multicast,
    Blackhole,
    Unreachable,
    Prohibit,
    Throw,
    Nat,
    ExternalResolve,
}

impl RouteType {
    pub fn from_raw_value(value: u8) -> Option<Self> {
        use RouteType::*;

        Some(match value {
            RTN_UNSPEC => Unspecified,
            RTN_UNICAST => Unicast,
            RTN_LOCAL => Local,
            RTN_BROADCAST => Broadcast,
            RTN_ANYCAST => Anycast,
            RTN_MULTICAST => Multicast,
            RTN_BLACKHOLE => Blackhole,
            RTN_UNREACHABLE => Unreachable,
            RTN_PROHIBIT => Prohibit,
            RTN_THROW => Throw,
            RTN_NAT => Nat,
            RTN_XRESOLVE => ExternalResolve,
            _ => None?
        })
    }

    pub fn raw_value(&self) -> u8 {
        use RouteType::*;

        match self {
            Unspecified => RTN_UNSPEC,
            Unicast => RTN_UNICAST,
            Local => RTN_LOCAL,
            Broadcast => RTN_BROADCAST,
            Anycast => RTN_ANYCAST,
            Multicast => RTN_MULTICAST,
            Blackhole => RTN_BLACKHOLE,
            Unreachable => RTN_UNREACHABLE,
            Prohibit => RTN_PROHIBIT,
            Throw => RTN_THROW,
            Nat => RTN_NAT,
            ExternalResolve => RTN_XRESOLVE,
        }
    }
}
