use crate::{error::Result, terminal::Executer};
use std::process::Command;

pub struct Pacman;

impl Pacman {
	pub fn install(packages: impl IntoIterator<Item = &'static str>) -> Result<()> {
		Executer::execute(
			Command::new("arch-chroot")
				.arg("/mnt")
				.arg("pacman")
				.arg("-Syu")
				.args(packages),
			"pacman error on install packages",
		)
	}
}

/*#[cfg(test)]
mod test {
	#[test]
	fn build() {}

}*/
