use std::fs;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("19.txt");

	let mut workflows: Workflows = HashMap::new();

	for l in lines.into_iter() {

		if l.len() == 0 { break; }

		let (name, w) = new_workflow(l);
		// println!("[{}] {:?}", name, w);
		workflows.insert(name, w);

	}

	println!("w: {}", workflows.len());


	let mut accepted: Vec<Node> = vec![];

	for (name, w) in workflows.iter() {
		// println!("[{}]", name);
		for n in w.nodes.iter() {
			if n.dest == String::from("A") {
				let a = check(&workflows, name.clone(), n);
				accepted.push(a);
			}
		}
		// println!("----------------");
	}


	let s = accepted.iter().fold(0, |r, n| r + n.count());

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
	
	let nodes = Node::from(&rules);

	let w = Workflow { nodes: nodes };

	(s[0].clone(), w)
}

fn check(workflows: &Workflows, name: String, n: &Node) -> Node {
	let mut n = n.clone();

	let p = workflows.get(&name).unwrap();
	let a = &p.nodes[n.index];
	n.merge(&a);
	// println!("child: {:?}", n);

	if name != String::from("in") {
		let mut gp_name = String::new();
		for (gp, w) in workflows.iter() {
			if let Some(p) = w.nodes.iter().find(|c| c.dest == name) {
			n.index = p.index;
			gp_name = gp.clone();
			break;
			}
		}
		n.dest = name.clone();
		// println!("gp: {}, parent: {:?}", gp_name, n);
		n = check(workflows, gp_name, &n);
	}

	n
}

#[derive(Debug, Clone, Copy)]
struct Interval {
	min: u32,
	max: u32
}

impl Interval {
	fn default() -> Self {
		Self { min: MIN, max: MAX }
	}
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
		let (_a, b, c, d) = if a <= c { (a, b, c, d) } else { (c, d, a, b) };

		if c <= b {
			Self { min: c, max: if d <= b { d } else { b } }
		} else {
			Self { min: 0, max: 0 }
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
}

#[derive(Debug, Clone)]
struct Node {
	intervals: HashMap<Category, Interval>,
	dest: String,
	index: usize
}

impl Node {
	fn from(rules: &Vec<Rule>) -> Vec<Self> {
		let mut conds: Vec<Condition> = vec![];
		let mut nodes: Vec<Node> = vec![];
		for (index, rule) in rules.iter().enumerate() {
			if let Some(c) = conds.pop() {
				conds.push(c.opposite());
			}
			if let Some(c) = rule.cond {
				conds.push(c);
			} 
			let mut intervals: HashMap<Category, Interval> = HashMap::from([
				(Category::X, Interval::default()),
				(Category::M, Interval::default()),
				(Category::A, Interval::default()),
				(Category::S, Interval::default())]);
			for c in conds.iter() {
				let i = Interval::from(c);
				intervals
					.entry(c.cat)
					.and_modify(|a| *a = a.merge(&i));
			}
			nodes.push(Node { intervals: intervals, dest: rule.dest.clone(), index: index });
		}
		nodes
	}
	fn merge(&mut self, other: &Self) {
		for (c, i) in self.intervals.iter_mut() {
			*i = i.merge(other.intervals.get(&c).unwrap());
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
	nodes: Vec<Node>
}


#[derive(Debug)]
struct Rule {
	cond: Option<Condition>,
	dest: String,
}

#[derive(Debug, Clone, Copy)]
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

