use anyhow::Result;

use crate::{HostFile, Connection, NetSH, ProxyBridge};



pub fn process(address_str: &str, host: &str, netsh: &mut NetSH, hosts: &mut HostFile) -> Result<()> {
	let connect_to: Connection = address_str.parse()?;

	let (bridge, is_new) = netsh.add_or_retreive(connect_to, ProxyBridge::V4ToV4)?;

	// If it's not newly added.
	if !is_new && hosts.find_item_by_host(host).is_some() {
		println!("Host already exists for {}", connect_to);
		return Ok(());
	}


	// Add Host, Delete Bridge IF Error.
	if let Err(e) = hosts.add(bridge.listen_to.address, host.to_string()) {
		netsh.delete(connect_to, ProxyBridge::V4ToV4)?;
		return Err(e);
	}

	if is_new {
		println!("Added {} to new Bridge for {:?}.", connect_to, host);
	} else {
		println!("Added {} to existing Bridge for {:?}.", connect_to, host);
	}

	Ok(())
}