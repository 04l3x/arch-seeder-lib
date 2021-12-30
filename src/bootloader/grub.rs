use super::*;
use crate::terminal::{pacman::Pacman, Executer};
use std::process::Command;

pub fn install(options: BootOptions) -> Result<()> {
	match options.bootmode {
		BootMode::Uefi => {
			if options.multiboot {
				Pacman::install(vec!["grub", "efibootmgr", "os-prober"]).unwrap();
			} else {
				Pacman::install(vec!["grub", "efibootmgr"]).unwrap();
			}
			uefi(options.removable).unwrap();
		}
		BootMode::Bios => {
			unimplemented!();
		}
	}
	if options.multiboot {
		// running cfg two time for suring os detect
		cfg().unwrap();
	}
	cfg().unwrap();
	Ok(())
}

fn cfg() -> Result<()> {
	Executer::execute(
		Command::new("arch-chroot")
			.arg("/mnt")
			.arg("grub-mkconfig")
			.arg("-o")
			.arg("/boot/grub/grub.cfg"),
		"error generaing grub main config file",
	)
}

fn uefi(removable: bool) -> Result<()> {
	if removable {
		Executer::execute(
			Command::new("arch-chroot")
				.arg("/mnt")
				.arg("grub-install")
				.arg("--target=x86_64-efi")
				.arg("--efi-directory=/boot/efi")
				.arg("--bootloader-id=ArchLinux")
				.arg("--removable"),
			"error installing grub",
		)
	} else {
		Executer::execute(
			Command::new("arch-chroot")
				.arg("/mnt")
				.arg("grub-install")
				.arg("--target=x86_64-efi")
				.arg("--efi-directory=/boot/efi")
				.arg("--bootloader-id=ArchLinux"),
			"error installing grub",
		)
	}
}

fn _bios() -> Result<()> {
	unimplemented!();
}
