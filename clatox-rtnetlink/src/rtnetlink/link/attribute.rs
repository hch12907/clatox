use libc::*;

use std::mem::{size_of, transmute};

use crate::netlink::{Attribute, RawAttribute, RawAttributeIter};
use crate::utils;

use super::stats::{InterfaceStats, InterfaceStats64};
use super::{AddressFamilySpecific, LinkInfo};

/// Those attributes are to be used with `InterfaceInfoMessage`s. They
/// correspond to `IFLA_*` in libc.
///
/// TODO: The enum is complete, but some values are still untyped.
#[doc(alias("ifinfomsg", "ifi_type", "IFLA_"))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InterfaceInfoAttribute {
    /// `IFLA_UNSPEC`
    Unspecified(Vec<u8>),

    /// `IFLA_ADDRESS` - the L2 address of an interface.
    Address(Vec<u8>),

    /// `IFLA_BROADCAST` - the L2 broadcast address of an interface.
    Broadcast(Vec<u8>),

    /// `IFLA_IFNAME` - the name of an interface.
    InterfaceName(String),

    /// `IFLA_MTU` - the MTU of an interface.
    MTU(u32),

    /// `IFLA_LINK` - the link type of an interface.
    Link(i32),

    /// `IFLA_QDISC` - the queue discipline of an interface.
    QueueDiscipline(String),

    /// `IFLA_STATS` - interface statistics.
    Stats(InterfaceStats),

    /// `IFLA_COST`
    Cost(Vec<u8>),

    /// `IFLA_PRIORITY`
    Priority(Vec<u8>),

    /// `IFLA_MASTER`
    Master(Vec<u8>),

    /// `IFLA_WIRELESS`
    Wireless(Vec<u8>),

    /// `IFLA_PROTINFO`
    ProtocolInfo(Vec<u8>),

    /// `IFLA_TXQLEN`
    TxQueueLength(u32),

    /// `IFLA_MAP`
    Map(Vec<u8>),

    /// `IFLA_WEIGHT`
    Weight(Vec<u8>),

    /// `IFLA_OPERSTATE`
    OperationalState(u8),

    /// `IFLA_LINKMODE`
    LinkMode(u8),

    /// `IFLA_LINKINFO`
    LinkInfo(Vec<LinkInfo>),

    /// `IFLA_NET_NS_PID`
    NetNamespacePid(Vec<u8>),

    /// `IFLA_IFALIAS`
    InterfaceAlias(String),

    /// `IFLA_NUM_VF`
    NumVf(u32),

    /// `IFLA_VFINFO_LIST`
    VfInfoList(Vec<u8>),

    /// `IFLA_STATS64`
    Stats64(InterfaceStats64),

    /// `IFLA_VF_PORTS`
    VfPorts(Vec<u8>),

    /// `IFLA_PORT_SELF`
    PortSelf(Vec<u8>),

    /// `IFLA_AF_SPEC`
    AddressFamilySpecific(Vec<AddressFamilySpecific>),

    /// `IFLA_GROUP`
    Group(u32),

    /// `IFLA_NET_NS_FD`
    NetNamespaceFd(Vec<u8>),

    /// `IFLA_EXT_MASK`
    ExtMask(Vec<u8>),

    /// `IFLA_PROMISCUITY`
    Promiscuity(u32),

    /// `IFLA_NUM_TX_QUEUES`
    NumTxQueues(u32),

    /// `IFLA_NUM_RX_QUEUES`
    NumRxQueues(u32),

    /// `IFLA_CARRIER`
    Carrier(Vec<u8>),

    /// `IFLA_PHYS_PORT_ID`
    PhysicalPortId(Vec<u8>),

    /// `IFLA_CARRIER_CHANGES`
    CarrierChanges(u32),

    /// `IFLA_PHYS_SWITCH_ID`
    PhysicalSwitchId(Vec<u8>),

    /// `IFLA_LINK_NETNSID`
    LinkNetNamespaceId(Vec<u8>),

    /// `IFLA_PHYS_PORT_NAME`
    PhysicalPortName(Vec<u8>),

    /// `IFLA_PROTO_DOWN`
    ProtocolDown(Vec<u8>),

    /// `IFLA_GSO_MAX_SEGS`
    GsoMaxSegments(u32),

    /// `IFLA_GSO_MAX_SIZE`
    GsoMaxSize(u32),

    /// `IFLA_PAD`
    Pad(Vec<u8>),

    /// `IFLA_XDP`
    Xdp(Vec<u8>),

    /// `IFLA_EVENT`
    Event(Vec<u8>),

    /// `IFLA_NEW_NETNSID`
    NewNetNamespaceId(Vec<u8>),

    /// `IFLA_IF_NETNSID`
    InterfaceNetNamespaceId(Vec<u8>),

    /// `IFLA_TARGET_NETNSID`
    TargetNetNamespaceId(Vec<u8>),

    /// `IFLA_CARRIER_UP_COUNT`
    CarrierUpCount(u32),

    /// `IFLA_CARRIER_DOWN_COUNT`
    CarrierDownCount(u32),

    /// `IFLA_NEW_IFINDEX`
    NewInterfaceIndex(i32),

    /// `IFLA_MIN_MTU`
    MinMTU(u32),

    /// `IFLA_MAX_MTU`
    MaxMTU(u32),

    /// `IFLA_PROP_LIST`
    PropertiesList(Vec<u8>),

    /// `IFLA_ALT_IFNAME`
    AlternativeName(Vec<u8>),

    /// `IFLA_PERM_ADDRESS`
    PermanentAddress(Vec<u8>),

    /// `IFLA_PROTO_DOWN_REASON`
    ProtocolDownReason(Vec<u8>),

    /// `IFLA_PARENT_DEV_NAME`
    ParentDeviceName(String),

    /// `IFLA_PARENT_DEV_BUS_NAME`
    ParentDeviceBusName(String),

    /// `IFLA_GRO_MAX_SIZE`
    GroMaxSize(u32),

    /// `IFLA_TSO_MAX_SIZE`
    TsoMaxSize(u32),

    /// `IFLA_TSO_MAX_SEGS`
    TsoMaxSegments(u32),

    /// `IFLA_ALLMULTI`
    AllMulti(u32),
}

