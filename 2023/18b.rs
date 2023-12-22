use std::fs;
use std::iter::zip;
use std::collections::HashMap;

fn main() {
	let lines = read_lines("18.txt");

	let instructions: Vec<(Heading, i64)> = lines
		.iter()
		.map(|l| l.split(' ').collect::<Vec<&str>>()[2])
		.map(|l| l.chars().collect::<Vec<char>>())
		.map(|l| (l[2+5], l[2..2+5].to_vec()))
		.map(|(a, b)| (to_heading(a), b.into_iter().fold(0, |r, c| r * 16 + c.to_digit(16).unwrap() as i64)))
		.collect();

	/*
	let instructions: Vec<(Heading, i64)> = lines
		.iter()
		.map(|l| l.split(' ').collect::<Vec<&str>>())
		.map(|l| (l[0].chars().next().unwrap(), l[1].chars().collect::<Vec<char>>()))
		.map(|l| 
			(to_heading2(l.0), l.1.into_iter().fold(0, |r, a| r * 10 + a.to_digit(10).unwrap() as i64))
		)
		.collect();
	*/
	

	let mut h: Heading = instructions[0].0;
	let (mut left, mut right) = (0, 0);

	for &(heading, _) in instructions.iter() {
		let dir = to_dir(h, heading);
		if dir == Direction::Left { left += 1; }
		else if dir == Direction::Right { right += 1; }
		h = heading;
	}

	let g_curve = if right > left { Curve::Clock } else { Curve::Anti };
	println!("left: {}, right: {}, g_curve: {:?}", left, right, g_curve);

	let mut stack: Stack = Vec::with_capacity(instructions.len() + 1);
	stack.push((0, 0));
	let mut h: Heading = instructions[0].0;
	let mut d: Direction = g_curve.to_dir();
	let (mut a, mut b): Modifier = (0, 0);

	for (heading, num) in instructions.into_iter() {

		let dir = to_dir(h, heading);

		let m = if let Some(l_curve) = Curve::from_dirs(&d, &dir) {
			if l_curve == g_curve { 1 } else { -1 }
		} else {
			0
		};

		// println!("[{:?}] dir: {:?}, num: {}, m: {}", heading, dir, num, m);

		let mut last = stack.pop().unwrap();
		last = (last.0 + a * m, last.1 + b * m);
		stack.push(last);

		(a, b) = if heading == Heading::N {
			(0, 1)
		} else if heading == Heading::W {
			(-1, 0)
		} else if heading == Heading::S {
			(0, -1)
		} else {
			(1, 0)
		};

		let mut c = *stack.last().unwrap();
		c.0 += a * num;
		c.1 += b * num;

		stack.push(c);
		h = heading;
		d = if dir != Direction::Straight { dir } else { d };

	}

	/*
	for c in stack.iter() {
		println!("({}, {})", c.0, c.1);
	}

	println!("stack: {}", stack.len());
	*/


	let mut xs: Vec<_> = stack.clone().into_iter().map(|c| c.0).collect();
	xs.pop();
	let mut ys: Vec<_> = stack.clone().into_iter().map(|c| c.1).collect();
	ys.remove(0);

	let p = zip(xs, ys).fold(0, |r, (x, y)| r + x * y);

	let mut xs: Vec<_> = stack.clone().into_iter().map(|c| c.0).collect();
	xs.remove(0);
	let mut ys: Vec<_> = stack.clone().into_iter().map(|c| c.1).collect();
	ys.pop();

	let q = zip(xs, ys).fold(0, |r, (x, y)| r + x * y);

	println!("p: {}, q: {}", p, q);

	let s = (p - q).abs() / 2;

	println!("s: {}", s);

}

type Stack = Vec<Pos>;
type Pos = (i64, i64);
type Modifier = (i64, i64);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Heading { N, W, S, E }

fn to_heading(c: char) -> Heading {
	match c {
		'3' => Heading::N,
		'2' => Heading::W,
		'1' => Heading::S,
		_ => Heading::E
	}
}

fn to_heading2(c: char) -> Heading {
	match c {
		'U' => Heading::N,
		'L' => Heading::W,
		'D' => Heading::S,
		_ => Heading::E
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction { Left, Right, Straight }

fn to_dir(a: Heading, b: Heading) -> Direction {
	let h = HashMap::from([
		((Heading::N, Heading::W), Direction::Left),
		((Heading::W, Heading::S), Direction::Left),
		((Heading::S, Heading::E), Direction::Left),
		((Heading::E, Heading::N), Direction::Left)
	]);

	if a == b { Direction::Straight }
	else if let Some(d) = h.get(&(a, b)) { *d }
	else { Direction::Right }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Curve { Clock, Anti }

impl Curve {
	fn from_dirs(a: &Direction, b: &Direction) -> Option<Self> {
		match (a, b) { 
			(Direction::Left, Direction::Left) => Some(Self::Anti),
			(Direction::Right, Direction::Right) => Some(Self::Clock),
			_ => None
		}
	}
	
	fn to_dir(&self) -> Direction {
		match self {
			Self::Anti => Direction::Left,
			Self::Clock => Direction::Right
		}
	}
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

