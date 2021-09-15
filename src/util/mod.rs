mod hosts;
mod netsh;


use std::{fmt::{self, Display}, net::Ipv4Addr, str::FromStr};

pub use hosts::{HostFile, HostItem, DeletionType, has_write_permissions};
pub use netsh::{Bridge, NetSH, ProxyBridge};



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Connection {
	pub address: Ipv4Addr,
	pub port: u16
}

impl From<(Ipv4Addr, u16)> for Connection {
    fn from(value: (Ipv4Addr, u16)) -> Self {
        Self {
			address: value.0,
			port: value.1
		}
    }
}


impl FromStr for Connection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (addr, port) = s.split_once(':').expect("Expected a port");
        Ok(Self { address: addr.parse()?, port: port.parse()? })
    }
}

impl Display for Connection {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.address, self.port)
	}
}