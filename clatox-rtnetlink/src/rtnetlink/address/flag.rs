use libc::*;

use bitflags::bitflags;

bitflags! {
    /// A bitfield of all address flags.
    /// 
    /// It corresponds to `IFA_F_*` and `IFA_*` in libc.
    #[doc(alias("ifaddrmsg", "ifa_flags", "IFA_F_", "IFA_"))]
    #[derive(Copy, Debug, Clone, PartialEq, Eq)]
    pub struct AddressFlags: u32 {
        const Temporary = IFA_F_TEMPORARY;
        /// No Duplicated
        const NoDad = IFA_F_NODAD;
        const Optimistic = IFA_F_OPTIMISTIC;
        /// Duplicated Address
        const DadFailed = IFA_F_DADFAILED;
        const HomeAddress = IFA_F_HOMEADDRESS;
        const Deprecated = IFA_F_DEPRECATED;
        const Tentative = IFA_F_TENTATIVE;
        const Permanent = IFA_F_PERMANENT;
        /// Tells kernel to manage temporary addresses
        const ManageTempAddr = IFA_F_MANAGETEMPADDR;
        /// Don't create routes automatically
        const NoPrefixRoute = IFA_F_NOPREFIXROUTE;
        const MulticastAutoJoin = IFA_F_MCAUTOJOIN;
        /// Stable private IPv6 SLAAC addresses (RFC 7217)
        const StablePrivacy = IFA_F_STABLE_PRIVACY;
    }
}
