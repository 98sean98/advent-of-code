use std::fs;

fn main() {
	let lines = read_lines("12a.txt");

	let mut s = 0;

	for (i, line) in lines.iter().enumerate() {
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

		let mut e_row: Vec<char> = Vec::with_capacity(row.len() * 2 + 4);
		let mut e_unknowns: Vec<usize> = Vec::with_capacity(unknowns.len() * 2);
		let mut e_groups: Vec<u32> = Vec::with_capacity(groups.len() * 2);

		for j in 0..2 {
			let mut c_row = row.clone();
			e_row.append(&mut c_row);

			let mut c_unknowns = unknowns.iter().map(|u| *u + row.len() * j + j ).collect();
			e_unknowns.append(&mut c_unknowns);

			let mut c_groups = groups.clone();
			e_groups.append(&mut c_groups);

			if j < 1 {
				e_row.push('?');
				e_unknowns.push(e_row.len() - 1);
			}
		}

		
		prow(&e_row);
		print!("e_unknowns[{}]: ", e_unknowns.len());
		for u in e_unknowns.iter() { print!("{} ", u); }
		println!("");
		print!("e_groups[{}]: ", e_groups.len());
		for g in e_groups.iter() { print!("{} ", g); }
		println!("");
		

		let once = count(&mut row, &mut unknowns, &groups);
		let rep = count(&mut e_row, &mut e_unknowns, &e_groups);
		let ex = rep / once;
		let p = (0..4).fold(once, |res, _| res * ex);
		println!("once: {}, rep: {}, p: {}", once, rep, p);

		s += p;
		
		println!("---------");

		break;
	}

	println!("s: {}", s);
}

fn count(row: &mut Vec<char>, unknowns: &mut Vec<usize>, groups: &Vec<u32>) -> u32 {

	let mut s = 1;
	s
}

fn prow(row: &Vec<char>) {
	print!("[{}]: ", row.len());
	for c in row.iter() {
		print!("{} ", c);
	}
	println!("");
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

