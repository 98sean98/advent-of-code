use std::fs;

fn main() {
	let lines = read_lines("1.txt");
	
	let mut s = 0;
	let mut count = 0;
	for line in lines {
		let mut i: usize = 0;
		let l = line.as_bytes();
		let a = loop {
			let c = l[i] as char;
			if c.is_digit(10) {
				break c.to_digit(10).unwrap();
			}
			let p = w(&l, l.len(), i);
			if p < 10 { break p; }
			i += 1; 
		};
		i = line.len() - 1;
		let b = loop {
			let c = l[i] as char;
			if c.is_digit(10) {
				break c.to_digit(10).unwrap();
			}
			let p = x(&l, i);
			if p < 10 { break p; }
			i -= 1;
		};
		
		s += a * 10 + b;

	}

	println!("s: {}", s);
}

const words : [&str; 9] = [
	"one",
	"two",
	"three",
	"four",
	"five",
	"six",
	"seven",
	"eight",
	"nine"
];

fn w(l: &[u8], length: usize, i: usize) -> u32 {
		for wi in 0..9 {
			let wl = words[wi].len();
			let k = i + wl;
			if k < length && &l[i..k] == words[wi].as_bytes() {
				// println!("{}", words[wi]);
				return (wi + 1) as u32;
			}
		}
	10
}

fn x(l: &[u8], i: usize) -> u32 {
	for wi in 0..9 {
		let wl = words[wi].len();
		if i+1 >= wl {
			if &l[(i+1-wl)..(i+1)] == words[wi].as_bytes() {
				// println!("{}", words[wi]);
				return (wi + 1) as u32;
			}
		}
	}
	10
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

