use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("19.txt");

	let mut workflows: Workflows = HashMap::new();

	let mut start = false;

	let mut s = 0;

	for l in lines.into_iter() {

		if l.len() == 0 { start = true; continue; }

		if !start {
			let (name, w) = new_workflow(l);
			workflows.insert(name, w);
		} else {
			let p = new_part(l);
			s += process(&workflows, &p);
		}

	}

	println!("w: {}", workflows.len());


	println!("s: {}", s);

}

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

fn new_part(l: String) -> Part {
	let mut s = l.clone();
	s.remove(0); s.pop();
	
	let mut p = HashMap::with_capacity(4);

	for a in s.split(',') {
		let b: Vec<String> = String::from(a)
			.split('=')
			.map(|c| String::from(c))
			.collect();
		let cat = b[0].chars().next().unwrap();
		let val = b[1].chars().fold(0, |r, c| r * 10 + c.to_digit(10).unwrap());
		p.insert(Category::from_char(cat), val);
	}

	p
}

fn process(workflows: &Workflows, part: &Part) -> u32 {
	let init = String::from("in");
	let accepted = test(workflows, init, part);
	if accepted { count(part) } else { 0 }
}

fn test(workflows: &Workflows, name: String, part: &Part) -> bool {
	if name == "A" { true }
	else if name == "R" { false }
	else {
		let w = workflows.get(&name).unwrap();
		let mut dest = String::new();

		for r in w.rules.iter() {
			if let Some(cond) = &r.cond {
				if cond.test(part) {
					dest = r.dest.clone();
					break;
				}
			} else {
				dest = r.dest.clone();
				break;
			}
		}

		test(workflows, dest, part)
	}
}

fn count(p: &Part) -> u32 {
	p.values().fold(0, |r, &v| r + v)	
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
}


fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

