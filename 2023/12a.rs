use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("12a.txt");

	let mut s = 0;

	for line in lines.iter() {
		let l: Vec<char> = line.chars().collect();

		let mut record: Vec<char> = vec![];
		let mut groups: Vec<u32> = vec![];
		let mut space = false;

		let mut num = 0;

		for (j, c) in l.iter().enumerate() {
			if *c == ' ' {
				space = true;
			} else if !space {
				record.push(*c);
			} else if c.is_digit(10) {
				num = num * 10 + c.to_digit(10).unwrap();
			}
			if j == l.len() - 1 || l[j+1] == ',' {
				groups.push(num);
				num = 0;
			}
		}


		let mut memory: Memory = HashMap::new();
		s += count(&mut memory, &mut record, &mut groups);
		
		println!("------------");
	}


	println!("s: {}", s);
}

type Memory = HashMap<(Vec<char>, Vec<u32>), u32>;

fn count(memory: &mut Memory, record: &mut Vec<char>, groups: &mut Vec<u32>) -> u32 {
	if let Some(v) = memory.get(&(record.clone(), groups.clone())) {
		return *v;
	}

	let ans: u32 = match (record.last(), groups.last()) {
	(Some(&r), Some(&g)) => 
		if r == '.' {
			record.pop();
			let g_index = groups.len() - 1;
			if g == 0 { groups.pop(); };
			let d = count(memory, record, groups);
			if g == 0 { groups.push(g); }
			record.push(r);
			d
		} else if r == '#' {
			if g == 0 {
				0
			} else {
				record.pop();
				let g_index = groups.len() - 1;
				groups[g_index] = g - 1;
				let h = count(memory, record, groups);
				record.push(r); 
				groups[g_index] = g;
				h
			}
		} else {
			record.pop();
			let g_index = groups.len() - 1;
			if g == 0 { groups.pop(); };
			let d = count(memory, record, groups);
			if g == 0 { groups.push(g); }
			let h = {
				if g == 0 { 0 }
				else {
				groups[g_index] = g - 1;
				let h = count(memory, record, groups);
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

	pr(&record, &groups);
	println!("ans: {}", ans);

	memory.insert((record.clone(), groups.clone()), ans);
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

