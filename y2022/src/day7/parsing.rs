use super::*;

pub fn parse(input: &str) -> Solver {
	let lines = input.lines().map(parse_line).collect();
	Solver(lines)
}

fn parse_line(line: &str) -> CliLine {
	if line.starts_with("$ ") {
		let command = parse_command(&line[2..]);
		CliLine::Command(command)
	} else {
		let output = parse_output(line);
		CliLine::Output(output)
	}
}

fn parse_output(line: &str) -> LsOutput {
	if line.starts_with("dir ") {
		LsOutput::Dir {
			name: line[4..].to_string(),
		}
	} else {
		let words = line.split_whitespace().collect::<Vec<&str>>();
		let [size, name] = words[..] else {
			panic!();
		};

		LsOutput::File {
			name: name.to_string(),
			size: size.parse().unwrap(),
		}
	}
}

fn parse_command(command: &str) -> Command {
	match &command[..2] {
		"ls" => Command::Ls,
		"cd" => Command::Cd(parse_dir(&command[3..])),
		_ => panic!("Invalid"),
	}
}

fn parse_dir(dir: &str) -> Cd {
	match dir {
		"/" => Cd::Root,
		".." => Cd::Parent,
		x => Cd::Dir {
			name: x.to_string(),
		},
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_output_file() {
		let line = "654152 file_name";
		let result = parse_line(line);

		let expected = CliLine::Output(LsOutput::File {
			name: "file_name".to_string(),
			size: 654152,
		});
		assert_eq!(result, expected);
	}

	#[test]
	fn parse_output_dir() {
		let line = "dir dir_name";
		let result = parse_line(line);

		let expected = CliLine::Output(LsOutput::Dir {
			name: "dir_name".to_string(),
		});
		assert_eq!(result, expected);
	}

	#[test]
	fn parse_ls_test() {
		let line = "$ ls";
		let result = parse_line(line);

		assert_eq!(result, CliLine::Command(Command::Ls));
	}

	#[test]
	fn parse_cd_root_test() {
		let line = "$ cd /";
		let result = parse_line(line);

		assert_eq!(result, CliLine::Command(Command::Cd(Cd::Root)));
	}

	#[test]
	fn parse_cd_parrent_test() {
		let line = "$ cd ..";
		let result = parse_line(line);

		assert_eq!(result, CliLine::Command(Command::Cd(Cd::Parent)));
	}

	#[test]
	fn parse_cd_dir_test() {
		let line = "$ cd a_folder";
		let result = parse_line(line);

		let expected = CliLine::Command(Command::Cd(Cd::Dir {
			name: "a_folder".to_string(),
		}));

		assert_eq!(result, expected);
	}
}
