use std::{
	net::Ipv4Addr,
	process::Command
};

use anyhow::Result;

use crate::Connection;


pub struct NetSH {
	pub bridges: Vec<Bridge>
}

impl NetSH {
	pub fn new(proxy: ProxyBridge) -> Result<Self> {
		let output = Command::new("netsh")
			.args(&["interface", "portproxy", "show", proxy.into_str()])
			.output()?;

		if !output.status.success() {
			panic!("[netsh][LIST]: {}", output.status);
		}

		let value = String::from_utf8(output.stdout)?;

		let bridges = value.lines()
			// Skip until "-"
			.skip_while(|v| !v.starts_with('-'))
			// Skip "-"
			.skip(1)
			// Parse lines.
			.filter_map(|line| {
				let mut split = line.split_ascii_whitespace();

				Some(Bridge {
					listen_to: Connection {
						address: split.next()?.parse().ok()?,
						port: split.next()?.parse().ok()?
					},

					connect_to: Connection {
						address: split.next()?.parse().ok()?,
						port: split.next()?.parse().ok()?
					}
				})
			})
			.collect::<Vec<_>>();

		Ok(Self {
			bridges
		})
	}

	pub fn contains(&self, value: Connection) -> bool {
		self.bridges.iter().any(|v| v.connect_to == value || v.listen_to == value)
	}

	pub fn delete(&mut self, connect_to: Connection, proxy: ProxyBridge) -> Result<Option<Bridge>> {
		if let Some(index) = self.bridges.iter().position(|v| v.listen_to == connect_to || v.connect_to == connect_to) {
			let bridge = self.bridges.remove(index);

			bridge.delete(proxy)?;

			Ok(Some(bridge))
		} else {
			Ok(None)
		}
	}

	pub fn get_by_connection(&self, value: Connection) -> Option<&Bridge> {
		for bridge in &self.bridges {
			if bridge.connect_to == value || bridge.listen_to == value {
				return Some(bridge);
			}
		}

		None
	}

	/// Returns The Bridge and bool specifying if it's new.
	pub fn add_or_retreive(&mut self, connect_to: Connection, proxy: ProxyBridge) -> Result<(Bridge, bool)> {
		if let Some(c) = self.get_by_connection(connect_to) {
			Ok((c.clone(), false))
		} else {
			// Find unused listening address.
			let listen_to: Connection = loop {
				let oct = gen_octets();

				// Since people normally use 127.0.0.* I don't want to infringe on it.
				if oct.0 == 0 && oct.1 == 0 {
					continue;
				}

				let conn: Connection = (Ipv4Addr::new(127, oct.0, oct.1, oct.2), 80).into();

				if !self.contains(conn) {
					break conn;
				}
			};

			// Find connecting address & port.
			if self.contains(connect_to) {
				panic!("Bridge already exists for {}.", connect_to);
			}

			// netsh interface portproxy add v4tov4 listenport=80 listenaddress=127.*.*.* connectport=**** connectaddress=127.0.0.1
			let output = Command::new("netsh")
				.args(&[
					"interface",
					"portproxy",
					"add",
					proxy.into_str(),
					&format!("listenaddress={}", listen_to.address),
					&format!("listenport={}", listen_to.port),
					&format!("connectaddress={}", connect_to.address),
					&format!("connectport={}", connect_to.port),
				])
				.output()?;

			if !output.status.success() {
				panic!("[netsh][ADD]: {}", output.status);
			}

			self.bridges.push(Bridge {
				listen_to,
				connect_to
			});

			Ok((self.bridges.last().unwrap().clone(), true))
		}
	}
}




#[derive(Debug, Clone)]
pub struct Bridge {
	/// Randomly generated backend 127.*.*.* Ip Address.
	pub listen_to: Connection,
	/// Known IP Address used to connect to the Host Proxy.
	pub connect_to: Connection
}

impl Bridge {
	fn delete(&self, proxy: ProxyBridge) -> Result<()> {
		// netsh interface portproxy delete v4tov4 listenport=80 listenaddress=127.*.*.*
		let output = Command::new("netsh")
			.args(&[
				"interface",
				"portproxy",
				"delete",
				proxy.into_str(),
				&format!("listenport={}", self.listen_to.port),
				&format!("listenaddress={}", self.listen_to.address)
			])
			.output()?;

		if !output.status.success() {
			panic!("[netsh][DELETE]: {}", output.status);
		}

		Ok(())
	}
}


#[derive(Debug, Clone, Copy)]
pub enum ProxyBridge {
	V4ToV4,
	// V4ToV6,
	// V6ToV4,
	// V6ToV6
}

impl ProxyBridge {
	pub fn into_str(self) -> &'static str {
		match self {
			Self::V4ToV4 => "v4tov4",
			// Self::V4ToV6 => "v4tov6",
			// Self::V6ToV4 => "v6tov4",
			// Self::V6ToV6 => "v6tov6",
		}
	}
}



fn gen_octets() -> (u8, u8, u8) {
	let range = rand::random::<u32>();

	let addr_0 = range & 0xFF;
	let addr_1 = (range >> 8) & 0xFF;
	let addr_2 = (range >> 16) & 0xFF;

	(addr_0 as u8, addr_1 as u8, addr_2 as u8)
}