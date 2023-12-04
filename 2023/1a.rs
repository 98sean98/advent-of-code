use std::fs;

fn main() {
	let lines = read_lines("1.txt");
	
	let mut s = 0;
	let mut count = 0;
	for line in lines {
		let mut i = 0;
		let l = line.as_bytes();
		let a = loop {
			let c = l[i] as char;
			if (c.is_digit(10)) {
				break c.to_digit(10).unwrap();
			}
			i += 1; 
		};
		i = line.len() - 1;
		let b = loop {
			let c = l[i] as char;
			if c.is_digit(10) {
				break c.to_digit(10).unwrap();
			}
			i -= 1;
		};
		
		s += a * 10 + b;

	}

	println!("s: {}", s);
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

