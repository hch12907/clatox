use libc::*;

#[doc(alias("rtmsg", "RT_SCOPE_"))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RouteScope {
    Universe,
    Site,
    Link,
    Host,
    Nowhere,
}

impl RouteScope {
    pub fn from_raw_value(value: u8) -> Option<Self> {
        use RouteScope::*;

        Some(match value {
            RT_SCOPE_UNIVERSE => Universe,
            RT_SCOPE_SITE => Site,
            RT_SCOPE_LINK => Link,
            RT_SCOPE_HOST => Host,
            RT_SCOPE_NOWHERE => Nowhere,
            _ => None?
        })
    }

    pub fn raw_value(&self) -> u8 {
        use RouteScope::*;

        match self {
            Universe => RT_SCOPE_UNIVERSE,
            Site => RT_SCOPE_SITE,
            Link => RT_SCOPE_LINK,
            Host => RT_SCOPE_HOST,
            Nowhere => RT_SCOPE_NOWHERE,
        }
    }
}
