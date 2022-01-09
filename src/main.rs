use anyhow::Result;
use clap::{App, Arg, SubCommand};

// https://stackoverflow.com/questions/8652948/using-port-number-in-windows-host-file


mod command;
mod util;

pub use util::*;


const ARG_NAME_ADDRESS: &str = "ADDRESS";
const ARG_NAME_HOST: &str = "HOST";
const ARG_NAME_ADDRESS_HOST: &str = "ADDRESS/HOST";


const COMMAND_NAME_ADD: &str = "add";
const COMMAND_NAME_REMOVE: &str = "remove";
const COMMAND_NAME_LIST: &str = "list";
const COMMAND_NAME_TEST: &str = "test";


fn main() -> Result<()> {
	let matches = App::new("Local Hoster")
		.version("1.0")
		.author("Tim F. (https://github.com/Its-its/localhosting)")
		.about("Makes it easy for you to reverse-proxy your hosts on Windows.")
		// Add
		.subcommand(
			SubCommand::with_name(COMMAND_NAME_ADD)
			.about("Add a new local listening host")
			.arg(Arg::with_name(ARG_NAME_ADDRESS)
				.help("Sets the listening address (127.0.0.1:8080)")
				.required(true))
			.arg(Arg::with_name(ARG_NAME_HOST)
				.help("Sets the listening host (example.com)")
				.required(true))
		)
		// Remove
		.subcommand(
			SubCommand::with_name(COMMAND_NAME_REMOVE)
			.about("Remove listener based on Address OR Host")
			.arg(Arg::with_name(ARG_NAME_ADDRESS_HOST).required(true))
		)
		// List
		.subcommand(SubCommand::with_name(COMMAND_NAME_LIST).about("List host listeners"))
		// Test
		.subcommand(
			SubCommand::with_name(COMMAND_NAME_TEST)
			.about("Test listener(s) based on Address OR Host")
			.arg(Arg::with_name(ARG_NAME_ADDRESS_HOST).required(true))
		)
		.get_matches();


	let mut netsh = NetSH::create(ProxyBridge::V4ToV4)?;
	let mut hosts = HostFile::read()?;

	match matches.subcommand() {
		(COMMAND_NAME_ADD, Some(matches)) => {
			if has_write_permissions() {
				let address = matches.value_of(ARG_NAME_ADDRESS).unwrap();
				let host = matches.value_of(ARG_NAME_HOST).unwrap();

				command::add::process(address, host, &mut netsh, &mut hosts)?;
			} else {
				println!("Please run as Administrator.");
			}
		}

		(COMMAND_NAME_REMOVE, Some(matches)) => {
			if has_write_permissions() {
				let addr_or_host = matches.value_of(ARG_NAME_ADDRESS_HOST).unwrap();
				command::remove::process(addr_or_host, &mut netsh, &mut hosts)?;
			} else {
				println!("Please run as Administrator.");
			}
		}

		(COMMAND_NAME_LIST, _) => {
			command::list::process(&netsh, &hosts)?;
		}

		(COMMAND_NAME_TEST, Some(matches)) => {
			let addr_or_host = matches.value_of(ARG_NAME_ADDRESS_HOST).unwrap();
			command::test::process(addr_or_host, &netsh, &hosts)?;
		}

		_ => ()
	}

	Ok(())
}


#[cfg(test)]
mod tests {
	use crate::{NetSH, HostFile, command, Connection};

	const ADDRESS_HOST_COMBOS: [(&str, &str); 6] = [
		("127.0.0.1:8080", "one.test"),
		("127.0.0.1:8080", "a.one.test"),
		("127.0.0.1:8080", "b.one.test"),

		("127.0.0.1:8081", "two.test"),
		("127.0.0.1:8081", "a.two.test"),

		("127.0.0.1:8082", "three.test")
	];

	#[test]
	fn add_successes() {
		let (mut netsh, mut hosts) = (NetSH::default(), HostFile::default());

		for (addr, host) in ADDRESS_HOST_COMBOS {
			command::add::process(addr, host, &mut netsh, &mut hosts).unwrap();
		}

		// Attempt to add a duplicate (Should not add anything)
		command::add::process(ADDRESS_HOST_COMBOS.last().unwrap().0, ADDRESS_HOST_COMBOS.last().unwrap().1, &mut netsh, &mut hosts).unwrap();

		// Compare bridge listeners against ones which were attempted to add. (each iter for counts will be [3, 2, 1])
		for bridge in &netsh.bridges {
			let combo_count = ADDRESS_HOST_COMBOS.iter().filter(|(v, _)| bridge.connect_to == v.parse::<Connection>().unwrap()).count();

			let registered_count = hosts.count_addresses(bridge.listen_to.address);

			assert_eq!(combo_count, registered_count, "Unique Bridge Listeners");
		}

		assert_eq!(3, netsh.bridges.len(), "Bridges Count (Unique addresses)");
		assert_eq!(6, hosts.items.len(), "Hosts Length (Unique Connections in Hosts File)");
	}

	#[test]
	fn remove_successes() {
		let (mut netsh, mut hosts) = (NetSH::default(), HostFile::default());

		for (addr, host) in ADDRESS_HOST_COMBOS {
			command::add::process(addr, host, &mut netsh, &mut hosts).unwrap();
		}

		// Start off with initial state. Should be correct.
		assert_eq!(3, netsh.bridges.len(), "[Initial] Bridges Count (Unique addresses)");
		assert_eq!(6, hosts.items.len(), "[Initial] Hosts Length (Unique Connections in Hosts File)");

		// Remove single host only
		command::remove::process("a.one.test", &mut netsh, &mut hosts).unwrap();

		assert_eq!(3, netsh.bridges.len(), "[1 Host Removal] Bridges Count (Unique addresses)");
		assert_eq!(5, hosts.items.len(), "[1 Host Removal] Hosts Length (Unique Connections in Hosts File)");

		// Remove single host only
		command::remove::process("two.test", &mut netsh, &mut hosts).unwrap();

		assert_eq!(3, netsh.bridges.len(), "[2 Hosts Removal] Bridges Count (Unique addresses)");
		assert_eq!(4, hosts.items.len(), "[2 Hosts Removal] Hosts Length (Unique Connections in Hosts File)");

		// Remove multiple hosts and bridge
		command::remove::process("127.0.0.1:8080", &mut netsh, &mut hosts).unwrap();

		assert_eq!(2, netsh.bridges.len(), "[2 Hosts + Bridge Removal] Bridges Count (Unique addresses)");
		assert_eq!(2, hosts.items.len(), "[2 Hosts + Bridge Removal] Hosts Length (Unique Connections in Hosts File)");
	}

	#[test]
	fn add_failures() {
		//
	}

	#[test]
	fn remove_failures() {
		//
	}
}