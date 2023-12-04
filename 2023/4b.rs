use std::fs;

fn main() {
	let lines = read_lines("4.txt");

	let mut cards: Vec<u32> = vec![1; lines.len()];

	
	for (i, line) in lines.iter().enumerate() {
		let l: Vec<char> = line.chars().collect();

		let mut w: Vec<u32> = vec![];
		let mut num = 0;
		let mut winning = true;
		let mut p = i + 1;

		for j in 10..line.len() {
			if l[j].is_digit(10) {
				num = num * 10 + l[j].to_digit(10).unwrap();
			} else if l[j] == '|' {
				winning = false;
			}
			
			if num > 0 && (j == line.len() - 1 || l[j+1] == ' ') {
				if winning { w.push(num); }
				else {
					for e in w.iter() {
						if *e == num {
							cards[p] += cards[i];
							p += 1;
						}
					}
				}
				num = 0;
			}
		}

		for q in i..p {
			println!("q: {}, cards[q]: {}", q, cards[q]);
		}
		

	}

	let sum = cards.into_iter().fold(0, |res, a| res + a);

	println!("sum: {}", sum);
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
