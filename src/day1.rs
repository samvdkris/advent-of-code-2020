use crate::helper;

fn find_product(nums: &Vec<i64>) -> Option<i64> {
	for x in nums {
		for y in nums {
			if x + y == 2020 {
				return Some(x * y);
			}
		}
	}
	return None;
}

pub fn main() {
	println!("DAY 1");

	let nums = helper::read_lines_int(1)
		.into_iter()
		.filter(|&x| x <= 2020)
		.collect();

	let product = find_product(&nums);

	println!(
		"Product found: {}\n",
		match product {
			Some(n) => n.to_string(),
			None => String::from("haha nope fuck you"),
		}
	);
}
