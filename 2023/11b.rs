use std::fs;

fn main() {
	let lines = read_lines("11.txt");

	let mut galaxies: Vec<Vec<usize>> = vec![vec![]; lines.len()];
	
	for (i, line) in lines.iter().enumerate() {

		for (j, c) in line.chars().enumerate() {
			if c == '#' { galaxies[i].push(j); }
		}
	}

	let mut e_rows: Vec<usize> = Vec::with_capacity(lines.len());
	let mut e_cols: Vec<usize> = Vec::with_capacity(lines[0].len());

	for (i, row) in galaxies.iter().enumerate() {
		if row.len() == 0 { e_rows.push(i); }
	}
	for j in 0..lines[0].len() {
		if galaxies.iter().all(|row| row.iter().all(|&g| g != j)) { e_cols.push(j); }
	}

	print!("e_rows:");
	for e in e_rows.iter() { print!(" {}", e); }
	println!(""); print!("e_cols:");
	for e in e_cols.iter() { print!(" {}", e); }
	println!("");

	let mut s: u64 = 0;
	let mut count = 0;
	for (i, row) in galaxies.iter().enumerate() {
		for (j, &m) in row.iter().enumerate() {
			let p1 = (i, m);

			for n in j+1..row.len() {
				let p2 = (i, row[n]);
				s += dist(p1, p2, &e_rows, &e_cols);
				// println!("[{}] p1: ({}, {}), p2: ({}, {}), dist: {}", count, p1.0, p1.1, p2.0, p2.1, dist(p1, p2, &e_rows, &e_cols));
				count += 1;
			}

			for m in i+1..galaxies.len() {
				for &g in galaxies[m].iter() {
					let p2 = (m, g);
					s += dist(p1, p2, &e_rows, &e_cols);
					// println!("[{}] p1: ({}, {}), p2: ({}, {}), dist: {}", count, p1.0, p1.1, p2.0, p2.1, dist(p1, p2, &e_rows, &e_cols));
					count += 1;
				}
			}
		}
	}

	println!("s: {}", s);

	
}

fn dist(p1: (usize, usize), p2: (usize, usize), e_rows: &Vec<usize>, e_cols: &Vec<usize>) -> u64 {
	let (min_r, max_r) = if p1.0 <= p2.0 { (p1.0, p2.0) } else { (p2.0, p1.0) };
	let (min_c, max_c) = if p1.1 <= p2.1 { (p1.1, p2.1) } else { (p2.1, p1.1) };
	let r = max_r - min_r;
	let c = max_c - min_c;

	let r_e = e_rows.iter().fold(0, |res, &a| if min_r < a && a < max_r { res + 1000000 - 1 } else { res });
	let c_e = e_cols.iter().fold(0, |res, &a| if min_c < a && a < max_c { res + 1000000 - 1 } else { res });

	(r + c ) as u64 + r_e + c_e
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

