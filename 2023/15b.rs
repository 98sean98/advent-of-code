use std::fs;

fn main() {
	let lines = read_lines("15.txt");
	
	let steps: Vec<&str> = lines[0].split(',').collect();
	
	let mut boxes: Boxes = vec![vec![]; 256];

	for step in steps.iter() {
		let mut label: String = String::new();
		let mut op: char = '?';
		let mut box_i: u64 = 0;
		let mut focal: u8 = 0;

		for &v in step.as_bytes() {
			if v == b'-' {
				op = '-';
			} else if v == b'=' {
				op = '=';
			} else if op == '?' {
				box_i = hash(v, box_i);
				label.push(v as char);
			} else {
				focal = (v as char).to_digit(10).unwrap() as u8;
			}
		}

		if op == '-' {
			// println!("box_i: {}, label: {}", box_i, label);
			minus(&mut boxes, box_i as usize, label);

		} else if op == '=' {
			// println!("box_i: {}, label: {}, focal: {}", box_i, label, focal);
			add(&mut boxes, box_i as usize, label, focal);
		}

		// pb(&boxes);
	}

	let mut s: u64 = 0;

	for (i, b) in boxes.iter().enumerate() {
		let a = (i + 1) as u64;
		for (j, lens) in b.iter().enumerate() {
			let b = (j + 1) as u64;
			let c = lens.1 as u64;
			s += a * b * c;
		}
	}

	println!("s: {}", s);
}

type Box = Vec<(String, u8)>;
type Boxes = Vec<Box>;

fn minus(boxes: &mut Boxes, box_i: usize, label: String) {

	let mut lens_i = boxes[box_i].len();

	for (i, lens) in boxes[box_i].iter().enumerate() {
		if lens.0 == label { lens_i = i; break; }
	}
	
	if lens_i < boxes[box_i].len() {
		boxes[box_i].remove(lens_i);
	}
}

fn add(boxes: &mut Boxes, box_i: usize, label: String, focal: u8) {
	let mut lens_i = boxes[box_i].len();
	
	for (i, lens) in boxes[box_i].iter().enumerate() {
		if lens.0 == label { lens_i = i; break; }
	}

	if lens_i < boxes[box_i].len() {
		boxes[box_i][lens_i] = (label, focal);
	} else {
		boxes[box_i].push((label, focal));
	}
}

fn pb(boxes: &Boxes) {
	for (i, b) in boxes.iter().enumerate() {
		if b.len() == 0 { continue; }
		print!("[{}]: ", i);
		for lens in b.iter() {
			print!("[{} {}] ", lens.0, lens.1);
		}
		println!("");
	}
}

fn hash(c: u8, n: u64) -> u64 {
	let p = n + c as u64;
	(p * 17) % 256
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

