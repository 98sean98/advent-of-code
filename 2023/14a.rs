use std::fs;

fn main() {
	let lines = read_lines("14.txt");

	let platform: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
	
	let m_platform = north(&platform);

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

fn pp(platform: &Vec<Vec<char>>) {
	for r in platform.iter() {
		for c in r.iter() { print!("{}", c); }
		println!("");
	}
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

