use std::fs;

fn main() {
	let lines = read_lines("6.txt");

	let (l1, l2) = (lines[0].chars().collect(), lines[1].chars().collect());
	let (times, distances) = (read_line(l1), read_line(l2));

	let mut s = 1;

	for i in 0..times.len() {
		let (t, d) = (times[i], distances[i]);
		
		let mut w = 0;
		for j in 1..t {
			let f = j * (t - j);
			if f > d { w += 1; }
		}
		s *= w;
	}

	println!("s: {}", s);
}

fn read_line(l: Vec<char>) -> Vec<u32> {

	let mut v: Vec<u32> = vec![];
	let mut num = 0;
	for i in 12..l.len() {
		if l[i].is_digit(10) {
			num = num * 10 + l[i].to_digit(10).unwrap();
		}
		if num > 0 && (i == l.len() - 1 || l[i+1] == ' ') {
			v.push(num);
			num = 0;
		}
	}
	v
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
