use libc::*;

/// This enum corresponds to the `NLM_F_*` constants.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Flag {
    Request,
    Multi,
    Ack,
    Echo,

    // Get flags
    Root,
    Match,
    Atomic,
    Dump,

    // New flags
    Replace,
    Excl,
    Create,
    Append,
}

impl Flag {
    pub const fn raw_value(&self) -> u16 {
        use Flag::*;

        let value = match self {
            Request => NLM_F_REQUEST,
            Multi => NLM_F_MULTI,
            Ack => NLM_F_ACK,
            Echo => NLM_F_ECHO,
        
            // Get flags
            Root => NLM_F_ROOT,
            Match => NLM_F_MATCH,
            Atomic => NLM_F_ATOMIC,
            Dump => NLM_F_DUMP,
        
            // New flags
            Replace => NLM_F_REPLACE,
            Excl => NLM_F_EXCL,
            Create => NLM_F_CREATE,
            Append => NLM_F_APPEND,
        };

        value as u16
    }
}

/// This is a bitfield consisting of multiple Netlink flags.
/// It corresponds to `nlmsg_flags`.
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
pub struct Flags {
    inner: u16,
}

impl Flags {
    pub const fn new(flags: &[Flag]) -> Self {
        let mut inner = 0u16;

        let mut i = 0;
        while i < flags.len() {
            let flag = flags[i];
            inner |= flag.raw_value();
            i += 1;
        }

        Self { inner }
    }

    pub const unsafe fn with_bits(flags: u16) -> Self {
        Self { inner: flags }
    }

    pub const fn bits(&self) -> u16 {
        self.inner
    }

    pub const fn add_flag(mut self, flag: Flag) -> Self {
        self.inner |= flag.raw_value();
        self
    }

    pub const fn remove_flag(mut self, flag: Flag) -> Self {
        self.inner &= !flag.raw_value();
        self
    }

    pub const fn has_flag(&self, flag: Flag) -> bool {
        (self.inner & flag.raw_value()) > 0
    }
}
