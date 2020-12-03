use std::{str::FromStr, string::ParseError};

use regex::Regex;

use crate::helper;

#[derive(Debug)]
struct Policy {
	character: char,
	number_from: usize,
	number_to: usize,
}

#[derive(Debug)]
struct Password {
	password: String,
	policy: Policy,
}

impl FromStr for Password {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): (.*)").unwrap();
		let m = re.captures(s).unwrap();

		let number_from = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
		let number_to = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
		let character = m.get(3).unwrap().as_str().parse::<char>().unwrap();
		let password = m.get(4).unwrap().as_str().to_string();

		Ok(Password {
			password,
			policy: Policy {
				character,
				number_from,
				number_to,
			},
		})
	}
}

impl Password {
	/// Check the validity of a password
	// Prooooooooobably not the best way to do this.
	fn check_validity(&self) -> bool {
		(self.policy.number_from..self.policy.number_to + 1).contains(&(self.password.split(self.policy.character).count() - 1))
	}

	/// Check the validity of a password for real this time
	fn check_validity2(&self) -> bool {
		(self.password.chars().nth(self.policy.number_from - 1).unwrap() == self.policy.character)
			^ (self.password.chars().nth(self.policy.number_to - 1).unwrap() == self.policy.character)
	}
}

pub fn main() {
	println!("DAY 2");

	let lines = helper::read_lines(2);

	let count = lines.iter().map(|x| x.parse::<Password>().unwrap()).filter(Password::check_validity).count();
	let count2 = lines.iter().map(|x| x.parse::<Password>().unwrap()).filter(Password::check_validity2).count();

	println!("Valid passwords: {}", count);
	println!("Actually valid passwords: {}", count2);
}
