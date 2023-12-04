use std::fs;

fn main() {
	let lines = read_lines("3.txt");

	let vlines: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();

	let mut sum = 0;

	let mut num = 0;
	let mut num_digits = 0;

	for (i, line) in vlines.iter().enumerate() {
		for (j, c) in line.iter().enumerate() {
			if c.is_digit(10) {
				num = num * 10 + c.to_digit(10).unwrap();
				num_digits += 1
			}
			if num > 0 && (j == line.len() - 1 || !line[j + 1].is_digit(10)) {
				let p = if j >= num_digits { j - num_digits } else { 0 };
				let q = if j < line.len() - 1 { j + 1 } else { line.len() - 1 };
				let r = if i > 0 { i - 1 } else { 0 };
				let s = if i < vlines.len() - 1 { i + 1 } else { line.len() - 1 };
				// println!("{}, [{}, {}, {}, {}]", num, p, q, r, s);
				let mut has_symbol = false;
				'f: for y in r..=s {
					for x in p..=q {
						if vlines[y][x] != '.' && !vlines[y][x].is_digit(10) { has_symbol = true; break 'f; }
					}
				}

				
				if has_symbol {
					// println!("{}", num);
					sum += num;
				}

				num = 0;
				num_digits = 0;
			}
		}
	
	// if i > 4 { break; }

	}

	println!("sum: {}", sum);
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
