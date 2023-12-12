use std::fs;

fn main() {
	let lines = read_lines("12.txt");

	let mut s = 0;

	for line in lines.iter() {
		let l: Vec<char> = line.chars().collect();

		let mut row: Vec<char> = vec![];
		let mut groups: Vec<u32> = vec![];
		let mut unknowns: Vec<usize> = vec![];
		let mut space = false;

		let mut num = 0;

		for (j, c) in l.iter().enumerate() {
			if *c == ' ' {
				space = true;
			} else if !space {
				row.push(*c);
				if *c == '?' { unknowns.push(j); }
			} else if c.is_digit(10) {
				num = num * 10 + c.to_digit(10).unwrap();
			}
			if j == l.len() - 1 || l[j+1] == ',' {
				groups.push(num);
				num = 0;
			}
		}

		/*
		prow(&row);
		print!("groups: ");
		for g in groups.iter() { print!("{} ", g); }
		println!("");
		*/

		s += count(&mut row, &mut unknowns, &groups);
		
	}

	println!("s: {}", s);
}

fn count(row: &mut Vec<char>, unknowns: &mut Vec<usize>, groups: &Vec<u32>) -> u32 {

	let mut s = 0;

	if let Some(last) = unknowns.pop() {
		row[last] = '.';

		s += count(row, unknowns, groups);

		row[last] = '#';
		
		s += count(row, unknowns, groups);

		unknowns.push(last);
	} else {

		let mut current: u32 = 0;
		let mut candidate: Vec<u32> = vec![];
		
		for (j, c) in row.iter().enumerate() {
			if *c == '#' { current += 1; }
			if current > 0 && (*c == '.' || j == row.len() - 1) {
				candidate.push(current);
				current = 0;
			}
		}

		// print!("candidate:");
		// for c in candidate.iter() { print!(" {}", c); }
		// println!("");

		if candidate.eq(groups) {
			// prow(row);
			return 1;
		} else {
			return 0;
		}
	}

	s
}

fn prow(row: &Vec<char>) {
	for c in row.iter() {
		print!("{} ", c);
	}
	println!("");
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

