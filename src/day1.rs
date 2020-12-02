use crate::helper;

/// Find two numbers for which `x + y = 2020`, and return the product
fn find_product(nums: &Vec<i64>) -> Option<i64> {
	for x in nums {
		for y in nums {
			if x + y == 2020 {
				return Some(x * y);
			}
		}
	}
	None
}

/// Find three numbers for which `x + y + z = 2020`, and return the product
fn find_second_product(nums: &Vec<i64>) -> Option<i64> {
	for x in nums {
		for y in nums {
			for z in nums {
				if x + y + z == 2020 {
					return Some(x * y * z);
				}
			}
		}
	}
	None
}

pub fn main() {
	println!("DAY 1");

	let nums = helper::read_lines_int(1)
		.into_iter()
		.filter(|&x| x <= 2020)
		.collect();

	let product = find_product(&nums);
	let product2 = find_second_product(&nums);

	println!(
		"Product found: {}",
		match product {
			Some(n) => n.to_string(),
			None => String::from("haha nope fuck you"),
		}
	);

	println!(
		"Second product found: {}\n",
		match product2 {
			Some(n) => n.to_string(),
			None => String::from("haha nope fuck you"),
		}
	);
}
