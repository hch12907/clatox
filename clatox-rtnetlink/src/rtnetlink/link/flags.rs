use libc::*;

/// An enum of all interface flags.
/// It corresponds to `IFF_*` in libc.
#[doc(alias("ifinfomsg", "ifi_flags", "IFF_"))]
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum InterfaceFlag {
    /// `IFF_UP`
    Up,
    
    /// `IFF_BROADCAST`
    Broadcast,
    
    /// `IFF_DEBUG`
    Debug,
    
    /// `IFF_LOOPBACK`
    Loopback,
    
    /// `IFF_POINTOPOINT`
    PointToPoint,
    
    /// `IFF_RUNNING`
    Running,
    
    /// `IFF_NOARP`
    NoArp,
    
    /// `IFF_PROMISC`
    Promiscuous,
    
    /// `IFF_NOTRAILERS`
    NoTrailers,
    
    /// `IFF_ALLMULTI`
    AllMulticast,
    
    /// `IFF_MASTER`
    Master,
    
    /// `IFF_SLAVE`
    Slave,
    
    /// `IFF_MULTICAST`
    Multicast,
    
    /// `IFF_PORTSEL`
    PortSelect,
    
    /// `IFF_AUTOMEDIA`
    AutoMedia,
    
    /// `IFF_DYNAMIC`
    Dynamic,
    
    /// `IFF_LOWER_UP`
    LowerUp,
    
    /// `IFF_DORMANT`
    Dormant,
    
    /// `IFF_ECHO`
    Echo,
}

impl InterfaceFlag {
    pub const fn raw_value(&self) -> u32 {
        use InterfaceFlag::*;

        let value = match self {
            Up => IFF_UP,
            Broadcast => IFF_BROADCAST,
            Debug => IFF_DEBUG,
            Loopback => IFF_LOOPBACK,
            PointToPoint => IFF_POINTOPOINT,
            Running => IFF_RUNNING,
            NoArp => IFF_NOARP,
            Promiscuous => IFF_PROMISC,
            NoTrailers => IFF_NOTRAILERS,
            AllMulticast => IFF_ALLMULTI,
            Master => IFF_MASTER,
            Slave => IFF_SLAVE,
            Multicast => IFF_MULTICAST,
            PortSelect => IFF_PORTSEL,
            AutoMedia => IFF_AUTOMEDIA,
            Dynamic => IFF_DYNAMIC,
            LowerUp => IFF_LOWER_UP,
            Dormant => IFF_DORMANT,
            Echo => IFF_ECHO,
        };

        value as u32
    }

    pub fn from_raw_value(value: u32) -> Option<Self> {
        use InterfaceFlag::*;

        Some(match value as i32 {
            IFF_UP => Up,
            IFF_BROADCAST => Broadcast,
            IFF_DEBUG => Debug,
            IFF_LOOPBACK => Loopback,
            IFF_POINTOPOINT => PointToPoint,
            IFF_RUNNING => Running,
            IFF_NOARP => NoArp,
            IFF_PROMISC => Promiscuous,
            IFF_NOTRAILERS => NoTrailers,
            IFF_ALLMULTI => AllMulticast,
            IFF_MASTER => Master,
            IFF_SLAVE => Slave,
            IFF_MULTICAST => Multicast,
            IFF_PORTSEL => PortSelect,
            IFF_AUTOMEDIA => AutoMedia,
            IFF_DYNAMIC => Dynamic,
            IFF_LOWER_UP => LowerUp,
            IFF_DORMANT => Dormant,
            IFF_ECHO => Echo,
            _ => None?,
        })
    }
}

/// This is a bitfield consisting of multiple interface flags.
#[doc(alias("ifinfomsg", "ifi_flags", "IFF_"))]
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
pub struct InterfaceFlags {
    inner: u32,
}

impl InterfaceFlags {
    pub const fn new(flags: &[InterfaceFlag]) -> Self {
        let mut inner = 0u32;

        let mut i = 0;
        while i < flags.len() {
            let flag = flags[i];
            inner |= flag.raw_value();
            i += 1;
        }

        Self { inner }
    }

    pub const unsafe fn with_bits(flags: u32) -> Self {
        Self { inner: flags }
    }

    pub const fn bits(&self) -> u32 {
        self.inner
    }

    pub const fn add_flag(mut self, flag: InterfaceFlag) -> Self {
        self.inner |= flag.raw_value();
        self
    }

    pub const fn remove_flag(mut self, flag: InterfaceFlag) -> Self {
        self.inner &= !flag.raw_value();
        self
    }

    pub const fn has_flag(&self, flag: InterfaceFlag) -> bool {
        (self.inner & flag.raw_value()) > 0
    }
}
