use std::fs;

fn main() {
	let lines = read_lines("15.txt");
	
	let line = lines[0].as_bytes();
	
	let mut s: u64 = 0;
	let mut n: u64 = 0;

	for &c in line.iter() {
		if c != b',' {
			n = hash(c, n);
		} else {
			s += n;
			n = 0;
		}
	}

	s += n;

	println!("s: {}", s);
}

fn hash(c: u8, n: u64) -> u64 {
	let p = n + c as u64;
	(p * 17) % 256
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

