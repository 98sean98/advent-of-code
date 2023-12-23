use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("19.txt");

	let mut workflows: Workflows = HashMap::new();

	let mut start = false;

	for l in lines.into_iter() {

		if l.len() == 0 { break; }

		let (name, w) = new_workflow(l);
		workflows.insert(name, w);

	}

	println!("w: {}", workflows.len());


	

	let mut s = 0;
	println!("s: {}", s);

}

const MIN: u32 = 1;
const MAX: u32 = 4000;

fn new_workflow(l: String) -> (String, Workflow) {
	let mut m = l.clone();
	m.pop();
	let s: Vec<String> = m.split('{').map(|a| String::from(a)).collect();

	let rules: Vec<Rule> = s[1]
		.split(',')
		.map(|a| a.split(':').map(|b| String::from(b)).collect::<Vec<String>>())
		.map(|a|
			if a.len() == 2 {
				let mut it = a[0].chars();
				let cat = it.next().unwrap();
				let op = it.next().unwrap();
				let val = it.fold(0, |r, c| r * 10 + c.to_digit(10).unwrap());

				let cond = Condition { cat: Category::from_char(cat), op: op, val: val };
				Rule { cond: Some(cond), dest: a[1].clone() }
			} else {
				Rule { cond: None, dest: a[0].clone() }
			}
		)
		.collect();
	
	let w = Workflow { rules: rules };

	(s[0].clone(), w)
}

struct Interval {
	min: u32,
	max: u32
}

impl Interval {
	fn from_cond(c: &Condition) -> Self {
		let (min, max) = match c.op {
			'>' => (c.val + 1, MAX),
			_ => (MIN, c.val - 1),
		};
		Self { min: min, max: max }
	}
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Category { X, M, A, S }

impl Category {
	fn from_char(c: char) -> Self {
		match c {
			'x' => Self::X,
			'm' => Self::M,
			'a' => Self::A,
			's' => Self::S,
			_   => panic!("unsupported category")
		}
	}
}

type Part = HashMap<Category, u32>;

type Workflows = HashMap<String, Workflow>;

#[derive(Debug)]
struct Workflow {
	rules: Vec<Rule>
}


#[derive(Debug)]
struct Rule {
	cond: Option<Condition>,
	dest: String,
}

#[derive(Debug)]
struct Condition {
	cat: Category,
	op: char,
	val: u32
}

impl Condition {
	fn test(&self, p: &Part) -> bool {
		let t = *p.get(&self.cat).unwrap();
		let v = self.val;
		match self.op {
			'>' => t > v,
			_   => t < v
		}
	}
	fn opposite(&self) -> Self {
		let (op, val) = match self.op {
			'>' => ('<', self.val + 1),
			_ => ('>', self.val - 1)
		};
		Self { cat: self.cat, op: op, val: val }
	}
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

