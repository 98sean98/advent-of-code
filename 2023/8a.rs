use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("8.txt");

	let mut s = 0;

	let instructions: Vec<char> = lines[0].chars().collect();

	let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
	
	for i in 2..lines.len() {
		let line = lines[i].as_str();
		let n = &line[0..3];
		let (l, r) = (&line[7..10], &line[12..15]);

		map.insert(n, (l, r));
	}

	let mut current: &str = "AAA";
	let mut i = 0;

	while current != "ZZZ" {
		let f = instructions[i];
		let g = map[current];

		current = if f == 'L' { g.0 } else { g.1 };

		s += 1;
		i = if i == instructions.len() - 1 { 0 } else { i + 1 };

	}

	println!("s: {}", s);
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
