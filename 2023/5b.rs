use std::fs;

fn main() {
	let lines = read_lines("5.txt");

	let seeds = read_seeds(lines[0].clone());

	let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![vec![]];

	let mut map_id = 0;
	let mut num: u64 = 0;
	for i in 2..lines.len() {
		let l: Vec<char> = lines[i].chars().collect();

		if l.len() == 0 { 
			map_id += 1; 
			maps.push(vec![]); 
			continue;
		};

		if !l[0].is_digit(10) { continue; }

		let mut r: (u64, u64, u64) = (0, 0, 0);
		let mut c = 0;
		for e in l.iter() {
			if *e != ' ' { num = num * 10 + e.to_digit(10).unwrap() as u64; }
			else { if c == 0 {r.0 = num; c += 1;} else {r.1 = num;} num = 0; }
		}
		r.2 = num; num = 0;
		
		maps[map_id].push(r);
	}

	let mut lowest = u64::MAX;
	for (i, s) in seeds.iter().enumerate() {
		if i % 2 == 1 {
			let (start, range) = (seeds[i-1], s);
			println!("start: {}, range: {}", start, range);
			for seed in start..start+range {
		let mut n = seed;
		for map in maps.iter() {
			for r in map.iter() {
				if r.1 <= n && n < r.1 + r.2 {
					n = r.0 + n - r.1;
					break;
				}
			}
		}
		
		if lowest > n { lowest = n; println!("updated lowest: {}", lowest); }

		}}
		
	}

	println!("lowest: {}", lowest);

}

fn read_seeds(line: String) -> Vec<u64> {
	let mut s: Vec<u64> = vec![];

	let l: Vec<char> = line.chars().collect();
	let mut num: u64 = 0;
	for i in 7..line.len() {
		if l[i].is_digit(10) {
			num = num * 10 + l[i].to_digit(10).unwrap() as u64;
		} else {
			s.push(num);
			num = 0;
		}
	}
	s.push(num);
	s
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
