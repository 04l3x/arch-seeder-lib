use crate::error::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn write_on_file(
	path: impl AsRef<Path>,
	line: impl std::fmt::Display,
) -> std::result::Result<(), std::io::Error> {
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open(path)
		.unwrap();

	writeln!(file, "{}", line)
}

fn _mount() -> Result<()> {
	Ok(())
}

fn _umount() -> Result<()> {
	Ok(())
}

fn _remount() -> Result<()> {
	Ok(())
}

fn _is_mounted() -> Result<bool> {
	Ok(true)
}
