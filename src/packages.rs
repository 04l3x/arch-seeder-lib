use crate::{error::Result, terminal::Executer};
use serde::Deserialize;
use std::process::Command;

trait Install {
	fn install(&self) -> Result<()>;
}

#[derive(Debug, Deserialize)]
pub struct Packages {
	packages: Vec<String>,
}

impl Install for Packages {
	fn install(&self) -> Result<()> {
		Executer::execute(
			Command::new("pacstrap").arg("/mnt").args(&self.packages),
			"Error during installation of base packages",
		)
	}
}

pub mod worker {
	use super::*;

	pub fn install(packages: Packages) {
		packages.install().unwrap();
	}
}
