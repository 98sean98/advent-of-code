use std::fs;

fn main() {
	let lines = read_lines("9.txt");

	let mut s: i32 = 0;

	for line in lines.iter() {
		let l: Vec<char> = line.chars().collect();
		let (mut num, mut sign): (i32, i32) = (0, 1);
		let mut nums: Vec<i32> = vec![];
		for (j, c) in l.iter().enumerate() {
			if c.is_digit(10) {
				num = num * 10 + sign * c.to_digit(10).unwrap() as i32;
			} else if *c == '-' {
				sign = -1;
			}
			if j == l.len() - 1 || l[j+1] == ' ' {
				nums.push(num);
				num = 0; sign = 1;
			}
		}

		let mut diffs: Vec<Vec<i32>> = vec![nums];

		let mut diff: &Vec<i32> = diffs.last().unwrap();

		while !diff.iter().all(|&x| x == 0) {

			let mut new_diff: Vec<i32> = Vec::with_capacity(diff.len() - 1);
			
			for k in 1..diff.len() {
				let n = diff[k] - diff[k - 1];
				new_diff.push(n);
			}

			diffs.push(new_diff);
			diff = diffs.last().unwrap();
		}

		let mut ex: Vec<i32> = Vec::with_capacity(diffs.len());
		for j in 0..diffs.len() {
			let diff = &diffs[diffs.len() - j - 1];
			if j == 0 {
				ex.push(0);
			} else {
				ex.push(diff.first().unwrap() - ex.last().unwrap());
			}
		}

		s += ex.last().unwrap();

	}


	println!("s: {}", s);
	
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
