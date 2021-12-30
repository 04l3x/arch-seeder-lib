pub mod pacman;

use crate::error::*;
use std::io::Write;
use std::process::{Command as Cmd, Stdio};

pub struct Executer;

impl Executer {
	pub fn execute<'a>(command: &mut Cmd, panic_msg: &'a str) -> Result<()> {
		match command.spawn() {
			Ok(mut child) => match child.wait() {
				Ok(status) => {
					if status.success() {
						Ok(())
					} else {
						Err(Box::new(Error {}))
					}
				}
				Err(err) => {
					panic!("{} \n {:?}", panic_msg, err);
				}
			},
			Err(err) => {
				panic!("{} \n {:?}", panic_msg, err);
			}
		}
	}

	pub fn execute_with_input<'a>(
		command: &mut Cmd,
		panic_msg: &'a str,
		ordered_inputs: impl IntoIterator<Item = &'a str>,
	) -> Result<()> {
		command.stdin(Stdio::piped());

		let input = PromtInput::new(ordered_inputs).input();
		match command.spawn() {
			Ok(mut child) => {
				child
					.stdin
					.as_ref()
					.unwrap()
					.write(input.as_bytes())
					.unwrap();

				match child.wait() {
					Ok(status) => {
						if status.success() {
							Ok(())
						} else {
							Err(Box::new(Error {}))
						}
					}
					Err(err) => {
						panic!("{} \n {:?}", panic_msg, err);
					}
				}
			}
			Err(err) => {
				panic!("{} \n {:?}", panic_msg, err);
			}
		}
	}
}

struct PromtInput<'a, T>
where
	T: IntoIterator<Item = &'a str>,
{
	inputs: T,
}

impl<'a, T> PromtInput<'a, T>
where
	T: IntoIterator<Item = &'a str>,
{
	fn new(ordered_inputs: T) -> PromtInput<'a, T> {
		PromtInput {
			inputs: ordered_inputs.into(),
		}
	}

	fn input(self) -> String {
		let mut input = String::from("");

		for i in self.inputs.into_iter() {
			input.push_str(i);
			input.push('\n');
		}

		input
	}
}
