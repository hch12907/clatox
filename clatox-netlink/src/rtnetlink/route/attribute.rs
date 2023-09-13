use libc::*;

use crate::netlink::{Attribute, RawAttribute};
use crate::utils;

#[doc(alias("rtmsg", "RTA_"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteAttribute {
    /// `RTA_UNSPEC`
    Unspecified(Vec<u8>),

    /// `RTA_DST`
    Destination(Vec<u8>),

    /// `RTA_SRC`
    Source(Vec<u8>),

    /// `RTA_IIF`
    IncomingInterface(i32),

    /// `RTA_OIF`
    OutgoingInterface(i32),

    /// `RTA_GATEWAY`
    Gateway(Vec<u8>),

    /// `RTA_PRIORITY`
    RoutePriority(i32),

    /// `RTA_PREFSRC`
    PreferredSource(Vec<u8>),

    /// `RTA_METRICS`
    Metrics(i32),

    /// `RTA_MULTIPATH`
    Multipath(Vec<u8>),

    /// `RTA_PROTOINFO`
    ProtocolInfo(Vec<u8>),

    /// `RTA_FLOW`
    Flow(i32),

    /// `RTA_CACHEINFO`
    CacheInfo(Vec<u8>),

    /// `RTA_SESSION` - no longer used
    Session(Vec<u8>),

    /// `RTA_MP_ALGO` - no longer used
    MultipathAlgorithm(Vec<u8>),

    /// `RTA_TABLE`
    Table(i32),

    /// `RTA_MARK`
    Mark(i32),

    /// `RTA_MFC_STATS`
    MfcStats(Vec<u8>),

    /// `RTA_VIA`
    Via(Vec<u8>),

    /// `RTA_NEWDST`
    NewDestination(Vec<u8>),

    /// `RTA_PREF`
    RouterPreference(u8),

    /// `RTA_ENCAP_TYPE`
    EncapsulationType(i16),

    /// `RTA_ENCAP`
    Encapsulation(Vec<u8>),

    /// `RTA_EXPIRES`
    Expires(u32),

    Other(u16, Vec<u8>)
}

impl RouteAttribute {
    pub fn serialize_into(&self, buffer: &mut Vec<u8>) {
        utils::serialize_attribute_into(buffer, |buffer| match self {
            RouteAttribute::Unspecified(content) => {
                buffer.extend(content.iter());
                RTA_UNSPEC
            }
            RouteAttribute::Destination(content) => {
                buffer.extend(content.iter());
                RTA_DST
            }
            RouteAttribute::Source(content) => {
                buffer.extend(content.iter());
                RTA_SRC
            }
            RouteAttribute::IncomingInterface(iif) => {
                buffer.extend(iif.to_ne_bytes().into_iter());
                RTA_IIF
            }
            RouteAttribute::OutgoingInterface(oif) => {
                buffer.extend(oif.to_ne_bytes().into_iter());
                RTA_OIF
            }
            RouteAttribute::Gateway(content) => {
                buffer.extend(content.iter());
                RTA_GATEWAY
            }
            RouteAttribute::RoutePriority(priority) => {
                buffer.extend(priority.to_ne_bytes().into_iter());
                RTA_PRIORITY
            }
            RouteAttribute::PreferredSource(content) => {
                buffer.extend(content.iter());
                RTA_PREFSRC
            }
            RouteAttribute::Metrics(metrics) => {
                buffer.extend(metrics.to_ne_bytes().into_iter());
                RTA_METRICS
            }
            RouteAttribute::Multipath(content) => {
                buffer.extend(content.iter());
                RTA_MULTIPATH
            }
            RouteAttribute::ProtocolInfo(content) => {
                buffer.extend(content.iter());
                RTA_PROTOINFO
            }
            RouteAttribute::Flow(flow) => {
                buffer.extend(flow.to_ne_bytes().into_iter());
                RTA_FLOW
            }
            RouteAttribute::CacheInfo(content) => {
                buffer.extend(content.iter());
                RTA_CACHEINFO
            }
            RouteAttribute::Session(content) => {
                buffer.extend(content.iter());
                RTA_SESSION
            }
            RouteAttribute::MultipathAlgorithm(content) => {
                buffer.extend(content.iter());
                RTA_MP_ALGO
            }
            RouteAttribute::Table(table) => {
                buffer.extend(table.to_ne_bytes().into_iter());
                RTA_TABLE
            }
            RouteAttribute::Mark(mark) => {
                buffer.extend(mark.to_ne_bytes().into_iter());
                RTA_MARK
            }
            RouteAttribute::MfcStats(content) => {
                buffer.extend(content.iter());
                RTA_MFC_STATS
            }
            RouteAttribute::Via(content) => {
                buffer.extend(content.iter());
                RTA_VIA
            }
            RouteAttribute::NewDestination(content) => {
                buffer.extend(content.iter());
                RTA_NEWDST
            }
            RouteAttribute::RouterPreference(pref) => {
                buffer.extend(pref.to_ne_bytes().into_iter());
                RTA_PREF
            }
            RouteAttribute::EncapsulationType(encap_type) => {
                buffer.extend(encap_type.to_ne_bytes().into_iter());
                RTA_ENCAP_TYPE
            }
            RouteAttribute::Encapsulation(content) => {
                buffer.extend(content.iter());
                RTA_ENCAP
            }
            RouteAttribute::Expires(exp) => {
                buffer.extend(exp.to_ne_bytes().into_iter());
                RTA_EXPIRES
            }
            RouteAttribute::Other(typ, content) => {
                buffer.extend(content.iter());
                *typ
            }
        })
    }
}

