use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("8.txt");

	let instructions: Vec<char> = lines[0].chars().collect();
	let mut currents: Vec<&str> = vec![];

	let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
	
	for i in 2..lines.len() {
		let line = lines[i].as_str();
		let n = &line[0..3];
		let (l, r) = (&line[7..10], &line[12..15]);
		
		if &n[2..] == "A" { currents.push(n); }

		map.insert(n, (l, r));
	}

	let mut steps: Vec<u64> = Vec::with_capacity(currents.len());

	for c in currents.iter_mut() {
		let mut i = 0;
		let mut step = 0;
		while &c[2..] != "Z" {
			let f = instructions[i];

			let g = map[*c];
			*c = if f == 'L' { g.0 } else { g.1 };

			step += 1;
			i = if i == instructions.len() - 1 { 0 } else { i + 1 };
		}	
		steps.push(step);
	}

	// compute LCM of all steps
	let s: u64 = steps.iter().fold(1, |a, &b| a * b / gcd(a, b));
	println!("s: {}", s);

}

fn gcd(a: u64, b: u64) -> u64 {
	let (mut c, mut d) = (a, b);
	while d != 0 {
		let t = d;
		d = c % d;
		c = t;
	}
	return c;
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
