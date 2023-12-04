use std::fs;


fn main() {
	let input= fs::read_to_string("input.txt").unwrap();

	let groups: Vec<&str> = input.split("\n\n").collect();

	println!("{}", groups.len());

	let mut cs: Vec<i32> = groups.into_iter().map(|g| {
		let cs: Vec<&str> = g.split("\n").collect();
		let csi: Vec<i32> = cs.iter().map(|c| c.parse::<i32>().unwrap_or(0)).collect();
		csi.iter().fold(0, |acc, x| acc + x)
		}).collect();

	println!("{}", cs.len());

	cs.sort();

	println!("largest: {}", cs[cs.len() - 1]);

	let top3 = cs[(cs.len() - 3)..].iter().fold(0, |acc, x| acc + x);

	println!("top3: {}", top3);
}