impl Attribute for RouteAttribute {
    fn from_raw(raw: RawAttribute) -> Option<Self> {
        let attr_type = raw.attr_type();
        let content = raw.into_payload();

        let attr = match attr_type {
            RTA_UNSPEC => RouteAttribute::Unspecified(content),
            RTA_DST => RouteAttribute::Destination(content),
            RTA_SRC => RouteAttribute::Source(content),
            RTA_IIF => { 
                let content = <[u8; 4]>::try_from(content).ok()?;
                let iif = i32::from_ne_bytes(content);
                RouteAttribute::IncomingInterface(iif)
            },
            RTA_OIF => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let oif = i32::from_ne_bytes(content);
                RouteAttribute::OutgoingInterface(oif)
            },
            RTA_GATEWAY => RouteAttribute::Gateway(content),
            RTA_PRIORITY => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let priority = i32::from_ne_bytes(content);
                RouteAttribute::RoutePriority(priority)
            },
            RTA_PREFSRC => RouteAttribute::PreferredSource(content),
            RTA_METRICS => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let metrics = i32::from_ne_bytes(content);
                RouteAttribute::Metrics(metrics)
            },
            RTA_MULTIPATH => RouteAttribute::Multipath(content),
            RTA_PROTOINFO => RouteAttribute::ProtocolInfo(content),
            RTA_FLOW => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let flow = i32::from_ne_bytes(content);
                RouteAttribute::Flow(flow)
            },
            RTA_CACHEINFO => RouteAttribute::CacheInfo(content),
            RTA_SESSION => RouteAttribute::Session(content),
            RTA_MP_ALGO => RouteAttribute::MultipathAlgorithm(content),
            RTA_TABLE => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let table = i32::from_ne_bytes(content);
                RouteAttribute::Table(table)
            },
            RTA_MARK => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                let mark = i32::from_ne_bytes(content);
                RouteAttribute::Mark(mark)
            },
            RTA_MFC_STATS => RouteAttribute::MfcStats(content),
            RTA_VIA => RouteAttribute::Via(content),
            RTA_NEWDST => RouteAttribute::NewDestination(content),
            RTA_PREF => { 
                let pref = *content.iter().next()?;
                RouteAttribute::RouterPreference(pref)
            },
            RTA_ENCAP_TYPE => { 
                let content = <[u8; 2]>::try_from(content).ok()?;
                let encap_type = i16::from_ne_bytes(content);
                RouteAttribute::EncapsulationType(encap_type)
            },
            RTA_ENCAP => RouteAttribute::Encapsulation(content),
            RTA_EXPIRES => { 
                let content = <[u8; 4]>::try_from(content).ok()?;
                let exp = u32::from_ne_bytes(content);
                RouteAttribute::Expires(exp)
            },
            typ => RouteAttribute::Other(typ, content),
        };

        Some(attr)
    }
}
