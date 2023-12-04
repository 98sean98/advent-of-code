use std::fs;


fn main() {
	let input = fs::read_to_string("2.txt").unwrap();

	let mut r: Vec<&str> = input.split("\n").collect();
	r.pop();

	println!("{}", r.len());

	let r: Vec<(char, char)> = 
		r.iter().map(|x| {let y = x.as_bytes(); (y[0] as char, y[2] as char)}).collect();
	
	println!("{} {}", r[0].0, r[0].1);


	let sum = 
		r.iter().fold(0, (|acc, x| {
			acc + match x {
			('A', 'X') => 0 + 3,
			('A', 'Y') => 3 + 1,
			('A', 'Z') => 6 + 2,
			('B', 'X') => 0 + 1,
			('B', 'Y') => 3 + 2,
			('B', 'Z') => 6 + 3,
			('C', 'X') => 0 + 2,
			('C', 'Y') => 3 + 3,
			('C', 'Z') => 6 + 1,
			_ => 0,
			}

			}));

	println!("sum: {}", sum);
}
