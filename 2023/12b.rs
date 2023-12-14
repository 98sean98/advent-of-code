use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("12.txt");

	let mut s = 0;
	let mut memory: Memory = HashMap::new();

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

		for j in 0..5 {
			let mut c_row = row.clone();
			e_row.append(&mut c_row);

			let mut c_unknowns = unknowns.iter().map(|u| *u + row.len() * j + j ).collect();
			e_unknowns.append(&mut c_unknowns);

			let mut c_groups = groups.clone();
			e_groups.append(&mut c_groups);

			if j < 4 {
				e_row.push('?');
				e_unknowns.push(e_row.len() - 1);
			}
		}

		/*
		let once = count(&mut memory, &mut row, &mut groups, false);
		let rep = 
		let ex = rep / once;
		let p = (0..4).fold(once, |res, _| res * ex);
		println!("once: {}, rep: {}, p: {}", once, rep, p);
		*/

		s += count(&mut memory, &mut e_row, &mut e_groups, false);
		
	}

	println!("s: {}", s);
}

type Memory = HashMap<(Vec<char>, Vec<u32>, bool), u64>;

fn count(memory: &mut Memory, record: &mut Vec<char>, groups: &mut Vec<u32>, ed: bool) -> u64 {
	if let Some(v) = memory.get(&(record.clone(), groups.clone(), ed)) {
		return *v;
	}

	let ans: u64 = match (record.last(), groups.last()) {
	(Some(&r), Some(&g)) => 
		if r == '.' {
			if ed { 0 } else {
			record.pop();
			let g_index = groups.len() - 1;
			if g == 0 { groups.pop(); };
			let d = count(memory, record, groups, false);
			if g == 0 { groups.push(g); }
			record.push(r);
			d
			}
		} else if r == '#' {
			if g == 0 {
				0
			} else {
				record.pop();
				let g_index = groups.len() - 1;
				groups[g_index] = g - 1;
				let h = count(memory, record, groups, g-1 > 0);
				record.push(r); 
				groups[g_index] = g;
				h
			}
		} else {
			record.pop();
			let g_index = groups.len() - 1;
			if g == 0 { groups.pop(); };
			let d = if ed { 0 } else { count(memory, record, groups, false) };
			if g == 0 { groups.push(g); }
			let h = {
				if g == 0 { 0 }
				else {
				groups[g_index] = g - 1;
				let h = count(memory, record, groups, g-1 > 0);
				groups[g_index] = g;
				h
				}
			};
			record.push(r); 
			d + h
		}
	,
	(Some(_), None) => if record.iter().all(|&r| r != '#') { 1 } else { 0 },
	(None, Some(&g)) => if groups.len() == 1 && g == 0 { 1 } else { 0 },
	(None, None) => 1
	};

	// pr(&record, &groups);
	// println!("ans: {}", ans);

	memory.insert((record.clone(), groups.clone(), ed), ans);
	ans

}

fn pr(record: &Vec<char>, groups: &Vec<u32>) {
	for c in record.iter() {
		print!("{}", c);
	}
	print!("  ");
	for g in groups.iter() {
		print!("{},", g);
	}
	println!("");
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

