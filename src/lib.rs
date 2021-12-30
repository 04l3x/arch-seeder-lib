pub mod bootloader;
pub mod error;
mod fs;
pub mod packages;
pub mod partitions;
pub mod system;
mod terminal;
pub mod users;

pub fn exit() -> error::Result<()> {
	terminal::Executer::execute(
		std::process::Command::new("umount").arg("-R").arg("/mnt"),
		"Error umounting fs",
	)
}
