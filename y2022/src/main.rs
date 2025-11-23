use std::error::Error;

use y2022::solve_day;

fn main() -> Result<(), Box<dyn Error>> {
	let mut arguments = std::env::args().skip(1);

	let day = arguments.next();
	let result = solve_day(day)?;
	println!("The final result:\n{}", result);

	Ok(())
}
