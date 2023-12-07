use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
	let lines = read_lines("7.txt");

	let mut hands: Vec<Vec<char>> = vec![];
	let mut bids: Vec<u32> = vec![];

	let mut s = 0;

	let mut num = 0;
	for line in lines.iter() {
		let l: Vec<char> = line.chars().collect();
		let hand = l[0..5].to_vec();
		
		for j in 6..l.len() {
			num = num * 10 + l[j].to_digit(10).unwrap();
		}
		
		hands.push(hand);
		bids.push(num);
		num = 0;
	}

	let mut cards: HashMap<char, u8> = HashMap::with_capacity(13);

let cards_list: [(char, u8); 13] = [
	('A', 12),
	('K', 11),
	('Q', 10),
	('T', 9),
	('9', 8),
	('8', 7),
	('7', 6),
	('6', 5),
	('5', 4),
	('4', 3),
	('3', 2),
	('2', 1),
	('J', 0)
];

	for (k, v) in cards_list {
		cards.insert(k, v);
	}


	let order = sort(&hands, 0, lines.len() as u32 - 1, &cards);
	
	for (i, o) in order.iter().enumerate() {s += (i + 1) as u32 * bids[*o as usize]; }



	println!("s: {}", s);
}

fn sort(hands: &Vec<Vec<char>>, start: u32, end: u32, cards: &HashMap<char, u8>) -> VecDeque<u32> {
	if start == end { let mut v: VecDeque<u32> = VecDeque::with_capacity(1); v.push_back(start); return v; }

	let mid: u32 = (start + end) / 2;
	// println!("mid: {}", mid);
	
	let mut left = sort(hands, start, mid, cards);
	let mut right = sort(hands, mid + 1, end, cards);
	// println!("left[0]: {}, right[0]: {}", left[0], right[0]);

	let mut v: VecDeque<u32> = VecDeque::with_capacity(left.len() + right.len());

	while left.len() > 0 || right.len() > 0 {
		if left.len() == 0 {
			while let Some(r) = right.pop_front() { v.push_back(r); }
		} else if right.len() == 0 {
			while let Some(l) = left.pop_front() { v.push_back(l); }
		} else {
			let (l, r) = (left[0], right[0]);
			let (h1, h2) = (&hands[l as usize], &hands[r as usize]);
			if compare(h1, h2, cards) {
				let l = left.pop_front().unwrap();
				v.push_back(l);
			} else {
				let r = right.pop_front().unwrap();
				v.push_back(r);
			}
		}
	}

	// for i in 0..v.len() { println!("v[{}]: {}", i, v[i]); }
	
	v
}

fn compare(h1: &Vec<char>, h2: &Vec<char>, cards: &HashMap<char, u8>) -> bool {
	let (t1, t2) = (find_type(h1), find_type(h2));
	if t1 > t2 { return false; }
	else if t1 < t2 { return true; }
	

	for i in 0..5 {
		let (c1, c2) = (cards.get(&h1[i]).unwrap(), cards.get(&h2[i]).unwrap());
		// println!("c1: {}, c2: {}", c1, c2);
		if c1 != c2 { return c1 < c2; }
	}
	panic!("no tiebreaker");
}

fn find_type(h: &Vec<char>) -> u32 {
	let mut m: HashMap<char, u8> = HashMap::with_capacity(5);
	let mut jokers: u8 = 0;
	
	for c in h.iter() {
		if *c != 'J' {
			m.entry(*c).and_modify(|a| *a += 1).or_insert(1);
		} else {
			jokers += 1;
		}
	}

	let mut v: Vec<u8> = m.values().map(|e| *e).collect();
	v.sort();
	// for i in 0..v.len() { println!("v[{}]: {}", i, v[i]); }

	if v.len() == 1 || jokers == 5 { return 6; };
	if v.len() == 2 {
		if v[v.len() - 1] + jokers == 4 { return 5; }
		return 4;
	}
	if v.len() == 3 {
		if v[v.len() - 1] + jokers == 3 { return 3; }
		return 2;
	}
	if v.len() == 4 { return 1 };

	0
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}
