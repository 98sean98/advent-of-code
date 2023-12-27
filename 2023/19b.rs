use std::fs;
use std::collections::HashMap;
use std::iter::zip;

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


	let mut nodes: Vec<Node> = vec![];

	


	let s = nodes.iter().fold(0, |r, n| r + n.count());

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

#[derive(Debug)]
struct Interval {
	min: u32,
	max: u32
}

impl Interval {
	fn from(c: &Condition) -> Self {
		let (min, max) = match c.op {
			'>' => (c.val + 1, MAX),
			_ => (MIN, c.val - 1),
		};
		Self { min: min, max: max }
	}
	fn merge(&self, other: &Interval) -> Self {
		let (a, b) = (self.min, self.max);
		let (c, d) = (other.min, other.max);
		let (a, b, c, d) = if a <= c { (a, b, c, d) } else { (c, d, a, b) };

		if c <= b {
			Interval { min: c, max: b }
		} else {
			Interval { min: 0, max: 0 }
		}
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
	fn list() -> Vec<Self> {
		vec![Category::X, Category::M, Category::A, Category::S]
	}
}

#[derive(Debug)]
struct Node {
	intervals: HashMap<Category, Interval>
}

impl Node {
	fn from(w: &Workflow) -> Self {
		let mut h = HashMap::with_capacity(4);
		for c in category.list().into_iter() {
			h.insert(c, Interval { min: 1, max: 4000 });
		}
		for r in w.rules.iter() {
			if let Some(cond) = r.cond {
				let i = Interval::from(cond);
				// todo
			}
		}
	}
	fn count(&self) -> u64 {
		self.intervals
			.values()
			.fold(1, |r, a| r * if a.min > 0 { (a.max - a.min) as u64 + 1 } else { 0 })
	}
}

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

