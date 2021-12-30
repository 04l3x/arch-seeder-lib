use crate::fs::write_on_file;
use crate::{error::Result, terminal::Executer};
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct System {
	timezone: Timezone,
	localization: Localization,
	network: Network,
}

impl System {
	fn set_config(self) {
		self.timezone.set();
		self.localization.set();
		self.network.set();
	}
}

#[derive(Debug, Deserialize)]
pub struct Timezone {
	region: String,
	city: String,
}

impl Timezone {
	fn set(&self) {
		self.set_timezone().unwrap();
		Timezone::set_hwclock().unwrap();
	}

	fn set_timezone(&self) -> Result<()> {
		let timezone = format!("/usr/share/zoneinfo/{}/{}", self.region, self.city);

		Executer::execute(
			Command::new("arch-chroot")
				.arg("/mnt")
				.arg("ln")
				.arg("-sf")
				.arg(timezone)
				.arg("/etc/localtime"),
			"error setting timezone",
		)
	}

	fn set_hwclock() -> Result<()> {
		Executer::execute(
			Command::new("arch-chroot")
				.arg("/mnt")
				.arg("hwclock")
				.arg("--systohc"),
			"error setting hwclock",
		)
	}
}

#[derive(Debug, Deserialize)]
pub struct Localization {
	locale: String,
	lang: String,
	keymap: Option<String>,
}

///TODO: personalize default options
impl Localization {
	///all executes of command call a panic, so is not needed matching pattern
	fn set(&self) {
		self.set_locale().unwrap();
		self.set_lang().unwrap();
		self.set_keymap().unwrap();
	}

	fn set_locale(&self) -> Result<()> {
		write_on_file("/mnt/etc/locale.gen", self.locale.clone()).expect("error setting locale");
		Executer::execute(
			Command::new("arch-chroot").arg("/mnt").arg("locale-gen"),
			"error generating locale",
		)
	}

	fn set_lang(&self) -> Result<()> {
		let lang = format!("LANG={}", self.lang);
		Ok(write_on_file("/mnt/etc/locale.conf", lang).expect("error setting  lang"))
	}

	fn set_keymap(&self) -> Result<()> {
		match &self.keymap {
			Some(keymap) => {
				if !keymap.eq("") {
					let km = format!("KEYMAP={}", keymap);
					Ok(write_on_file("/mnt/etc/vconsole.conf", km).expect("error setting keymap"))
				} else {
					Ok(())
				}
			}
			None => Ok(()),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct Network {
	hostname: String,
}

impl Network {
	fn set(&self) {
		self.set_hostname().unwrap();
		self.set_hosts().unwrap();
	}

	fn set_hostname(&self) -> Result<()> {
		Ok(write_on_file("/mnt/etc/hostname", self.hostname.clone())
			.expect("error setting hostname"))
	}

	fn set_hosts(&self) -> Result<()> {
		let localdomain = &format!("127.0.1.1\t{}", self.hostname);

		let hosts = vec!["127.0.0.1\tlocalhost", "::1\t\tlocalhost", localdomain];

		for host in hosts {
			write_on_file("/mnt/etc/hosts", host).expect("error setting /etc/hosts");
		}
		Ok(())
	}
}

pub mod worker {
	use super::*;
	use crate::error::Result;
	use crate::terminal::Executer;
	use std::{
		process::{Command, Stdio},
		str,
	};

	fn gen_fstab() -> Result<()> {
		match Command::new("genfstab")
			.stdout(Stdio::piped())
			.arg("-U")
			.arg("/mnt")
			.output()
		{
			Ok(o) => {
				write_on_file("/mnt/etc/fstab", str::from_utf8(&o.stdout).unwrap()).unwrap();
				Ok(())
			}
			Err(e) => panic!("error generating fstab\n{:?}", e),
		}
	}

	fn set_initramfs() -> Result<()> {
		Executer::execute(
			Command::new("arch-chroot")
				.arg("/mnt")
				.arg("mkinitcpio")
				.arg("-P"),
			"error setting initramfs",
		)
	}

	//TODO: order manager
	pub fn set_system_config(system: System) {
		self::gen_fstab().unwrap();
		system.set_config();
		self::set_initramfs().unwrap();
	}
}
