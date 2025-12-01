use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let mut arguments = std::env::args().skip(1);

	let year = arguments.next().expect("Need year argument");
	let year: u16 = year.parse().expect("Invalid year value");

	let result = match year {
		2022 => y2022::solve_day(arguments),
		2025 => y2025::solve_day(arguments),
		_ => panic!(),
	}?;
	println!("The final result:\n{}", result);

	Ok(())
}
