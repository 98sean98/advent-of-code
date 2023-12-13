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

	'loop_p: for (p, pattern) in patterns.iter_mut().enumerate() {
		// find original line
		let h_k = find_h_mir(pattern, 0).unwrap_or(0);
		let v_k = find_v_mir(pattern, 0).unwrap_or(0);

		for a in 0..pattern.len() {
			for b in 0..pattern[0].len() {
				let prev: char = pattern[a][b];
				pattern[a][b] = if prev == '.' { '*' } else { '.' };
				// find horizontal mirror
				if let Some(k) = find_h_mir(pattern, h_k) {
					s += 100 * k;
					continue 'loop_p;
				}

				// find vertial mirror
				if let Some(k) = find_v_mir(pattern, v_k) {
					s += k;
					continue 'loop_p;
				}

				pattern[a][b] = prev;
			}
		}
	}

	println!("s: {}", s);
}

fn ppattern(pattern: &Vec<Vec<char>>) {
	for row in pattern.iter() {
		for p in row.iter() {
			print!("{}", p);
		}
		println!("");
	}
}

fn find_h_mir(pattern: &Vec<Vec<char>>, old: usize) -> Option<usize> {
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
			if is_mirror && old != k {
				// println!("[{}]: {}", p, k);
				return Some(k);
			}
		}
	None
}

fn find_v_mir(pattern: &Vec<Vec<char>>, old: usize) -> Option<usize> {
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
			if is_mirror && old != k {
				// println!("[{}]: {}", p, k);
				return Some(k);
			}
		}
	None
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

