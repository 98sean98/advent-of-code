use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn main() {
	let lines = read_lines("17.txt");

	let town: Vec<Vec<u32>> = lines.iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() ).collect()).collect();

	let s = dijkstra(&town, (0, 0), (town.len() - 1, town[0].len() - 1));

	println!("s: {}", s);

}

fn dijkstra(town: &Vec<Vec<u32>>, start: Position, end: Position) -> u32 {

	let mut dist: Vec<Vec<Vec<Vec<u32>>>> = vec![
		vec![
			vec![
				vec![u32::MAX; 10]; 4
			]; town[0].len()
		]; town.len()];
	let mut heap = BinaryHeap::new();

	dist[start.0][start.1] = vec![vec![0; 10]; 4];
	heap.push(State { cost: 0, pos: start, dir: 0, step: 0 });
	
	while let Some(c) = heap.pop() {
		
		if c.step > 0 && c.cost > dist[c.pos.0][c.pos.1][c.dir][c.step - 1] { continue; }

		let ns = find(town, c);
		
		for n in ns.into_iter() {
			if n.cost < dist[n.pos.0][n.pos.1][n.dir][n.step - 1] {
				heap.push(n);
				dist[n.pos.0][n.pos.1][n.dir][n.step - 1] = n.cost;
			}
		}
	}

	*dist[end.0][end.1].iter().flatten().min().unwrap()

}

fn find(town: &Vec<Vec<u32>>, c: State) -> Vec<State> {

	let (rows, cols) = (town.len(), town[0].len());
	let mut v = vec![];

	if c.dir != 2 && c.pos.1 < cols - 1 { v.push((c.pos.0, c.pos.1 + 1, 0)); }
	if c.dir != 3 && c.pos.0 < rows - 1 { v.push((c.pos.0 + 1, c.pos.1, 1)); }
	if c.dir != 0 && c.pos.1 > 0 { v.push((c.pos.0, c.pos.1 - 1, 2)); }
	if c.dir != 1 && c.pos.0 > 0 { v.push((c.pos.0 - 1, c.pos.1, 3)); }
		
	v.into_iter()
		.filter(|n| {
			if n.2 == c.dir { c.step < 10 }
			else { c.step >= 4 }
		})
		.map(|n| State { cost: c.cost + town[n.0][n.1], pos: (n.0, n.1), dir: n.2, step: if c.dir == n.2 { c.step + 1 } else { 1 } })
		.collect()

}

fn pns(c: &State, ns: &Vec<State>) {
	print!("[({}, {}): {}, ({}, {})] – ", c.pos.0, c.pos.1, c.cost, c.dir, c.step);
	for &State { cost, pos, dir, step } in ns.iter() {
		print!("[({}, {}): {}, ({} {})] ", pos.0, pos.1, cost, dir, step);
	}
	println!("");
}

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
	cost: u32,
	pos: Position,
	dir: usize,
	step: usize,
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

