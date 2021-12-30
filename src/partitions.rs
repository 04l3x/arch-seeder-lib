use crate::error::Result;
use serde::Deserialize;
use std::collections::HashMap;

//TODO: detach and replace for fs module
use std::path::PathBuf;

//TODO: detach and replace for terminal module
use std::process::{Command, Output};

#[derive(Debug, Deserialize)]
pub struct Partition {
	partition: String,
	device: PathBuf,
	fs: FS,
	format: bool,
}

impl Partition {
	fn format(&self) -> Result<()> {
		match Command::new("mkfs")
			.arg("-t")
			.arg(self.fs.to_string().to_lowercase())
			.arg(self.device.clone())
			.output()
		{
			Ok(output) => {
				if output.status.success() {
					Ok(())
				} else {
					panic!("{:?}", String::from_utf8(output.stderr).expect(""));
				}
			}
			Err(e) => panic!(
				"something went wrong at formating {:?}... \n {:?}",
				self.device, e
			),
		}
	}

	///if partition is swap activate is not mount
	fn mount(&self) -> Result<()> {
		let mount_path: PathBuf = self.mount_point().into();

		let is_success = |output: Output| -> Result<()> {
			if output.status.success() {
				Ok(())
			} else {
				panic!("{:?}", String::from_utf8(output.stderr).expect(""));
			}
		};

		match self.partition.as_str() {
			"swap" => match Command::new("swapon").arg(self.device.clone()).output() {
				Ok(output) => is_success(output),
				Err(e) => panic!(
					"something went wrong at activate swap partition {:?}... \n {:?}",
					self.device, e
				),
			},
			_ => {
				if !mount_path.exists() {
					std::fs::create_dir_all(&mount_path).expect("error creating mount point");
				}
				match Command::new("mount")
					.arg(self.device.clone())
					.arg(mount_path)
					.output()
				{
					Ok(output) => is_success(output),
					Err(e) => panic!(
						"something went wrong at mounting {:?} partition... \n {:?}",
						self.device, e
					),
				}
			}
		}
	}

	/// returns mount point based on partition
	fn mount_point(&self) -> impl Into<PathBuf> {
		match self.partition.as_str() {
			"root" => format!("/mnt/"),
			"boot" => format!("/mnt/boot/efi"), //FIXME: select from boot mode (uefi or mbr)
			_ => format!("/mnt/{}", self.partition),
		}
	}
}

pub mod worker {
	use super::*;

	pub fn mount_file_system(partitions: Vec<Partition>) {
		let fs = partitions_to_hashmap(partitions);

		match fs.get("root") {
			Some(root) => {
				if root.format {
					root.format().unwrap();
				}
				root.mount().unwrap();

				for (mount_point, partition) in fs.iter() {
					if mount_point != "root" {
						if partition.format {
							partition.format().unwrap();
						}
						partition.mount().unwrap();
					}
				}
			}
			None => panic!("error root partition is needed!"),
		}
	}

	fn partitions_to_hashmap(partitions: Vec<Partition>) -> HashMap<String, Partition> {
		let mut hm = HashMap::new();

		for p in partitions {
			hm.insert(p.partition.clone(), p);
		}

		hm
	}
}

#[derive(Debug, Deserialize)]
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
