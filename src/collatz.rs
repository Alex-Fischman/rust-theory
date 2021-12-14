fn factorize(n: u64) -> Vec<(u64, u64)> {
	for m in 2..num::integer::sqrt(n) + 1 {
		if n % m == 0 {
			let mut out = factorize(n / m);
			if out[0].0 == m {
				out[0].1 += 1;
			} else {
				out.insert(0, (m, 1));
			}
			return out;
		}
	}
	vec![(n, 1)]
}

pub fn main() {
	let max_value = 200;
	let mut nodes = vec![1];
	let mut i = 0;
	let mut graph = String::new();

	graph += "strict digraph {\n";
	while i < nodes.len() {
		if nodes[i] <= max_value && nodes[i] % 3 != 0 {
			let a = nodes[i];
			let b = nodes[i] * 2;
			let c = (nodes[i] - 1) / 3;
			if c != 0 && c != 1 && nodes[i] % 3 == 1 && c % 2 == 1 {
				graph += &format!("\t{} -> {}\n", a, c);
				nodes.push(c);
			}
			graph += &format!("\t{} -> {}\n", a, b);
			nodes.push(b);
		}
		i += 1;
	}

	for n in nodes {
		let mut factors = String::new();
		for (factor, count) in factorize(n) {
			factors += &format!(
				"{}<SUP><FONT POINT-SIZE=\"12\">{}</FONT></SUP>",
				factor, count
			);
		}

		graph += &format!(
			"\t{} [color=\"{}\", label=<{}>]\n",
			n,
			match n % 3 {
				0 => "red",
				1 => "green",
				2 => "blue",
				_ => unreachable!(),
			},
			factors,
		);
	}

	graph += "}\n";

	std::fs::write("collatz.dot", graph).unwrap();
	std::process::Command::new("dot")
		.arg("-Tsvg")
		.arg("collatz.dot")
		.arg("-ocollatz.svg")
		.status()
		.unwrap();
}
