use anyhow::Result;

use crate::{DeletionType, HostFile, Connection, NetSH, ProxyBridge};


pub fn process(addr_or_host: &str, mut netsh: NetSH, mut hosts: HostFile) -> Result<()> {
	// Manage Command "file.exe remove 127.0.0.1:8080"
	if addr_or_host.contains(':') {
		let connect_to = addr_or_host.parse::<Connection>()?;

		// Find Bridge from Connection Address and Port (ex: 127.0.0.1:8080)
		if let Some(bridge) = netsh.get_by_connection(connect_to) {
			let removed = hosts.delete(DeletionType::Address(bridge.listen_to.address))?;

			for host in removed {
				netsh.delete((host.address, 80).into(), ProxyBridge::V4ToV4)?;
			}
		}
	}

	// Manage Command "file.exe remove example.com"
	else {
		let removed = hosts.delete(DeletionType::Host(addr_or_host))?;

		for host in removed {
			netsh.delete((host.address, 80).into(), ProxyBridge::V4ToV4)?;
		}
	}

	Ok(())
}