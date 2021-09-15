use std::{
	fs,
	net::Ipv4Addr
};

use anyhow::Result;

pub const HOSTS_FILE_PATH: &str = "C:/Windows/System32/drivers/etc/hosts";


#[derive(Debug)]
pub struct HostFile {
	items: Vec<HostItem>
}

impl HostFile {
	pub fn new() -> Result<Self> {
		let value = fs::read_to_string(HOSTS_FILE_PATH)?;

		let items = value.lines()
			// Remove Empty AND Commented Lines
			.filter(|v| !v.is_empty() && !v.starts_with('#'))
			// Parse Lines
			.filter_map(parse_line)
			.collect::<Result<_>>()?;

		Ok(HostFile {
			items
		})
	}

	pub fn find_item_by_host(&self, value: &str) -> Option<&HostItem> {
		self.items.iter().find(|v| v.host == value)
	}

	pub fn find_by_address(&self, value: Ipv4Addr) -> Vec<&HostItem> {
		self.items.iter().filter(|v| v.address == value).collect()
	}

	pub fn count_addresses(&self, value: Ipv4Addr) -> usize {
		self.items.iter().filter(|v| v.address == value).count()
	}

	pub fn add(&mut self, address: Ipv4Addr, host: &str) -> Result<()> {
		self.items.push(HostItem {
			address,
			host: host.to_string()
		});

		let contents = fs::read_to_string(HOSTS_FILE_PATH)?;

		fs::write(
			HOSTS_FILE_PATH,
			format!(
				"{}\n{} {} # Do NOT Remove. Added Automatically (https://github.com/Its-its/localhosting)",
				contents,
				address,
				host
			)
		)?;

		Ok(())
	}


	pub fn delete(&mut self, value: DeletionType) -> Result<Vec<HostItem>> {
		let file = fs::read_to_string(HOSTS_FILE_PATH)?;


		let filter = |line: &str| -> bool {
			match value {
				DeletionType::Address(v) => !line.contains(&v.to_string()),
				DeletionType::Host(v) => !line.contains(v)
			}
		};


		let contents = file
			.lines()
			.filter(|v| filter(v))
			.collect::<Vec<_>>();


		fs::write(
			HOSTS_FILE_PATH,
			contents.join("\n")
		)?;


		let hosts = file
			.lines()
			.filter(|v| !filter(v))
			.filter_map(parse_line)
			.collect::<Result<Vec<_>>>()?;

		Ok(hosts)
	}

}


pub enum DeletionType<'a> {
	Address(Ipv4Addr),
	Host(&'a str)
}


#[derive(Debug, Clone)]
pub struct HostItem {
	pub address: Ipv4Addr,
	pub host: String
}


// TODO: Better way to check if has write permissions? Check if ran as Administrator somehow?
pub fn has_write_permissions() -> Result<bool> {
	let contents = fs::read_to_string(HOSTS_FILE_PATH)?;
	Ok(fs::write(HOSTS_FILE_PATH, contents).is_ok())
}


fn parse_line(line: &str) -> Option<Result<HostItem>> {
	let mut split = line.split_ascii_whitespace();

	let address = split.next()?.parse();
	let host = split.next()?;

	match address {
		Ok(address) => Some(Ok(HostItem {
			address,
			host: host.to_string()
		})),

		Err(e) => Some(Err(e.into()))
	}
}