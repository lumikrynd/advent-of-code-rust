use std::collections::{HashMap, hash_map::Values};

use aoc_helpers::PuzzleSolver;

mod parsing;

pub struct Solver<'a> {
	cli_lines: Vec<CliLine<'a>>,
}

impl<'a> PuzzleSolver for Solver<'a> {
	fn solve_part_1(&self) -> Option<String> {
		let fs = self.to_dir_structure();
		let result: usize = fs
			.get_all_dirs(vec![])
			.iter()
			.map(|d| d.get_size())
			.filter(|size| *size <= 100000)
			.sum();

		Some(result.to_string())
	}

	fn solve_part_2(&self) -> Option<String> {
		None
	}
}

impl<'a> Solver<'a> {
	pub fn new(input: &'a str) -> Box<Solver<'a>> {
		let s = Solver {
			cli_lines: parsing::parse(input),
		};
		Box::new(s)
	}

	fn to_dir_structure(&'a self) -> FsItem<'a> {
		let mut iter = self.cli_lines.iter();
		iter.next(); //root

		let children = get_dir_content(&mut iter);

		FsItem::Dir {
			name: "/",
			content: children,
		}
	}
}

fn get_dir_content<'a, T>(iter: &mut T) -> DirContent<'a>
where
	T: Iterator<Item = &'a CliLine<'a>>,
{
	let mut content = DirContent::new();

	while let Some(cli) = iter.next() {
		match cli {
			// The nesting of enum types was a mistake XD (but a fun one)
			CliLine::Command(Command::Ls) => {}
			CliLine::Output(LsOutput::Dir { .. }) => {}
			CliLine::Output(LsOutput::File { name, size }) => {
				let size = *size;
				content.add(FsItem::File { name, size });
			}
			CliLine::Command(Command::Cd(Cd::Root)) => {
				unimplemented!("cd root not implemented")
			}
			CliLine::Command(Command::Cd(Cd::Parent)) => {
				break;
			}
			CliLine::Command(Command::Cd(Cd::Dir { name })) => {
				let dir_content = get_dir_content(iter);
				let dir = FsItem::Dir {
					name,
					content: dir_content,
				};
				content.add(dir);
			}
		}
	}

	return content;
}

#[derive(Debug, PartialEq)]
enum FsItem<'a> {
	File {
		name: &'a str,
		size: usize,
	},
	Dir {
		name: &'a str,
		content: DirContent<'a>,
	},
}

#[derive(Debug, PartialEq)]
struct DirContent<'a>(HashMap<&'a str, FsItem<'a>>);

impl<'a> FsItem<'a> {
	/// Return a vector of all directories in the five FdItem recursively
	/// Takes in a list of already found directories which the result should be
	/// added to. (takes ownership and returns the modified version)
	fn get_all_dirs(
		&'a self,
		mut intermediate: Vec<&'a Self>,
	) -> Vec<&'a Self> {
		match self {
			FsItem::File { .. } => intermediate,
			FsItem::Dir { content, .. } => {
				intermediate.push(&self);
				content
					.values()
					.fold(intermediate, |acc, v| v.get_all_dirs(acc))
			}
		}
	}

	fn get_size(&self) -> usize {
		match self {
			FsItem::File { size, .. } => *size,
			FsItem::Dir { content, .. } => {
				content.values().map(|f| f.get_size()).sum()
			}
		}
	}

	fn name(&self) -> &'a str {
		match self {
			FsItem::File { name, .. } => name,
			FsItem::Dir { name, .. } => name,
		}
	}

	#[allow(dead_code)] //for easier test
	fn create_dir(name: &'a str, content: Vec<FsItem<'a>>) -> FsItem<'a> {
		FsItem::Dir {
			name,
			content: DirContent::new_from(content),
		}
	}
}

impl<'a> DirContent<'a> {
	fn new() -> DirContent<'a> {
		DirContent(HashMap::new())
	}

	fn add(&mut self, item: FsItem<'a>) {
		let name = item.name();
		self.0.insert(name, item);
	}

	fn values(&self) -> Values<'_, &'a str, FsItem<'a>> {
		self.0.values()
	}

	fn new_from(content: Vec<FsItem>) -> DirContent {
		let mut res = DirContent::new();
		for item in content {
			res.add(item);
		}
		res
	}
}

