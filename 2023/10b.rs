use std::fs;
use std::collections::HashMap;


fn main() {
	let lines = read_lines("10c.txt");

	let pipes: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

	let mut start: (usize, usize) = (0, 0);

	'l: for (i, pipe) in pipes.iter().enumerate() {
		for (j, p) in pipe.iter().enumerate() {
			if *p == 'S' { start = (i, j); break 'l; }
		}
	}

	println!("start: ({}, {})", start.0, start.1);

	let dirs: HashMap<char, HashMap<char, &str>> = {
		let mut dirs: HashMap<char, HashMap<char, &str>> = HashMap::with_capacity(6);
		let mut h1 = HashMap::with_capacity(2);
		h1.insert('S', "up");
		h1.insert('N', "down");
		dirs.insert('|', h1);
		let mut h2 = HashMap::with_capacity(2);
		h2.insert('E', "left");
		h2.insert('W', "right");
		dirs.insert('-', h2);
		let mut h3 = HashMap::with_capacity(2);
		h3.insert('N', "right");
		h3.insert('E', "up");
		dirs.insert('L', h3);
		let mut h4 = HashMap::with_capacity(2);
		h4.insert('N', "left");
		h4.insert('W', "up");
		dirs.insert('J', h4);
		
		let mut h5 = HashMap::with_capacity(2);
		h5.insert('S', "left");
		h5.insert('W', "down");
		dirs.insert('7', h5);
		let mut h6 = HashMap::with_capacity(2);
		h6.insert('S', "right");
		h6.insert('E', "down");
		dirs.insert('F', h6);
		dirs
	};
	
	let firsts = vec![
		(start.0 - 1, start.1, vec!['7', '|', 'F'], 'S'),
		(start.0, start.1 - 1, vec!['-', 'L', 'F'], 'E'),
		(start.0, start.1 + 1, vec!['-', 'J', '7'], 'W'),
		(start.0 + 1, start.1, vec!['J', '|', 'L'], 'N')
	];

	let mut current: ((usize, usize), char, char) = ((0, 0), '-', 'E');
	for f in firsts.iter() {
		if let Some(p) = f.2.iter().find(|&&v| v == pipes[f.0][f.1]) {
			current = ((f.0, f.1), *p, f.3);
			break;
		}
	}

	let mut s: u32 = 0;

	let mut main: Vec<Vec<usize>> = vec![vec![]; pipes.len()];
	main[current.0.0].push(current.0.1);
	
	while current.1 != 'S' {
		let dir: &str = &dirs[&current.1][&current.2];

		// println!("dir: {}", dir);

		let (pos, from) = if dir == "up" {
			((current.0.0 - 1, current.0.1), 'S')
		} else if dir == "left" {
			((current.0.0, current.0.1 - 1), 'E')
		} else if dir == "right" {
			((current.0.0, current.0.1 + 1), 'W')
		} else {
			((current.0.0 + 1, current.0.1), 'N')
		};

		let pipe = pipes[pos.0][pos.1];
		current = (pos, pipe, from);

		main[pos.0].push(pos.1);
		
	}

	main.iter_mut().for_each(|v| v.sort());

	for (i, p) in main.iter().enumerate() {
		print!("{}: ", i);
		for j in p.iter() { print!("{} ", j) }
		println!("");
	}


	for (i, row) in pipes.iter().enumerate() {
		let mut count = 0;
		for (j, p) in row.iter().enumerate() {
			if main[i].iter().any(|&x| x == j) {
			count += 1;
			} else if count % 2 == 1 && j < *main[i].last().unwrap() {
				println!("({}, {})", i, j);
				s += 1
			}
		}
	}

	
	println!("s: {}", s);
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

