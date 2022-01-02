use crate::error::*;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

pub fn write_on_file<T, E>(path: T, line: E) -> std::result::Result<(), std::io::Error>
where
	T: AsRef<Path>,
	E: std::fmt::Display,
{
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open(path)
		.unwrap();

	writeln!(file, "{}", line)
}

pub fn mount<T>(device: T, dir: T) -> Result<()>
where
	T: AsRef<Path> + AsRef<OsStr>,
{
	match Command::new("mount").arg(device).arg(dir).status() {
		Ok(status) => {
			if status.success() {
				Ok(())
			} else {
				panic!()
			}
		}
		Err(_e) => panic!(),
	}
}

pub fn umount<T>(device: T) -> Result<()>
where
	T: AsRef<Path> + AsRef<OsStr>,
{
	match Command::new("umount").arg(device).status() {
		Ok(status) => {
			if status.success() {
				Ok(())
			} else {
				panic!()
			}
		}
		Err(_e) => panic!(),
	}
}

pub fn remount<T>(device: T, dir: T) -> Result<()>
where
	T: AsRef<Path> + AsRef<OsStr>,
{
	match umount(&device) {
		Ok(_) => match mount(device, dir) {
			Ok(_) => Ok(()),
			Err(_) => panic!(),
		},
		Err(_) => panic!(),
	}
}

pub fn is_mounted<T>(device: T) -> Result<bool>
where
	T: AsRef<Path> + AsRef<OsStr>,
{
	match Command::new("findmnt").arg(device).output() {
		Ok(output) => {
			if output.status.success() {
				if output.stdout.len() == 0 {
					Ok(false)
				} else {
					Ok(true)
				}
			} else {
				panic!()
			}
		}
		Err(_) => panic!(),
	}
}

pub fn format<T>(device: T, fs: FS) -> Result<()>
where
	T: AsRef<Path> + AsRef<OsStr>,
{
	match Command::new("mkfs")
		.stdin(Stdio::piped())
		.arg("-t")
		.arg(fs.to_string().to_lowercase())
		.arg(device)
		.status()
	{
		Ok(status) => {
			if status.success() {
				Ok(())
			} else {
				panic!()
			}
		}
		Err(_) => panic!(),
	}
}

pub fn activate_swap<T>(device: T) -> Result<()>
where
	T: AsRef<Path> + AsRef<OsStr>,
{
	match Command::new("swapon").arg(device).status() {
		Ok(status) => {
			if status.success() {
				Ok(())
			} else {
				panic!()
			}
		}
		Err(_) => panic!(),
	}
}

pub fn swap_is_active<T>(device: T) -> Result<bool>
where
	T: AsRef<Path> + AsRef<OsStr> + AsRef<str>,
{
	match Command::new("swapon").arg("-s").output() {
		Ok(output) => {
			if output.status.success() {
				if output.stdout.len() == 0 {
					Ok(false)
				} else {
					let out = str::from_utf8(&output.stdout).unwrap();
					if out.contains::<&str>(device.as_ref()) {
						Ok(true)
					} else {
						Ok(false)
					}
				}
			} else {
				panic!()
			}
		}
		Err(_) => panic!(),
	}
}

pub fn path_exists<T>(path: T) -> bool
where
	T: AsRef<Path>,
{
	path.as_ref().exists()
}

pub fn create_if_no_exists<T>(path: T)
where
	T: AsRef<Path>,
{
	if !path_exists(&path) {
		std::fs::create_dir_all(&path).expect("error creating mount point");
	}
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FS {
	Ext2,
	Ext3,
	Ext4,
	Vfat,
	Swap,
	Fat32,
}

impl std::fmt::Display for FS {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}
