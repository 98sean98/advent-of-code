use std::fs;

fn main() {
	let lines = read_lines("18.txt");

	let instructions: Vec<(char, usize, Color)> = lines
		.iter()
		.map(|l| l.split(' ').collect())
		.map(|l: Vec<&str>| (l[0].chars().next().unwrap(), l[1].chars().collect::<Vec<char>>(), l[2].chars().collect::<Vec<char>>()))
		.map(|l| 
			(l.0, l.1.into_iter().fold(0, |r, a| r * 10 + a.to_digit(10).unwrap() as usize), l.2[2..8].to_vec())
		)
		.collect();
	
	let mut c: Pos = (0, 0);

	let mut area: Area = vec![vec![true]];

	for (dir, num, _) in instructions.into_iter() {

		c = if dir == 'U' {
			up(&mut area, c, num)
		} else if dir == 'D' {
			down(&mut area, c, num)
		} else if dir == 'L' {
			left(&mut area, c, num)
		} else {
		// dir == 'R'
			right(&mut area, c, num)
		};

	}

	// pa(&area);

	let mut stack: Vec<Pos> = vec![];
	
	for j in 0..area[0].len() {
		if !area[0][j] { stack.push((0, j)); }
		if !area[area.len()-1][j] { stack.push((area.len()-1, j)); }
	}
	for i in 1..area.len()-1 {
		if !area[i][0] { stack.push((i, 0)); }
		if !area[i][area[i].len()-1] { stack.push((i, area[i].len() - 1)); }
	}

	let mut flood: Area = vec![vec![false; area[0].len()]; area.len()];
	
	while let Some(c) = stack.pop() {
		let ns = find(&flood, c);
		
		for n in ns.into_iter() {
			if !flood[n.0][n.1] && !area[n.0][n.1] {
				flood[n.0][n.1] = true;
				stack.push(n);
			}
		}
	}

	// pa(&flood);	

	let s = flood.iter().flatten().fold(0, |r, &a| r + if !a { 1 } else { 0 });

	println!("s: {}", s);
}

type Area = Vec<Vec<bool>>;
type Color = Vec<char>;
type Pos = (usize, usize);

fn up(area: &mut Area, c: Pos, num: usize) -> Pos {
	if c.0 < num {
		for i in 0..c.0 {
			area[i][c.1] = true;
		}
		for _ in 0..(num - c.0) {
			let mut v = vec![false; area[0].len()];
			v[c.1] = true;
			area.insert(0, v);
		}
		(0, c.1)
	} else {
		for i in c.0-num..c.0 {
			area[i][c.1] = true;
		}
		(c.0 - num, c.1)
	}
}

fn down(area: &mut Area, c: Pos, num: usize) -> Pos {
	if c.0 + num >= area.len() {
		for i in c.0+1..area.len() {
			area[i][c.1] = true;
		}
		for _ in 0..(c.0 + num + 1 - area.len()) {
			let mut v = vec![false; area[0].len()];
			v[c.1] = true;
			area.push(v);
		}
		(area.len() - 1, c.1)
	} else {
		for i in c.0..=c.0+num {
			area[i][c.1] = true;
		}
		(c.0 + num, c.1)
	}
}

fn left(area: &mut Area, c: Pos, num: usize) -> Pos {
	if c.1 < num {
		for j in 0..c.1 {
			area[c.0][j] = true;
		}
		for (i, r) in area.iter_mut().enumerate() {
			let v = vec![i == c.0; num - c.1];
			for e in v.into_iter() {
				r.insert(0, e);
			}
		}
		(c.0, 0)
	} else {
		for j in c.1-num..c.1 {
			area[c.0][j] = true;
		}
		(c.0, c.1 - num)
	}
}

fn right(area: &mut Area, c: Pos, num: usize) -> Pos {
	if c.1 + num >= area[0].len() {
		for j in c.1+1..area[0].len() {
			area[c.0][j] = true;
		}
		let ext = c.1 + num + 1 - area[0].len();
		for (i, r) in area.iter_mut().enumerate() {
			let mut v = vec![i == c.0; ext];
			r.append(&mut v);
		}
		(c.0, area[0].len() - 1)
	} else {
		for j in c.1+1..=c.1+num {
			area[c.0][j] = true;
		}
		(c.0, c.1 + num)
	}
}

fn find(flood: &Area, (i, j): Pos) -> Vec<Pos> {
	let (rows, cols) = (flood.len(), flood[0].len());
	let mut v = vec![];

	if j + 1 < cols { v.push((i, j + 1)); }
	if i + 1 < rows { v.push((i + 1, j)); }
	if j > 0 { v.push((i, j - 1)); }
	if i > 0 { v.push((i - 1, j)); }
	v
}

fn pa(area: &Area) {
	for row in area.iter() {
		for &c in row.iter() {
			print!("{}", if c { '#' } else { '.' });
		}
		println!("");
	}
	println!("{} {}", area.len(), area[0].len());
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

