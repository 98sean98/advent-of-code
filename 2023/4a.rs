use std::fs;

fn main() {
	let lines = read_lines("4.txt");

	let mut sum = 0;
	
	for line in lines.iter() {
		let l: Vec<char> = line.chars().collect();

		let mut w: Vec<u32> = vec![];
		let mut num = 0;
		let mut winning = true;
		let mut p = 0;

		for j in 10..line.len() {
			if l[j].is_digit(10) {
				num = num * 10 + l[j].to_digit(10).unwrap();
			} else if l[j] == '|' {
				winning = false;
			}
			
			if num > 0 && (j == line.len() - 1 || l[j+1] == ' ') {
				if winning { w.push(num); }
				else {
					// println!("num: {}", num);
					for e in w.iter() {
						if *e == num {
							p = if p == 0 { 1 } else { p * 2};
						}
						// println!("e: {}, p: {}", e, p);
					}
				}
				num = 0;
			}
		}

		sum += p;

	}

	println!("sum: {}", sum);
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
