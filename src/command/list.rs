use anyhow::Result;

use crate::{HostFile, NetSH};


pub fn process(netsh: NetSH, hosts: HostFile) -> Result<()> {
	for bridge in netsh.bridges {
		let found = hosts.find_by_address(bridge.listen_to.address);

		if !found.is_empty() {
			println!(r#"Listening to "{}" for hosts(s): "#, bridge.connect_to);

			for item in found {
				println!("\t- {}", item.host);
			}
		}

		println!();
	}

	Ok(())
}