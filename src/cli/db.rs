use clap::Args;
use std::{
    fmt::Display,
    net::{AddrParseError, Ipv4Addr, SocketAddr, SocketAddrV4},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug, Args, Clone, Default)]
pub struct DbModeOptions {
    pub session_id: String,

    #[arg(short, long, default_value_t)]
    pub addr: DbAddr,
}

#[derive(Debug, Clone)]
pub struct DbAddr(pub SocketAddr);

impl Default for DbAddr {
    fn default() -> Self {
        Self(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            9042,
        )))
    }
}

impl Display for DbAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DbAddr {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(SocketAddr::from_str(s)?))
    }
}

impl Deref for DbAddr {
    type Target = SocketAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
