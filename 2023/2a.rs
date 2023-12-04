use std::fs;

fn main() {
	let lines = read_lines("2.txt");

	let mut sum = 0;

	'line: for (i, line) in lines.iter().enumerate() {
		let l: Vec<char> = line.chars().collect();

		let mut start = false;
		let mut expect_color = false;
		let mut num = 0;
		let mut j = 6;
		while j < line.len() {
			if l[j] == ':' {
				start = true;
				j += 1;
				continue;
			}
			if !start { 
			} else if l[j].is_digit(10) {
				num = num * 10 + l[j].to_digit(10).unwrap();
			} else if num > 0 && l[j] == ' ' {
				expect_color = true;
			} else if expect_color {
				if l[j] == 'r' {
					if num > 12 {
				 		continue 'line; }
					j += 2;
				} else if l[j] == 'g' {
					if num > 13 {
					 	continue 'line; }
					j += 4;
				} else {
					if num > 14 { 
						continue 'line; }
					j += 3;
				}
				num = 0;
				expect_color = false;
			}
			j += 1;
		}
		sum += i + 1;

	}

	println!("sum: {}", sum);
	
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
