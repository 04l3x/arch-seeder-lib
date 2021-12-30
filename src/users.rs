use crate::{error::*, terminal::Executer};
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct User {
	name: String,
	groups: Option<Vec<String>>,
	passwd: String,
	#[serde(default = "defaults::terminal")]
	terminal: String,
}

impl User {
	fn add(self) -> Result<()> {
		match self.groups {
			Some(groups) => Executer::execute(
				Command::new("arch-chroot")
					.arg("/mnt")
					.arg("useradd")
					.arg("-m")
					.arg("-g")
					.arg("users")
					.arg("-G")
					.arg(User::groups(groups))
					.arg("-s")
					.arg(User::terminal(&self.terminal))
					.arg("-p")
					.arg(&self.passwd)
					.arg(&self.name),
				"Error creating users",
			),
			None => Executer::execute(
				Command::new("arch-chroot")
					.arg("/mnt")
					.arg("useradd")
					.arg("-m")
					.arg("-g")
					.arg("users")
					.arg("-s")
					.arg(User::terminal(&self.terminal))
					.arg("-p")
					.arg(&self.passwd)
					.arg(&self.name),
				"Error creating users",
			),
		}
	}

	fn groups(groups: Vec<String>) -> String {
		let mut g = String::new();
		for group in groups {
			g.push_str(&group);
			g.push(',');
		}
		g.pop();
		g
	}

	fn terminal<'a>(terminal: &'a str) -> &'a str {
		match terminal {
			"zsh" => "/bin/zsh",
			"sh" => "/bin/sh",
			_ => "/bin/bash",
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct Root {
	passwd: String,
}

impl Root {
	fn set_pwd(&self) -> Result<()> {
		Executer::execute_with_input(
			Command::new("arch-chroot").arg("/mnt").arg("passwd"),
			"error setting root passwd",
			[self.passwd.as_str(), self.passwd.as_str()],
		)
	}
}

pub mod worker {
	use super::*;
	use crate::error::Result;

	pub fn set_root(root: Root) -> Result<()> {
		root.set_pwd().unwrap();
		Ok(())
	}

	pub fn set_users(users: Vec<User>) -> Result<()> {
		for user in users {
			user.add().unwrap();
		}
		Ok(())
	}
}

mod defaults {
	pub fn terminal() -> String {
		String::from("bash")
	}
}
