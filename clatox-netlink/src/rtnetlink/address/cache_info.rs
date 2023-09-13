#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias("ifa_cacheinfo"))]
#[repr(C)]
pub struct AddressCacheInfo {
    pub preferred: u32,
    pub valid: u32,
    pub created_timestamp: u32,
    pub updated_timestamp: u32,
}
