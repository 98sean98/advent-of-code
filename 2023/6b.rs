use std::fs;

fn main() {
	let lines = read_lines("6.txt");

	let (l1, l2) = (lines[0].chars().collect(), lines[1].chars().collect());
	let (t, d) = (read_line(l1), read_line(l2));

	println!("t: {}, d: {}", t, d);

	let mut a = 0;
		for i in 1..t {
			let f = i * (t - i);
			if f > d { a = i; break; }
		}

	println!("a: {}", a);
	let s = t - a - a + 1;
	println!("s: {}", s);
}

fn read_line(l: Vec<char>) -> u64 {

	let mut num: u64 = 0;
	for i in 12..l.len() {
		if l[i].is_digit(10) {
			num = num * 10 + l[i].to_digit(10).unwrap() as u64;
		}
	}
	num
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
