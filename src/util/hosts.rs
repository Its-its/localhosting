use std::{fs, net::Ipv4Addr, path::Path};

use faccess::{AccessMode, PathExt};
use anyhow::Result;


pub const HOSTS_FILE_PATH: &str = "C:/Windows/System32/drivers/etc/hosts";

const COMMENT_CHARACTER: char = '#';


#[derive(Debug, Default)]
pub struct HostFile {
	/// Check to see if we should use the windows host file. Used for tests.
	pub uses_host_file: bool,
	pub items: Vec<HostItem> // TODO: Store line pos for better removal?
}

impl HostFile {
	pub fn read() -> Result<Self> {
		let value = fs::read_to_string(HOSTS_FILE_PATH)?;

		let items = value.lines()
			// Remove Empty AND Commented Lines
			.filter(|v| !v.is_empty() && !v.starts_with(COMMENT_CHARACTER))
			// Parse Lines
			.filter_map(parse_line)
			.collect::<Result<_>>()?;

		Ok(HostFile {
			uses_host_file: true,
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

	pub fn add(&mut self, address: Ipv4Addr, host: String) -> Result<()> {
		if self.uses_host_file {
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
		}

		self.items.push(HostItem {
			address,
			host
		});

		Ok(())
	}

	pub fn delete(&mut self, value: DeletionType) -> Result<Vec<HostItem>> {
		if self.uses_host_file {
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
		}

		// Find items which need to be deleted.
		let deleting = self.items.iter()
			.enumerate()
			.filter(|(_, item)| match &value {
				DeletionType::Address(v) => &item.address == v,
				DeletionType::Host(v) => item.host.contains(v)
			})
			.map(|(i, _)| i)
			.rev()
			.collect::<Vec<_>>();

		let mut deleted_hosts = Vec::new();

		for index in deleting {
			deleted_hosts.push(self.items.remove(index));
		}

		Ok(deleted_hosts)
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


pub fn has_write_permissions() -> bool {
	Path::new(HOSTS_FILE_PATH).access(AccessMode::WRITE).is_ok()
}


fn parse_line(line: &str) -> Option<Result<HostItem>> {
	let mut split = line.split_ascii_whitespace();

	let address = split.next()?.parse();
	let host = split.next()?;

	// Check if Host contains comment. If so, only grab text before it.
	let host = if let Some(index) = host.find(COMMENT_CHARACTER) {
		&host[0..index]
	} else {
		host
	};

	match address {
		Ok(address) => Some(Ok(HostItem {
			address,
			host: host.to_string()
		})),

		Err(e) => Some(Err(e.into()))
	}
}