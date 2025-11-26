use super::*;

pub fn parse<'a>(input: &'a str) -> Vec<CliLine<'a>> {
	input.lines().map(parse_line).collect()
}

fn parse_line<'a>(line: &'a str) -> CliLine<'a> {
	if line.starts_with("$ ") {
		let command = parse_command(&line[2..]);
		CliLine::Command(command)
	} else {
		let output = parse_output(line);
		CliLine::Output(output)
	}
}

fn parse_output<'a>(line: &'a str) -> LsOutput<'a> {
	if line.starts_with("dir ") {
		LsOutput::Dir { name: &line[4..] }
	} else {
		let words = line.split_whitespace().collect::<Vec<&str>>();
		let [size, name] = words[..] else {
			panic!();
		};

		LsOutput::File {
			name: name,
			size: size.parse().unwrap(),
		}
	}
}

fn parse_command<'a>(command: &'a str) -> Command<'a> {
	match &command[..2] {
		"ls" => Command::Ls,
		"cd" => Command::Cd(parse_dir(&command[3..])),
		_ => panic!("Invalid"),
	}
}

fn parse_dir<'a>(dir: &'a str) -> Cd<'a> {
	match dir {
		"/" => Cd::Root,
		".." => Cd::Parent,
		x => Cd::Dir { name: x },
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
			name: "file_name",
			size: 654152,
		});
		assert_eq!(result, expected);
	}

	#[test]
	fn parse_output_dir() {
		let line = "dir dir_name";
		let result = parse_line(line);

		let expected = CliLine::Output(LsOutput::Dir { name: "dir_name" });
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

		let expected = CliLine::Command(Command::Cd(Cd::Dir { name: "a_folder" }));

		assert_eq!(result, expected);
	}
}
