use libc::*;

use crate::utils::read_u16;

pub(crate) fn align_attribute_len(len: i32) -> i32 {
    // TODO: we shouldn't use NLA_ALIGNTO here.
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

/// Those attributes are to be used with `InterfaceInfoMessage`s.
///
/// TODO: The enum is complete, but some values are still untyped.
#[doc(alias("ifinfomsg", "ifi_type", "IFLA_"))]
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Stats(Vec<u8>),

    /// `IFLA_COST`
    Cost(Vec<u8>),

    /// `IFLA_PRIORITY`
    Priority(Vec<u8>),

    /// `IFLA_MASTER`
    Master(Vec<u8>),

    /// `IFLA_WIRELESS`
    Wireless(Vec<u8>),

    /// `IFLA_PROTINFO`
    ProtoInfo(Vec<u8>),

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
    LinkInfo(Vec<u8>),

    /// `IFLA_NET_NS_PID`
    NetNamespacePid(Vec<u8>),

    /// `IFLA_IFALIAS`
    InterfaceAlias(Vec<u8>),

    /// `IFLA_NUM_VF`
    NumVf(u32),

    /// `IFLA_VFINFO_LIST`
    VfInfoList(Vec<u8>),

    /// `IFLA_STATS64`
    Stats64(Vec<u8>),

    /// `IFLA_VF_PORTS`
    VfPorts(Vec<u8>),

    /// `IFLA_PORT_SELF`
    PortSelf(Vec<u8>),

    /// `IFLA_AF_SPEC`
    AddressFamilySpecific(Vec<u8>),

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
    ProtoDown(Vec<u8>),

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
    ProtoDownReason(Vec<u8>),

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
        let original_len = buffer.len();

        // Push a length of zero into the buffer first
        buffer.extend(0u16.to_be_bytes().into_iter());

        // And then a type of zero second
        buffer.extend(0u16.to_be_bytes().into_iter());

        let attr_type;

        match self {
            InterfaceInfoAttribute::Unspecified(unspec) => {
                attr_type = IFLA_UNSPEC;
                buffer.extend(unspec.iter().cloned());
            }

            InterfaceInfoAttribute::Address(addr) => {
                attr_type = IFLA_ADDRESS;
                buffer.extend(addr.iter().cloned());
            }

            InterfaceInfoAttribute::Broadcast(addr) => {
                attr_type = IFLA_BROADCAST;
                buffer.extend(addr.iter().cloned());
            }

            InterfaceInfoAttribute::MTU(mtu) => {
                attr_type = IFLA_MTU;
                buffer.extend(mtu.to_ne_bytes().into_iter());
            }

            InterfaceInfoAttribute::Link(link) => {
                attr_type = IFLA_LINK;
                buffer.extend(link.to_ne_bytes().into_iter());
            }

            InterfaceInfoAttribute::InterfaceName(name) => {
                attr_type = IFLA_IFNAME;
                buffer.extend(name.bytes());
                buffer.push(0u8); // zero terminated string
            }

            InterfaceInfoAttribute::QueueDiscipline(queue) => {
                attr_type = IFLA_QDISC;
                buffer.extend(queue.bytes());
                buffer.push(0u8); // zero terminated string
            }

            InterfaceInfoAttribute::Stats(stats) => {
                attr_type = IFLA_STATS;
                buffer.extend(stats.iter().cloned());
            }

            InterfaceInfoAttribute::Cost(content) => {
                attr_type = IFLA_COST;
                buffer.extend(content.iter().cloned());
            }

            InterfaceInfoAttribute::Priority(content) => {
                attr_type = IFLA_PRIORITY;
                buffer.extend(content.iter().cloned());
            }

            InterfaceInfoAttribute::Master(content) => {
                attr_type = IFLA_MASTER;
                buffer.extend(content.iter().cloned());
            }

            InterfaceInfoAttribute::Wireless(content) => {
                attr_type = IFLA_WIRELESS;
                buffer.extend(content.iter().cloned());
            }

            InterfaceInfoAttribute::ProtoInfo(content) => {
                attr_type = IFLA_PROTINFO;
                buffer.extend(content.iter().cloned());
            }

            InterfaceInfoAttribute::TxQueueLength(content) => {
                attr_type = IFLA_TXQLEN;
                buffer.extend(content.to_ne_bytes().into_iter());
            }

            InterfaceInfoAttribute::Map(content) => {
                attr_type = IFLA_MAP;
                buffer.extend(content.iter().cloned());
            }

            InterfaceInfoAttribute::Weight(content) => {
                attr_type = IFLA_WEIGHT;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::OperationalState(content) => {
                attr_type = IFLA_OPERSTATE;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::LinkMode(content) => {
                attr_type = IFLA_LINKMODE;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::LinkInfo(content) => {
                attr_type = IFLA_LINKINFO;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::NetNamespacePid(content) => {
                attr_type = IFLA_NET_NS_PID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::InterfaceAlias(content) => {
                attr_type = IFLA_IFALIAS;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::NumVf(content) => {
                attr_type = IFLA_NUM_VF;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::VfInfoList(content) => {
                attr_type = IFLA_VFINFO_LIST;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::Stats64(content) => {
                attr_type = IFLA_STATS64;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::VfPorts(content) => {
                attr_type = IFLA_VF_PORTS;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::PortSelf(content) => {
                attr_type = IFLA_PORT_SELF;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::AddressFamilySpecific(content) => {
                attr_type = IFLA_AF_SPEC;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::Group(content) => {
                attr_type = IFLA_GROUP;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::NetNamespaceFd(content) => {
                attr_type = IFLA_NET_NS_FD;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::ExtMask(content) => {
                attr_type = IFLA_EXT_MASK;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::Promiscuity(content) => {
                attr_type = IFLA_PROMISCUITY;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::NumTxQueues(content) => {
                attr_type = IFLA_NUM_TX_QUEUES;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::NumRxQueues(content) => {
                attr_type = IFLA_NUM_RX_QUEUES;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::Carrier(content) => {
                attr_type = IFLA_CARRIER;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::PhysicalPortId(content) => {
                attr_type = IFLA_PHYS_PORT_ID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::CarrierChanges(content) => {
                attr_type = IFLA_CARRIER_CHANGES;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::PhysicalSwitchId(content) => {
                attr_type = IFLA_PHYS_SWITCH_ID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::LinkNetNamespaceId(content) => {
                attr_type = IFLA_LINK_NETNSID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::PhysicalPortName(content) => {
                attr_type = IFLA_PHYS_PORT_NAME;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::ProtoDown(content) => {
                attr_type = IFLA_PROTO_DOWN;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::GsoMaxSegments(content) => {
                attr_type = IFLA_GSO_MAX_SEGS;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::GsoMaxSize(content) => {
                attr_type = IFLA_GSO_MAX_SIZE;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::Pad(content) => {
                attr_type = IFLA_PAD;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::Xdp(content) => {
                attr_type = IFLA_XDP;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::Event(content) => {
                attr_type = IFLA_EVENT;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::NewNetNamespaceId(content) => {
                attr_type = IFLA_NEW_NETNSID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::InterfaceNetNamespaceId(content) => {
                attr_type = IFLA_IF_NETNSID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::TargetNetNamespaceId(content) => {
                attr_type = IFLA_TARGET_NETNSID;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::CarrierUpCount(content) => {
                attr_type = IFLA_CARRIER_UP_COUNT;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::CarrierDownCount(content) => {
                attr_type = IFLA_CARRIER_DOWN_COUNT;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::NewInterfaceIndex(content) => {
                attr_type = IFLA_NEW_IFINDEX;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::MinMTU(content) => {
                attr_type = IFLA_MIN_MTU;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::MaxMTU(content) => {
                attr_type = IFLA_MAX_MTU;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::PropertiesList(content) => {
                attr_type = IFLA_PROP_LIST;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::AlternativeName(content) => {
                attr_type = IFLA_ALT_IFNAME;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::PermanentAddress(content) => {
                attr_type = IFLA_PERM_ADDRESS;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::ProtoDownReason(content) => {
                attr_type = IFLA_PROTO_DOWN_REASON;
                buffer.extend(content.iter().cloned())
            }

            InterfaceInfoAttribute::ParentDeviceName(content) => {
                attr_type = IFLA_PARENT_DEV_NAME;
                buffer.extend(content.bytes());
                buffer.push(0u8); // zero-terminated string
            }

            InterfaceInfoAttribute::ParentDeviceBusName(content) => {
                attr_type = IFLA_PARENT_DEV_BUS_NAME;
                buffer.extend(content.bytes());
                buffer.push(0u8); // zero-terminated string
            }

            InterfaceInfoAttribute::GroMaxSize(content) => {
                attr_type = IFLA_GRO_MAX_SIZE;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::TsoMaxSize(content) => {
                attr_type = IFLA_TSO_MAX_SIZE;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::TsoMaxSegments(content) => {
                attr_type = IFLA_TSO_MAX_SEGS;
                buffer.extend(content.to_ne_bytes().into_iter())
            }

            InterfaceInfoAttribute::AllMulti(content) => {
                attr_type = IFLA_ALLMULTI;
                buffer.extend(content.to_ne_bytes().into_iter())
            }
        };

        for (i, byte) in (buffer.len() as u16).to_ne_bytes().into_iter().enumerate() {
            buffer[i] = byte;
        }

        for (i, byte) in attr_type.to_ne_bytes().into_iter().enumerate() {
            buffer[i + 2] = byte;
        }

        // Align the buffer to RTA_ALIGNTO bytes by filling in zeroes
        let length = (buffer.len() - original_len) as i32;
        for _ in 0..(align_attribute_len(length) - length) {
            buffer.push(0u8);
        }
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut buffer = Vec::with_capacity(8);
        self.serialize_into(&mut buffer);
        buffer.into_boxed_slice()
    }

    pub fn deserialize(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut iter = bytes.iter();

        let attr_len = read_u16(iter.by_ref().take(2).cloned()).unwrap();
        let attr_type = read_u16(iter.by_ref().take(2).cloned()).unwrap();
        let aligned_attr_len = align_attribute_len(attr_len as i32) as usize;
        let mut content = iter
            .by_ref()
            .take(aligned_attr_len - 4)
            .cloned()
            .collect::<Vec<_>>();

        // This will happen when there's an error.
        if attr_len == 0 {
            return None;
        }

        content.truncate(attr_len as usize - 4);

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
            IFLA_STATS => InterfaceInfoAttribute::Stats(content),
            IFLA_COST => InterfaceInfoAttribute::Cost(content),
            IFLA_PRIORITY => InterfaceInfoAttribute::Priority(content),
            IFLA_MASTER => InterfaceInfoAttribute::Master(content),
            IFLA_WIRELESS => InterfaceInfoAttribute::Wireless(content),
            IFLA_PROTINFO => InterfaceInfoAttribute::ProtoInfo(content),
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
            IFLA_LINKINFO => InterfaceInfoAttribute::LinkInfo(content),
            IFLA_NET_NS_PID => InterfaceInfoAttribute::NetNamespacePid(content),
            IFLA_IFALIAS => InterfaceInfoAttribute::InterfaceAlias(content),
            IFLA_NUM_VF => {
                let content = <[u8; 4]>::try_from(content).ok()?;
                InterfaceInfoAttribute::NumVf(u32::from_ne_bytes(content))
            }
            IFLA_VFINFO_LIST => InterfaceInfoAttribute::VfInfoList(content),
            IFLA_STATS64 => InterfaceInfoAttribute::Stats64(content),
            IFLA_VF_PORTS => Self::VfPorts(content),
            IFLA_PORT_SELF => Self::PortSelf(content),
            IFLA_AF_SPEC => Self::AddressFamilySpecific(content),
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
            IFLA_PROTO_DOWN => Self::ProtoDown(content),
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
            IFLA_PROTO_DOWN_REASON => Self::ProtoDownReason(content),
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

        Some((attr, aligned_attr_len))
    }
}
