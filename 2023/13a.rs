use std::fs;

fn main() {
	let lines = read_lines("13.txt");

	let mut patterns: Vec<Vec<Vec<char>>> = vec![vec![]];

	for line in lines.iter() {
		if line.len() == 0 {
			patterns.push(vec![]);
			continue;
		}

		let l = line.chars().collect();
		let index = patterns.len() - 1;
		patterns[index].push(l);
	}


	let mut s = 0;

	'loop_p: for (p, pattern) in patterns.iter().enumerate() {
		// find horizontal mirror
		for k in 1..=pattern.len()-1 {
			let mut is_mirror = true;
			for i in 0..k {
				let j = 2 * k - 1 - i;
				// println!("k: {}, i: {}, j: {}", k, i, j);
				if j < pattern.len() {
					is_mirror = is_mirror && h_mir(pattern, i, j);
				}
				if !is_mirror { break; }
			}
			if is_mirror {
				// println!("[{}]: {}", p, k);
				s += 100 * k;
				continue 'loop_p;
			}
		}

		// find vertial mirror
		let columns = pattern[0].len();
		for k in 1..=columns-1 {
			let mut is_mirror = true;
			for i in 0..k {
				let j = 2 * k - 1 - i;
				// println!("k: {}, i: {}, j: {}", k, i, j);
				if j < columns {
					is_mirror = is_mirror && v_mir(pattern, i, j);
				}
				if !is_mirror { break; }
			}
			if is_mirror {
				// println!("[{}]: {}", p, k);
				s += k;
				continue 'loop_p;
			}
		}

	}

	println!("s: {}", s);
}

fn h_mir(pattern: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
	pattern[i].eq(&pattern[j])
}

fn v_mir(pattern: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
	pattern.iter().all(|p| p[i] == p[j])
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

