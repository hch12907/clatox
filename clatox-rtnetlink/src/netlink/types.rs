use libc::*;

/// Message types in Netlink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    Noop,
    Error,
    Done,
    Overrun,

    Route(RouteType),
}

impl Type {
    pub const fn raw_value(&self) -> u16 {
        use Type::*;

        let value = match self {
            Noop => NLMSG_NOOP,
            Error => NLMSG_ERROR,
            Done => NLMSG_DONE,
            Overrun => NLMSG_OVERRUN,
            Route(r) => r.raw_value() as i32,
        };

        value as u16
    }

    pub const fn from_raw_value(value: u16) -> Option<Self> {
        let result = match value as i32 {
            NLMSG_NOOP => Some(Type::Noop),
            NLMSG_ERROR => Some(Type::Error),
            NLMSG_DONE => Some(Type::Done),
            NLMSG_OVERRUN => Some(Type::Overrun),
            _ => None,
        };

        // TODO: replace this with regular or_else(...).map(...) once const fn
        // gets smarter
        macro_rules! const_or_else_map {
            ($old_opt:expr, $new_opt:expr, $map_to:path) => {
                if $old_opt.is_none() {
                    match $new_opt {
                        Some(x) => Some($map_to(x)),
                        None => None,
                    }
                } else {
                    $old_opt
                }
            };
        }

        let result = const_or_else_map!(result, RouteType::from_raw_value(value), Type::Route);

        result
    }
}

/// Messsage types in the Route protocol.
///
/// Naming conventions
/// ======
/// The abbreviations used in the original `RTM_*` libc constants are expanded
/// for greater readability.
///
/// The expansion rules are listed below:
///
/// ```
/// Del => Delete
/// Addr => Address
/// Neigh => Neighbor
/// T => Traffic
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteType {
    NewLink,
    DeleteLink,
    GetLink,

    NewAddress,
    DeleteAddress,
    GetAddress,

    NewRoute,
    DeleteRoute,
    GetRoute,

    NewNeighbor,
    DeleteNeighbor,
    GetNeighbor,

    NewRule,
    DeleteRule,
    GetRule,

    NewTrafficClass,
    DeleteTrafficClass,
    GetTrafficClass,

    NewTrafficFilter,
    DeleteTrafficFilter,
    GetTrafficFilter,
}

impl RouteType {
    pub const fn raw_value(&self) -> u16 {
        use RouteType::*;

        let value = match self {
            NewLink => RTM_NEWLINK,
            DeleteLink => RTM_DELLINK,
            GetLink => RTM_GETLINK,

            NewAddress => RTM_NEWADDR,
            DeleteAddress => RTM_DELADDR,
            GetAddress => RTM_GETADDR,

            NewRoute => RTM_NEWROUTE,
            DeleteRoute => RTM_DELROUTE,
            GetRoute => RTM_GETROUTE,

            NewNeighbor => RTM_NEWNEIGH,
            DeleteNeighbor => RTM_DELNEIGH,
            GetNeighbor => RTM_GETNEIGH,

            NewRule => RTM_NEWRULE,
            DeleteRule => RTM_DELRULE,
            GetRule => RTM_GETRULE,

            NewTrafficClass => RTM_NEWTCLASS,
            DeleteTrafficClass => RTM_DELTCLASS,
            GetTrafficClass => RTM_GETTCLASS,

            NewTrafficFilter => RTM_NEWTFILTER,
            DeleteTrafficFilter => RTM_DELTFILTER,
            GetTrafficFilter => RTM_GETTFILTER,
        };

        value as u16
    }

    pub const fn from_raw_value(value: u16) -> Option<Self> {
        match value {
            RTM_NEWLINK => Some(RouteType::NewLink),
            RTM_DELLINK => Some(RouteType::DeleteLink),
            RTM_GETLINK => Some(RouteType::GetLink),

            RTM_NEWADDR => Some(RouteType::NewAddress),
            RTM_DELADDR => Some(RouteType::DeleteAddress),
            RTM_GETADDR => Some(RouteType::GetAddress),

            RTM_NEWROUTE => Some(RouteType::NewRoute),
            RTM_DELROUTE => Some(RouteType::DeleteRoute),
            RTM_GETROUTE => Some(RouteType::GetRoute),

            RTM_NEWNEIGH => Some(RouteType::NewNeighbor),
            RTM_DELNEIGH => Some(RouteType::DeleteNeighbor),
            RTM_GETNEIGH => Some(RouteType::GetNeighbor),

            RTM_NEWRULE => Some(RouteType::NewRule),
            RTM_DELRULE => Some(RouteType::DeleteRule),
            RTM_GETRULE => Some(RouteType::GetRule),

            RTM_NEWTCLASS => Some(RouteType::NewTrafficClass),
            RTM_DELTCLASS => Some(RouteType::DeleteTrafficClass),
            RTM_GETTCLASS => Some(RouteType::GetTrafficClass),

            RTM_NEWTFILTER => Some(RouteType::NewTrafficFilter),
            RTM_DELTFILTER => Some(RouteType::DeleteTrafficFilter),
            RTM_GETTFILTER => Some(RouteType::GetTrafficFilter),

            _ => None,
        }
    }
}

impl From<RouteType> for Type {
    fn from(route: RouteType) -> Self {
        Type::Route(route)
    }
}
