mod algebra;
mod collatz;
mod dual;

use crate::algebra::*;
use crate::dual::Dual;

fn main() {
	let x = Dual::variable(3.0);
	println!("{:?}", (x + Dual::value(1.0)) / Dual::value(2.0));

	println!();

	let mut m = new_matrix(vec![vec![1, 2, 0, -1], vec![2, 0, 1, 0], vec![1, 1, 0, 2]]);
	m[0].push(Expression::new_var("a"));
	m[1].push(Expression::new_var("b"));
	m[2].push(Expression::new_var("c"));
	let m = reduce(m);
	for r in m {
		for n in r {
			print!("{}, ", n.simplify());
		}
		println!();
	}

	println!();

	let m = new_matrix(vec![
		vec![1, -1, 1],
		vec![-2, 3, -1],
		vec![1, -6, -4],
		vec![2, -1, 3],
	]);
	let m = reduce(m);
	for r in m {
		for n in r {
			print!("{}, ", n.simplify());
		}
		println!();
	}

	collatz::main();
}
