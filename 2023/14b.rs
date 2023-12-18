use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("14.txt");

	let platform: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
	

	let cycles = 1000000000;
	let mut m_platform = platform.clone();
	let mut mem: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
	let (mut a, mut b) = (0, 0);

	for i in 0..cycles {
		if let Some(v) = mem.get(&m_platform) {
			(a, b) = (*v, i);
			break;
		} else {
			mem.insert(m_platform.clone(), i);
			m_platform = north(&m_platform);
			m_platform = west(&m_platform);
			m_platform = south(&m_platform);
			m_platform = east(&m_platform);
		}
	}


	let r = a + (cycles - a) % (b - a);
	println!("a: {}, b: {}, r: {}", a, b, r);

	for (k, v) in mem.iter() {
		if *v == r {
			m_platform = k.clone();
			break;
		}
	}


	let mut s = 0;

	for (i, row) in m_platform.iter().enumerate() {
		s += row.iter().fold(0, |r, &a| if a == 'O' { r + 1 } else { r }) * (m_platform.len() - i);
	}

	println!("s: {}", s);
	
}

fn north(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut lasts: Vec<usize> = vec![0; platform[0].len()];
	let mut m_platform: Vec<Vec<char>> = platform.clone();

	for (i, row) in platform.iter().enumerate() {

		for j in 0..row.len() {
			if row[j] == 'O' {
				m_platform[i][j] = '.';
				m_platform[lasts[j]][j] = 'O';
				lasts[j] += 1;
			} else if row[j] == '#' {
				lasts[j] = i + 1;
			}
		}
	}

	m_platform
}

fn west(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut last = 0;
	let mut m_platform: Vec<Vec<char>> = platform.clone();
	for (i, row) in platform.iter().enumerate() {
		for (j, &c) in row.iter().enumerate() {
			if c == 'O' {
				m_platform[i][j] = '.';
				m_platform[i][last] = 'O';
				last += 1;
			} else if c == '#' {
				last = j + 1;
			}
		}
		last = 0;
	}

	m_platform
}

fn south(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut lasts: Vec<usize> = vec![platform.len() - 1; platform[0].len()];
	let mut m_platform: Vec<Vec<char>> = platform.clone();
	for i in 0..platform.len() {
		let r_i = platform.len() - i - 1;
		let row = &platform[r_i];
		for (j, &c) in row.iter().enumerate() {
			if c == 'O' {
				m_platform[r_i][j] = '.';
				m_platform[lasts[j]][j] = 'O';
				if lasts[j] > 0 { lasts[j] -= 1; }
			} else if c == '#' && r_i > 0 {
				lasts[j] = r_i - 1;
			}
		}
	}
	
	m_platform
}

fn east(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut last = platform[0].len() - 1;
	let mut m_platform: Vec<Vec<char>> = platform.clone();
	for (i, row) in platform.iter().enumerate() {
		for j in 0..row.len() {
			let c_i = row.len() - j - 1;
			let c = row[c_i];
			if c == 'O' {
				m_platform[i][c_i] = '.';
				m_platform[i][last] = 'O';
				if last > 0 { last -= 1; }
			} else if c == '#' && c_i > 0 {
				last = c_i - 1;
			}
		}
		last = platform[0].len() - 1;
	}

	m_platform
}

fn pp(platform: &Vec<Vec<char>>) {
	for r in platform.iter() {
		for c in r.iter() { print!("{}", c); }
		println!("");
	}
	println!("------------------");
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

