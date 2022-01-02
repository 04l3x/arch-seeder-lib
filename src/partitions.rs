use crate::{error::Result, fs};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Partition {
	partition: String,
	device: String,
	fs: fs::FS,
	format: bool,
}

impl Partition {
	fn format(&self) -> Result<()> {
		fs::format(&self.device, self.fs)
	}

	fn mount(&self) -> Result<()> {
		match self.partition.as_str() {
			"swap" => {
				if !fs::swap_is_active(&self.device).expect("") {
					fs::activate_swap(&self.device)
				} else {
					Ok(())
				}
			}

			_ => {
				let path = self.mount_point();

				fs::create_if_no_exists(&path);

				if !fs::is_mounted(&self.device).expect("") {
					fs::mount(&self.device, &path)
				} else {
					fs::remount(&self.device, &path)
				}
			}
		}
	}

	fn mount_point(&self) -> String {
		match self.partition.as_str() {
			"root" => format!("/mnt"),
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
					if mount_point != &"root" {
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
