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


	let netsh = NetSH::new(ProxyBridge::V4ToV4)?;
	let hosts = HostFile::new()?;

	match matches.subcommand() {
		(COMMAND_NAME_ADD, Some(matches)) => {
			if has_write_permissions() {
				let address = matches.value_of(ARG_NAME_ADDRESS).unwrap();
				let host = matches.value_of(ARG_NAME_HOST).unwrap();

				command::add::process(address, host, netsh, hosts)?;
			} else {
				println!("Please run as Administrator.");
			}
		}

		(COMMAND_NAME_REMOVE, Some(matches)) => {
			if has_write_permissions() {
				let addr_or_host = matches.value_of(ARG_NAME_ADDRESS_HOST).unwrap();
				command::remove::process(addr_or_host, netsh, hosts)?;
			} else {
				println!("Please run as Administrator.");
			}
		}

		(COMMAND_NAME_LIST, _) => {
			command::list::process(netsh, hosts)?;
		}

		(COMMAND_NAME_TEST, Some(matches)) => {
			let addr_or_host = matches.value_of(ARG_NAME_ADDRESS_HOST).unwrap();
			command::test::process(addr_or_host, netsh, hosts)?;
		}

		_ => ()
	}

	Ok(())
}