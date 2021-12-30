mod grub;

use crate::error::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BootOptions {
	#[serde(default = "defaults::removable")]
	removable: bool,
	#[serde(default = "defaults::multiboot")]
	multiboot: bool,
	bootmode: BootMode,
	bootloader: Bootloaders,
}

pub fn install(options: BootOptions) -> Result<()> {
	match options.bootloader {
		Bootloaders::Grub => {
			grub::install(options).unwrap();
		}
	}
	Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BootMode {
	Uefi,
	Bios,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Bootloaders {
	Grub,
}

mod defaults {
	pub fn removable() -> bool {
		false
	}

	pub fn multiboot() -> bool {
		false
	}
}
