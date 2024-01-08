use std::fs;

fn main() {
	let lines = read_lines("21.txt");

	let steps = 64;

	let mut map: Map = lines.into_iter()
		.map(|l| l.chars().collect())
		.collect();

	let mut start: Position = (0, 0);
	for (i, row) in map.iter().enumerate() {
		for (j, c) in row.iter().enumerate() {
			if *c == 'S' { start = (i, j) }
		}
	}

	let mut visited: Visited = vec![vec![vec![]; map[0].len()]; map.len()];
	
	walk(&mut map, &mut visited, start, steps);
	pm(&map);

	let s = map.iter()
		.fold(0, |a, l| a + l.iter().fold(0, |b, c| b + if *c == 'O' { 1 } else { 0 }));

	println!("s: {}", s);

}

type Map = Vec<Vec<char>>;
type Visited = Vec<Vec<Vec<usize>>>;
type Position = (usize, usize);

fn walk(map: &mut Map, visited: &mut Visited, position: Position, steps: usize) {
	if visited[position.0][position.1].iter().find(|p| **p == steps) != None {
		return;
	}

	visited[position.0][position.1].push(steps);

	if steps == 0 {
		map[position.0][position.1] = 'O';
		return;
	}

	let ns = neighbors(map, position);

	for n in ns.into_iter() {
		walk(map, visited, n, steps - 1);
	}
}

fn neighbors(map: &Map, (i, j): Position)  -> Vec<Position> {
	let (rows, cols) = (map.len(), map[0].len());

	let mut v = vec![];
	
	if i > 0 {
		if map[i-1][j] != '#' { v.push((i-1, j)); }
	}
	if j > 0 {
		if map[i][j-1] != '#' { v.push((i, j-1)); }
	}
	if i < rows - 1 {
		if map[i+1][j] != '#' { v.push((i+1, j)); }
	}
	if j < cols - 1 {
		if map[i][j+1] != '#' { v.push((i, j+1)); }
	}
	
	v
}

fn pm(map: &Map) {
	for row in map.iter() {
		for c in row.iter() {
			print!("{}", c);
		}
		println!("");
	}
	println!("");
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

