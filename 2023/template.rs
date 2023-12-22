use std::fs;

fn main() {
	let lines = read_lines(".txt");
}

fn read_lines(f: &str) -> Vec<String> {
	fs::read_to_string(f).unwrap().lines().map(String::from).collect()
}

