/// The field names are taken verbatim from <linux/if_link.h>.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias("rtnl_link_stats"))]
#[repr(C)]
pub struct InterfaceStats {
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub rx_bytes: u32,
    pub tx_bytes: u32,
    pub rx_errors: u32,
    pub tx_errors: u32,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub multicast: u32,
    pub collisions: u32,

    pub rx_length_errors: u32,
    pub rx_over_errors: u32,
    pub rx_crc_errors: u32,
    pub rx_frame_errors: u32,
    pub rx_fifo_errors: u32,
    pub rx_missed_errors: u32,

    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_fifo_errors: u32,
    pub tx_heartbeat_errors: u32,
    pub tx_window_errors: u32,

    pub rx_compressed: u32,
    pub tx_compressed: u32,
    pub rx_nohandler: u32,
}

/// The 64-bit version of InterfaceStats.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias("rtnl_link_stats64"))]
#[repr(C)]
pub struct InterfaceStats64 {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub multicast: u64,
    pub collisions: u64,

    pub rx_length_errors: u64,
    pub rx_over_errors: u64,
    pub rx_crc_errors: u64,
    pub rx_frame_errors: u64,
    pub rx_fifo_errors: u64,
    pub rx_missed_errors: u64,

    pub tx_aborted_errors: u64,
    pub tx_carrier_errors: u64,
    pub tx_fifo_errors: u64,
    pub tx_heartbeat_errors: u64,
    pub tx_window_errors: u64,

    pub rx_compressed: u64,
    pub tx_compressed: u64,
    pub rx_nohandler: u64,

    pub rx_otherhost_dropped: u64,
}