#[derive(Debug, PartialEq)]
enum CliLine<'a> {
	Output(LsOutput<'a>),
	Command(Command<'a>),
}

#[derive(Debug, PartialEq)]
enum LsOutput<'a> {
	Dir { name: &'a str },
	File { name: &'a str, size: usize },
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
	Cd(Cd<'a>),
	Ls,
}

#[derive(Debug, PartialEq)]
enum Cd<'a> {
	Root,
	Parent,
	Dir { name: &'a str },
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn get_dir_content_test() {
		let input = vec![
			cmd_ls(),
			out_file("a", 3),
			out_dir("subdir"),
			out_file("b", 5),
			cmd_cd_dir("subdir"), // Into inner
			cmd_ls(),
			out_file("c", 35),
			out_file("d", 42),
			cmd_cd_parent(), // out of inner
			cmd_cd_parent(), // finish
			cmd_cd_dir("dir_not_in_folder"),
		];
		let mut iter = input.iter();
		let result = get_dir_content(&mut iter);

		let (Some(_), None) = (iter.next(), iter.next()) else {
			panic!("Shouldn't remove anything after exiting dir with 'Cd ..'")
		};

		let expected = DirContent::new_from(vec![
			fs_file("a", 3),
			fs_file("b", 5),
			fs_dir(
				"subdir",
				vec![
					fs_file("c", 35), // don't format this line
					fs_file("d", 42),
				],
			),
		]);

		assert_eq!(result, expected);
	}

	fn fs_file<'a>(name: &'a str, size: usize) -> FsItem<'a> {
		FsItem::File { name, size }
	}

	fn fs_dir<'a>(name: &'a str, content: Vec<FsItem<'a>>) -> FsItem<'a> {
		FsItem::create_dir(name, content)
	}

	fn cmd_ls() -> CliLine<'static> {
		CliLine::Command(Command::Ls)
	}

	fn cmd_cd_parent() -> CliLine<'static> {
		CliLine::Command(Command::Cd(Cd::Parent))
	}

	fn cmd_cd_dir<'a>(name: &'a str) -> CliLine<'a> {
		CliLine::Command(Command::Cd(Cd::Dir { name }))
	}

	fn out_file<'a>(name: &'a str, size: usize) -> CliLine<'a> {
		CliLine::Output(LsOutput::File { name, size })
	}

	fn out_dir<'a>(name: &'a str) -> CliLine<'a> {
		CliLine::Output(LsOutput::Dir { name })
	}
}

#[cfg(test)]
mod fs_item_tests {
	use super::*;

	#[test]
	fn fs_item_list_dirs_for_file() {
		let size = 1;
		let item = file("", size);
		assert_eq!(item.get_size(), size);
	}

	#[test]
	fn fs_item_list_dirs_for_dir() {
		let root_dir = dir(
			"root_dir",
			vec![
				dir(
					"dir",
					vec![
						file("a", 3), // don't format this line
						file("b", 6),
					],
				),
				file("c", 5),
			],
		);

		assert_eq!(root_dir.get_size(), 14);
	}

	#[test]
	fn fs_item_get_all_dirs() {
		let dir1_name = "dir";
		let dir2_name = "root_dir";

		let root_dir = dir(
			dir2_name,
			vec![
				dir(
					dir1_name,
					vec![
						file("a", 3), // don't format this line
						file("b", 6),
					],
				),
				file("c", 5),
			],
		);

		let dir_names: Vec<_> = root_dir
			.get_all_dirs(vec![])
			.iter()
			.map(|f| f.name())
			.collect();

		let expected = vec![dir2_name, dir1_name];
		assert_eq!(dir_names, expected);
	}

	fn file<'a>(name: &'a str, size: usize) -> FsItem<'a> {
		FsItem::File { name, size }
	}

	fn dir<'a>(name: &'a str, content: Vec<FsItem<'a>>) -> FsItem<'a> {
		FsItem::create_dir(name, content)
	}
}