impl InterfaceInfoAttribute {
    pub fn serialize_into(&self, buffer: &mut Vec<u8>) {
        utils::serialize_attribute_into(buffer, |buffer| match self {
            InterfaceInfoAttribute::Unspecified(unspec) => {
                buffer.extend(unspec.iter().cloned());
                IFLA_UNSPEC
            }

            InterfaceInfoAttribute::Address(addr) => {
                buffer.extend(addr.iter().cloned());
                IFLA_ADDRESS
            }

            InterfaceInfoAttribute::Broadcast(addr) => {
                buffer.extend(addr.iter().cloned());
                IFLA_BROADCAST
            }

            InterfaceInfoAttribute::MTU(mtu) => {
                buffer.extend(mtu.to_ne_bytes().into_iter());
                IFLA_MTU
            }

            InterfaceInfoAttribute::Link(link) => {
                buffer.extend(link.to_ne_bytes().into_iter());
                IFLA_LINK
            }

            InterfaceInfoAttribute::InterfaceName(name) => {
                buffer.extend(name.bytes());
                buffer.push(0u8); // zero terminated string
                IFLA_IFNAME
            }

            InterfaceInfoAttribute::QueueDiscipline(queue) => {
                buffer.extend(queue.bytes());
                buffer.push(0u8); // zero terminated string
                IFLA_QDISC
            }

            InterfaceInfoAttribute::Stats(stats) => {
                // SAFETY: It is safe to transmute InterfaceStats into a byte array as
                // the type does not contain any paddings.
                let bytes = unsafe {
                    transmute::<InterfaceStats, [u8; size_of::<InterfaceStats>()]>(stats.clone())
                };
                buffer.extend(bytes.iter().cloned());
                IFLA_STATS
            }

            InterfaceInfoAttribute::Cost(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_COST
            }

            InterfaceInfoAttribute::Priority(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PRIORITY
            }

            InterfaceInfoAttribute::Master(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_MASTER
            }

            InterfaceInfoAttribute::Wireless(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_WIRELESS
            }

            InterfaceInfoAttribute::ProtocolInfo(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PROTINFO
            }

            InterfaceInfoAttribute::TxQueueLength(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_TXQLEN
            }

            InterfaceInfoAttribute::Map(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_MAP
            }

            InterfaceInfoAttribute::Weight(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_WEIGHT
            }

            InterfaceInfoAttribute::OperationalState(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_OPERSTATE
            }

            InterfaceInfoAttribute::LinkMode(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_LINKMODE
            }

            InterfaceInfoAttribute::LinkInfo(infos) => {
                for info in infos {
                    info.serialize_into(buffer);
                }
                IFLA_LINKINFO
            }

            InterfaceInfoAttribute::NetNamespacePid(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_NET_NS_PID
            }

            InterfaceInfoAttribute::InterfaceAlias(alias) => {
                buffer.extend(alias.bytes());
                buffer.push(0u8); // zero terminated string
                IFLA_IFALIAS
            }

            InterfaceInfoAttribute::NumVf(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_NUM_VF
            }

            InterfaceInfoAttribute::VfInfoList(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_VFINFO_LIST
            }

            InterfaceInfoAttribute::Stats64(stats) => {
                // SAFETY: It is safe to transmute InterfaceStats64 into a byte array as
                // the type does not contain any paddings.
                let bytes = unsafe {
                    transmute::<InterfaceStats64, [u8; size_of::<InterfaceStats64>()]>(
                        stats.clone(),
                    )
                };
                buffer.extend(bytes.iter().cloned());
                IFLA_STATS64
            }

            InterfaceInfoAttribute::VfPorts(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_VF_PORTS
            }

            InterfaceInfoAttribute::PortSelf(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PORT_SELF
            }

            InterfaceInfoAttribute::AddressFamilySpecific(specs) => {
                for spec in specs {
                    spec.serialize_in(buffer);
                }
                IFLA_AF_SPEC
            }

            InterfaceInfoAttribute::Group(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_GROUP
            }

            InterfaceInfoAttribute::NetNamespaceFd(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_NET_NS_FD
            }

            InterfaceInfoAttribute::ExtMask(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_EXT_MASK
            }

            InterfaceInfoAttribute::Promiscuity(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_PROMISCUITY
            }

            InterfaceInfoAttribute::NumTxQueues(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_NUM_TX_QUEUES
            }

            InterfaceInfoAttribute::NumRxQueues(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_NUM_RX_QUEUES
            }

            InterfaceInfoAttribute::Carrier(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_CARRIER
            }

            InterfaceInfoAttribute::PhysicalPortId(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PHYS_PORT_ID
            }

            InterfaceInfoAttribute::CarrierChanges(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_CARRIER_CHANGES
            }

            InterfaceInfoAttribute::PhysicalSwitchId(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PHYS_SWITCH_ID
            }

            InterfaceInfoAttribute::LinkNetNamespaceId(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_LINK_NETNSID
            }

            InterfaceInfoAttribute::PhysicalPortName(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PHYS_PORT_NAME
            }

            InterfaceInfoAttribute::ProtocolDown(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PROTO_DOWN
            }

            InterfaceInfoAttribute::GsoMaxSegments(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_GSO_MAX_SEGS
            }

            InterfaceInfoAttribute::GsoMaxSize(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_GSO_MAX_SIZE
            }

            InterfaceInfoAttribute::Pad(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PAD
            }

            InterfaceInfoAttribute::Xdp(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_XDP
            }

            InterfaceInfoAttribute::Event(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_EVENT
            }

            InterfaceInfoAttribute::NewNetNamespaceId(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_NEW_NETNSID
            }

            InterfaceInfoAttribute::InterfaceNetNamespaceId(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_IF_NETNSID
            }

            InterfaceInfoAttribute::TargetNetNamespaceId(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_TARGET_NETNSID
            }

            InterfaceInfoAttribute::CarrierUpCount(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_CARRIER_UP_COUNT
            }

            InterfaceInfoAttribute::CarrierDownCount(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_CARRIER_DOWN_COUNT
            }

            InterfaceInfoAttribute::NewInterfaceIndex(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_NEW_IFINDEX
            }

            InterfaceInfoAttribute::MinMTU(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_MIN_MTU
            }

            InterfaceInfoAttribute::MaxMTU(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_MAX_MTU
            }

            InterfaceInfoAttribute::PropertiesList(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PROP_LIST
            }

            InterfaceInfoAttribute::AlternativeName(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_ALT_IFNAME
            }

            InterfaceInfoAttribute::PermanentAddress(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PERM_ADDRESS
            }

            InterfaceInfoAttribute::ProtocolDownReason(content) => {
                buffer.extend(content.iter().cloned());
                IFLA_PROTO_DOWN_REASON
            }

            InterfaceInfoAttribute::ParentDeviceName(content) => {
                buffer.extend(content.bytes());
                buffer.push(0u8); // zero-terminated string
                IFLA_PARENT_DEV_NAME
            }

            InterfaceInfoAttribute::ParentDeviceBusName(content) => {
                buffer.extend(content.bytes());
                buffer.push(0u8); // zero-terminated string
                IFLA_PARENT_DEV_BUS_NAME
            }

            InterfaceInfoAttribute::GroMaxSize(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_GRO_MAX_SIZE
            }

            InterfaceInfoAttribute::TsoMaxSize(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_TSO_MAX_SIZE
            }

            InterfaceInfoAttribute::TsoMaxSegments(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_TSO_MAX_SEGS
            }

            InterfaceInfoAttribute::AllMulti(content) => {
                buffer.extend(content.to_ne_bytes().into_iter());
                IFLA_ALLMULTI
            }
        });
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut buffer = Vec::with_capacity(8);
        self.serialize_into(&mut buffer);
        buffer.into_boxed_slice()
    }
}

