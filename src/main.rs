use anyhow::Result;
use clap::{App, Arg, SubCommand};

// https://stackoverflow.com/questions/8652948/using-port-number-in-windows-host-file


mod command;
mod util;

pub use util::*;


fn main() -> Result<()> {
	let matches = App::new("Local Hoster")
		.version("1.0")
		.author("Tim F. (https://github.com/Its-its/localhosting)")
		.about("Makes it easy for you to reverse-proxy your hosts on Windows.")
		// Add
		.subcommand(
			SubCommand::with_name("add")
			.about("Add a new local listening host")
			.arg(Arg::with_name("ADDRESS")
				.help("Sets the listening address (127.0.0.1:8080)")
				.required(true))
			.arg(Arg::with_name("HOST")
				.help("Sets the listening host (example.com)")
				.required(true))
		)
		// Remove
		.subcommand(
			SubCommand::with_name("remove")
			.about("Remove listener based on Address OR Host")
			.arg(Arg::with_name("ADDRESS/HOST").required(true))
		)
		// List
		.subcommand(SubCommand::with_name("list").about("List host listeners"))
		// Test
		.subcommand(
			SubCommand::with_name("test")
			.about("Test listener(s) based on Address OR Host")
			.arg(Arg::with_name("ADDRESS/HOST").required(true))
		)
		.get_matches();


	let netsh = NetSH::new(ProxyBridge::V4ToV4)?;
	let hosts = HostFile::new()?;

	match matches.subcommand() {
		("add", Some(matches)) => {
			if has_write_permissions()? {
				let address = matches.value_of("ADDRESS").unwrap();
				let host = matches.value_of("HOST").unwrap();

				command::add::process(address, host, netsh, hosts)?;
			} else {
				println!("Please run as Administrator.");
			}
		}

		("remove", Some(matches)) => {
			if has_write_permissions()? {
				let addr_or_host = matches.value_of("ADDRESS/HOST").unwrap();
				command::remove::process(addr_or_host, netsh, hosts)?;
			} else {
				println!("Please run as Administrator.");
			}
		}

		("list", _) => {
			command::list::process(netsh, hosts)?;
		}

		("test", Some(matches)) => {
			let addr_or_host = matches.value_of("ADDRESS/HOST").unwrap();
			command::test::process(addr_or_host, netsh, hosts)?;
		}

		_ => ()
	}

	Ok(())
}