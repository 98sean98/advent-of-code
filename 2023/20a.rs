use std::fs;
use std::any::Any;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("20b.txt");
	let lines = lines.into_iter()
			.map(|l| {
				let mut s = l.split(" -> ").map(String::from);
				let a = s.next().unwrap();
				let b = s.next().unwrap();

				let next: Next = b.split(", ").map(String::from).collect();
				(a, next)
			})
			.collect::<Vec<(String, Next)>>();

	let mut modules: Modules = HashMap::new();

	for (a, next) in lines.iter() {
		let (n, m): (String, Module) = if a == "broadcaster" {
			(String::from("broadcaster"), Box::new(<Broadcast as Modular>::from(next.clone())))
		} else {
			let mut name = a.clone();
			let c = name.remove(0);
			(name, match c {
				'%' => Box::new(<FlipFlop as Modular>::from(next.clone())),
				_ => Box::new(<Conjunction as Modular>::from(next.clone()))
			})
		};
		modules.insert(n, m);
	}


	for (a, next) in lines.into_iter() {
		// todo add mem hashmap for each Conjunction
		let name = if a == "broadcaster" {
			String::from("broadcaster")
		} else {
			let mut n = a.clone();
			n.remove(0);
			n
		};
		for b in next.into_iter() {
			if let Some(m) = modules.get_mut(&b) {
			if let Some(c) = m.as_any_mut().downcast_mut::<Conjunction>() {
				c.insert_mem(name.clone());
			}
			}
		}
	}

	for (n, m) in modules.iter() {
		println!("[{}] {:?}", n, m);
	}


	let mut pulses: Pulses = VecDeque::new();
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse { Low, High }

// source, destination, pulse
type Pulsing = (String, String, Pulse);
type Pulses = VecDeque<Vec<Pulsing>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State { On, Off }

trait Modular: std::fmt::Debug {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn from(next: Next) -> Self where Self: Sized;
	fn dest(&self) -> &Next;
	fn run(&mut self, source: String, pulse: Pulse) -> Option<Pulse>;
}

type Module = Box<dyn Modular>;
type Modules = HashMap<String, Module>;
type Next = Vec<String>;

fn update(modules: &mut Modules, (source, dest, pulse): Pulsing) -> Vec<Pulsing> {
	let m = modules.get_mut(&dest).unwrap();
	if let Some(p) = m.run(source, pulse) {	
		let next = m.dest().clone();
		next.into_iter().map(|d| (dest.clone(), d, p)).collect()
	} else {
		vec![]
	}
}

#[derive(Debug, Clone)]
struct FlipFlop {
	next: Next,
	state: State
}


impl Modular for FlipFlop {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn from(next: Next) -> Self {
		Self { next: next, state: State::Off }
	}
	fn dest(&self) -> &Next {
		&self.next
	}
	fn run(&mut self, _source: String, pulse: Pulse) -> Option<Pulse> {
		match pulse {
			Pulse::Low => {
				Some(match self.state {
					State::Off => {
						self.state = State::On;
						Pulse::High },
					State::On => {
						self.state = State::Off;
						Pulse::Low
					}
				})
			},
			_ => { None }
		}
	}
}

#[derive(Debug, Clone)]
struct Conjunction {
	next: Next,
	mem: HashMap<String, Pulse>,
}

impl Modular for Conjunction {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn from(next: Next) -> Self {
		Self { next: next, mem: HashMap::new() }
	}
	fn dest(&self) -> &Next {
		&self.next
	}
	fn run(&mut self, source: String, pulse: Pulse) -> Option<Pulse> {
		self.mem.entry(source).and_modify(|p| *p = pulse);
		Some(if self.mem.values().all(|p| *p == Pulse::High) { Pulse::Low } else { Pulse::High })
	}
}

impl Conjunction {
	fn insert_mem(&mut self, a: String) {
		self.mem.insert(a, Pulse::Low);
	}
}

#[derive(Debug, Clone)]
struct Broadcast {
	next: Next
}

impl Modular for Broadcast {
	fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }
	fn from(next: Next) -> Self {
		Self { next: next }
	}
	fn dest(&self) -> &Next {
		&self.next
	}
	fn run(&mut self, _source: String, pulse: Pulse) -> Option<Pulse> {
		Some(pulse)
	}
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