impl Attribute for InterfaceInfoAttribute {
    fn from_raw(raw: RawAttribute) -> Option<Self> {
        let attr_type = raw.attr_type();
        let content = raw.into_payload();

        let attr = match attr_type {
            IFLA_UNSPEC => InterfaceInfoAttribute::Unspecified(content),
            IFLA_ADDRESS => InterfaceInfoAttribute::Address(content),
            IFLA_BROADCAST => InterfaceInfoAttribute::Broadcast(content),
            IFLA_IFNAME => {
                let mut content = String::from_utf8(content).ok()?;
                let zero = content.pop();
                debug_assert!(zero == Some('\0'));
                InterfaceInfoAttribute::InterfaceName(content)
            }
            IFLA_MTU => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::MTU(u32::from_ne_bytes(content))
            }
            IFLA_LINK => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::Link(i32::from_ne_bytes(content))
            }
            IFLA_QDISC => {
                let mut content = String::from_utf8(content).ok()?;
                let zero = content.pop();
                debug_assert!(zero == Some('\0'));
                InterfaceInfoAttribute::QueueDiscipline(content)
            }
            IFLA_STATS => {
                let content = <[u8; size_of::<InterfaceStats>()]>::try_from(content).ok()?;
                // SAFETY: InterfaceStats is a plain-old-data struct and contains no
                // paddings. We trust Rtnetlink to give us the correct values here, but
                // even if the values are wrong, there won't be a memory corruption or UB.
                let stats = unsafe {
                    transmute::<[u8; size_of::<InterfaceStats>()], InterfaceStats>(content)
                };
                InterfaceInfoAttribute::Stats(stats)
            }
            IFLA_COST => InterfaceInfoAttribute::Cost(content),
            IFLA_PRIORITY => InterfaceInfoAttribute::Priority(content),
            IFLA_MASTER => InterfaceInfoAttribute::Master(content),
            IFLA_WIRELESS => InterfaceInfoAttribute::Wireless(content),
            IFLA_PROTINFO => InterfaceInfoAttribute::ProtocolInfo(content),
            IFLA_TXQLEN => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::TxQueueLength(u32::from_ne_bytes(content))
            }
            IFLA_MAP => InterfaceInfoAttribute::Map(content),
            IFLA_WEIGHT => InterfaceInfoAttribute::Weight(content),
            IFLA_OPERSTATE => {
                let content = content.get(0)?;
                InterfaceInfoAttribute::OperationalState(*content)
            }
            IFLA_LINKMODE => {
                let content = content.get(0)?;
                InterfaceInfoAttribute::LinkMode(*content)
            }
            IFLA_LINKINFO => {
                let infos = RawAttributeIter::new(content.iter().cloned())
                    .map(LinkInfo::from_raw)
                    .try_collect()?;

                InterfaceInfoAttribute::LinkInfo(infos)
            }
            IFLA_NET_NS_PID => InterfaceInfoAttribute::NetNamespacePid(content),
            IFLA_IFALIAS => {
                let mut content = String::from_utf8(content).ok()?;
                let zero = content.pop();
                debug_assert!(zero == Some('\0'));
                InterfaceInfoAttribute::InterfaceAlias(content)
            }
            IFLA_NUM_VF => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::NumVf(u32::from_ne_bytes(content))
            }
            IFLA_VFINFO_LIST => InterfaceInfoAttribute::VfInfoList(content),
            IFLA_STATS64 => {
                let content = <[u8; size_of::<InterfaceStats64>()]>::try_from(content).ok()?;
                // SAFETY: InterfaceStats is a plain-old-data struct and contains no
                // paddings. We trust Rtnetlink to give us the correct values here, but
                // even if the values are wrong, there won't be a memory corruption or UB.
                let stats = unsafe {
                    transmute::<[u8; size_of::<InterfaceStats64>()], InterfaceStats64>(content)
                };
                InterfaceInfoAttribute::Stats64(stats)
            }
            IFLA_VF_PORTS => Self::VfPorts(content),
            IFLA_PORT_SELF => Self::PortSelf(content),
            IFLA_AF_SPEC => {
                let specs = RawAttributeIter::new(content.iter().cloned())
                    .map(AddressFamilySpecific::from_raw)
                    .try_collect()?;

                InterfaceInfoAttribute::AddressFamilySpecific(specs)
            }
            IFLA_GROUP => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::Group(u32::from_ne_bytes(content))
            }
            IFLA_NET_NS_FD => Self::NetNamespaceFd(content),
            IFLA_EXT_MASK => Self::ExtMask(content),
            IFLA_PROMISCUITY => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::Promiscuity(u32::from_ne_bytes(content))
            }
            IFLA_NUM_TX_QUEUES => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::NumTxQueues(u32::from_ne_bytes(content))
            }
            IFLA_NUM_RX_QUEUES => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::NumRxQueues(u32::from_ne_bytes(content))
            }
            IFLA_CARRIER => Self::Carrier(content),
            IFLA_PHYS_PORT_ID => Self::PhysicalPortId(content),
            IFLA_CARRIER_CHANGES => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::CarrierChanges(u32::from_ne_bytes(content))
            }
            IFLA_PHYS_SWITCH_ID => Self::PhysicalSwitchId(content),
            IFLA_LINK_NETNSID => Self::LinkNetNamespaceId(content),
            IFLA_PHYS_PORT_NAME => Self::PhysicalPortName(content),
            IFLA_PROTO_DOWN => Self::ProtocolDown(content),
            IFLA_GSO_MAX_SEGS => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::GsoMaxSegments(u32::from_ne_bytes(content))
            }
            IFLA_GSO_MAX_SIZE => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::GsoMaxSize(u32::from_ne_bytes(content))
            }
            IFLA_PAD => Self::Pad(content),
            IFLA_XDP => Self::Xdp(content),
            IFLA_EVENT => Self::Event(content),
            IFLA_NEW_NETNSID => Self::NewNetNamespaceId(content),
            IFLA_IF_NETNSID => Self::InterfaceNetNamespaceId(content),
            IFLA_CARRIER_UP_COUNT => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::CarrierUpCount(u32::from_ne_bytes(content))
            }
            IFLA_CARRIER_DOWN_COUNT => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::CarrierDownCount(u32::from_ne_bytes(content))
            }
            IFLA_NEW_IFINDEX => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::NewInterfaceIndex(i32::from_ne_bytes(content))
            }
            IFLA_MIN_MTU => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::MinMTU(u32::from_ne_bytes(content))
            }
            IFLA_MAX_MTU => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::MaxMTU(u32::from_ne_bytes(content))
            }
            IFLA_PROP_LIST => Self::PropertiesList(content),
            IFLA_ALT_IFNAME => Self::AlternativeName(content),
            IFLA_PERM_ADDRESS => Self::PermanentAddress(content),
            IFLA_PROTO_DOWN_REASON => Self::ProtocolDownReason(content),
            IFLA_PARENT_DEV_NAME => {
                let mut content = String::from_utf8(content).ok()?;
                let zero = content.pop();
                debug_assert!(zero == Some('\0'));
                InterfaceInfoAttribute::ParentDeviceName(content)
            }
            IFLA_PARENT_DEV_BUS_NAME => {
                let mut content = String::from_utf8(content).ok()?;
                let zero = content.pop();
                debug_assert!(zero == Some('\0'));
                InterfaceInfoAttribute::ParentDeviceBusName(content)
            }
            IFLA_GRO_MAX_SIZE => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::GroMaxSize(u32::from_ne_bytes(content))
            }
            IFLA_TSO_MAX_SIZE => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::TsoMaxSize(u32::from_ne_bytes(content))
            }
            IFLA_TSO_MAX_SEGS => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::TsoMaxSegments(u32::from_ne_bytes(content))
            }
            IFLA_ALLMULTI => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::AllMulti(u32::from_ne_bytes(content))
            }
            x @ _ => {
                panic!("received unknown rtnetlink attribute 0x{:X}", x)
            }
        };

        Some(attr)
    }
}
