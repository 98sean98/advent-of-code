use std::fs;
use std::collections::HashSet;


fn main() {
	let input = fs::read_to_string("3.txt").unwrap();

	let mut r: Vec<&str> = input.split("\n").collect();

	r.pop();

	println!("{}", r.len());

	let mut q : Vec<char> = Vec::new();

	for i in 0..(r.len() / 3) {
		let j = i * 3;

		let a = r[j];
		let b: HashSet<char> = r[j + 1].chars().collect();
		let c: HashSet<char> = r[j + 2].chars().collect();

		let d: Vec<char> = a.chars().filter(|p| b.contains(&p)).collect();
		let e = d.iter().find(|p| c.contains(&p)).unwrap();

		q.push(*e);

	}

	println!("{}", q.len());
	println!("{}", q[0]);

	println!("{} {} {} {}", 'a' as u8, 'z' as u8, 'A' as u8, 'Z' as u8);


	let r : Vec<i32> = q.iter().map(|x| {
		let y = *x as u8;

		if 65 <= y && y <= 90 {
			return (y - 38) as i32;
		}
		else {
			return (y - 96) as i32;
		}
		}).collect();

	let s :i32 = r.iter().sum();

	println!("sum: {}", s);

}
