use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("16.txt");
	
	let cave: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();


	let mut s = 0;
	for j in 0..cave[0].len() {
		let f = find(&cave, (0, j, 1));
		if s < f { s = f; }
		let f = find(&cave, (cave.len() - 1, j, 3));
		if s < f { s = f; }
	}
	for i in 0..cave.len() {
		let f = find(&cave, (i, 0, 0));
		if s < f { s = f; }
		let f = find(&cave, (i, cave[0].len() - 1, 2));
		if s < f { s = f; }
	}
	
	println!("s: {}", s);

}

type Memory = Vec<Vec<(bool, bool, bool, bool)>>;
type Light = (usize, usize, u8);
type New = Vec<Light>;

fn find(cave: &Vec<Vec<char>>, init: Light) -> u32 {
	let mut mem: Memory = vec![];
	for row in cave.iter() {
		mem.push(vec![(false, false, false, false); row.len()]);
	}

	let mut stack: Vec<Light> = vec![init];
	
	let m = if init.2 == 0 { (true, false, false, false) }
		else if init.2 == 1 { (false, true, false, false) }
		else if init.2 == 2 { (false, false, true, false) }
		else { (false, false, false, true) };
	mem[init.0][init.1] = m;

	while let Some(c) = stack.pop() {
		let tile = cave[c.0][c.1];

		let new: New = if tile == '.' {
			dot(&mem, c)
		} else if tile == '-' {
			dash(&mem, c)
		} else if tile == '|' {
			pipe(&mem, c)
		} else if tile == '\\' {
			backslash(&mem, c)
		} else {
		// if tile == '/' 
			slash(&mem, c)
		};

		// pnew(&new);

		for &(ni, nj, nd) in new.iter() {
			let x = mem[ni][nj];
			let y = if nd == 0 {
				(true, x.1, x.2, x.3)
			} else if nd == 1 {
				(x.0, true, x.2, x.3)
			} else if nd == 2 {
				(x.0, x.1, true, x.3)
			} else {
				(x.0, x.1, x.2, true)
			};
			if y != x {
				mem[ni][nj] = y;
				stack.push((ni, nj, nd));
			}

		}

	}

	mem.iter().fold(0, |a, v| a + v.iter().fold(0, |b, w| if vec![w.0, w.1, w.2, w.3].iter().any(|&x| x) {b+1} else {b}))
}

fn stream(rows: usize, cols: usize, (i, j, d): Light) -> Option<Light> {
	if d == 0 {
		if j + 1 < cols { Some((i, j + 1, d)) }
		else { None }
	} else if d == 1 {
		if i + 1 < rows { Some((i + 1, j, d)) }
		else { None }
	} else if d == 2 {
		if j > 0 { Some((i, j - 1, d)) }
		else { None }
	} else {
		if i > 0 { Some((i - 1, j, d)) }
		else { None }
	}
}

fn dot(mem: &Memory, (i, j, d): Light) -> New {
	if let Some(n) = stream(mem.len(), mem[0].len(), (i, j, d)) {
		vec![n]
	} else {
		vec![]
	}
}

fn dash(mem: &Memory, (i, j, d): Light) -> New {
	let (rows, cols) = (mem.len(), mem[0].len());
	let mut v = vec![];
	if d % 2 == 1 {
		if let Some(n) = stream(rows, cols, (i, j, 0)) { v.push(n); }
		if let Some(n) = stream(rows, cols, (i, j, 2)) { v.push(n); }
	} else {
		v = dot(mem, (i, j, d));
	}
	v
}

fn pipe(mem: &Memory, (i, j, d): Light) -> New {
	let (rows, cols) = (mem.len(), mem[0].len());
	let mut v = vec![];
	if d % 2 == 0 {
		if let Some(n) = stream(rows, cols, (i, j, 1)) { v.push(n); }
		if let Some(n) = stream(rows, cols, (i, j, 3)) { v.push(n); }
	} else {
		v = dot(mem, (i, j, d));
	}
	v
}

fn backslash(mem: &Memory, (i, j, d): Light) -> New {
	let (rows, cols) = (mem.len(), mem[0].len());
	let h = HashMap::from([(0, 1), (1, 0), (2, 3), (3, 2)]);
	let nd = h.get(&d).unwrap();
	if let Some(n) = stream(rows, cols, (i, j, *nd)) {
		vec![n]
	} else {
		vec![]
	}
}

fn slash(mem: &Memory, (i, j, d): Light) -> New {
	let (rows, cols) = (mem.len(), mem[0].len());
	let h = HashMap::from([(0, 3), (1, 2), (2, 1), (3, 0)]);
	let nd = h.get(&d).unwrap();
	if let Some(n) = stream(rows, cols, (i, j, *nd)) {
		vec![n]
	} else {
		vec![]
	}
}

fn pnew(new: &New) {
	for &(i, j, d) in new.iter() {
		print!("({}, {}) {} ", i, j, d);
	}
	println!("");
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

