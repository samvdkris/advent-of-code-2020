use std::{
	fs::File,
	io::{BufRead, BufReader},
};

/// Reads the text file for a certain day and return a `Vec<String>` of the lines
///
/// # Example
/// ```
/// let lines = helper::read_lines(1);
/// ```
pub fn read_lines(day: u8) -> Vec<String> {
	let path = format!("input/day{}.txt", day);
	let file = File::open(path).expect("Could not open file");
	let reader = BufReader::new(file);

	return reader
		.lines()
		.map(|x| x.expect("Could not read line"))
		.collect();
}

/// Reads the text file for a certain day and return a `Vec<i64>` of the lines
///
/// # Example
/// ```
/// let lines = helper::read_lines_int(1);
/// ```
pub fn read_lines_int(day: u8) -> Vec<i64> {
	let lines = read_lines(day);

	return lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
}
