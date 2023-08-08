use libc::*;

/// An enum of all address flags.
/// It corresponds to `IFA_F_*` and `IFA_*` in libc.
#[doc(alias("ifaddrmsg", "ifa_flags", "IFA_F_", "IFA_"))]
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AddressFlag {
    Temporary = IFA_F_TEMPORARY,
    /// No Duplicated Address Detection (DAD) performed
    NoDad = IFA_F_NODAD,
    Optimistic = IFA_F_OPTIMISTIC,
    /// Duplicated Address Detection (DAD) failed, there is a dupe in local network.
    DadFailed = IFA_F_DADFAILED,
    HomeAddress = IFA_F_HOMEADDRESS,
    Deprecated = IFA_F_DEPRECATED,
    Tentative = IFA_F_TENTATIVE,
    Permanent = IFA_F_PERMANENT,
}

impl AddressFlag {
    pub const fn raw_value(&self) -> u8 {
        use AddressFlag::*;

        let value = match self {
            Temporary => IFA_F_TEMPORARY,
            NoDad => IFA_F_NODAD,
            Optimistic => IFA_F_OPTIMISTIC,
            DadFailed => IFA_F_DADFAILED,
            HomeAddress => IFA_F_HOMEADDRESS,
            Deprecated => IFA_F_DEPRECATED,
            Tentative => IFA_F_TENTATIVE,
            Permanent => IFA_F_PERMANENT,
        };

        value as u8
    }

    pub fn from_raw_value(value: u8) -> Option<Self> {
        use AddressFlag::*;

        Some(match value as u32 {
            IFA_F_TEMPORARY => Temporary,
            IFA_F_NODAD => NoDad,
            IFA_F_OPTIMISTIC => Optimistic,
            IFA_F_DADFAILED => DadFailed,
            IFA_F_HOMEADDRESS => HomeAddress,
            IFA_F_DEPRECATED => Deprecated,
            IFA_F_TENTATIVE => Tentative,
            IFA_F_PERMANENT => Permanent,
            _ => None?,
        })
    }
}

/// This is a bitfield consisting of multiple address flags.
#[doc(alias("ifaddrmsg", "ifa_flags", "IFA_F_"))]
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
pub struct AddressFlags {
    inner: u8,
}

impl AddressFlags {
    pub const fn new(flags: &[AddressFlag]) -> Self {
        let mut inner = 0u8;

        let mut i = 0;
        while i < flags.len() {
            let flag = flags[i];
            inner |= flag.raw_value();
            i += 1;
        }

        Self { inner }
    }

    pub const unsafe fn with_bits(flags: u8) -> Self {
        Self { inner: flags }
    }

    pub const fn bits(&self) -> u8 {
        self.inner
    }

    pub const fn add_flag(mut self, flag: AddressFlag) -> Self {
        self.inner |= flag.raw_value();
        self
    }

    pub const fn remove_flag(mut self, flag: AddressFlag) -> Self {
        self.inner &= !flag.raw_value();
        self
    }

    pub const fn has_flag(&self, flag: AddressFlag) -> bool {
        (self.inner & flag.raw_value()) > 0
    }
}
