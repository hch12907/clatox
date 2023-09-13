use libc::*;

#[doc(alias("rtmsg", "RT_TABLE_"))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RouteTable {
    Unspecified,
    Compat,
    Default,
    Main,
    Local,
}

impl RouteTable {
    pub fn from_raw_value(value: u8) -> Option<Self> {
        use RouteTable::*;

        Some(match value {
            RT_TABLE_UNSPEC => Unspecified,
            RT_TABLE_COMPAT => Compat,
            RT_TABLE_DEFAULT => Default,
            RT_TABLE_MAIN => Main,
            RT_TABLE_LOCAL => Local,
            _ => None?
        })
    }

    pub fn raw_value(&self) -> u8 {
        use RouteTable::*;

        match self {
            Unspecified => RT_TABLE_UNSPEC,
            Compat => RT_TABLE_COMPAT,
            Default => RT_TABLE_DEFAULT,
            Main => RT_TABLE_MAIN,
            Local => RT_TABLE_LOCAL,
        }
    }
}
