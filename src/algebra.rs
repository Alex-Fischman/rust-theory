use num::One;
use num::Zero;

use Expression::Add;
use Expression::Inv;
use Expression::Mul;
use Expression::Val;
use Expression::Var;

#[derive(Clone, Debug)]
pub enum Expression {
	Val(num::Rational32),
	Var(num::Rational32, String),
	Add(Box<Expression>, Box<Expression>),
	Mul(Box<Expression>, Box<Expression>),
	Inv(Box<Expression>),
}

impl Expression {
	pub fn new_var(s: &str) -> Expression {
		Expression::Var(num::One::one(), s.to_string())
	}

	pub fn simplify(&self) -> Expression {
		fn simplify_(e: &Expression) -> Expression {
			match e {
				Add(a, b) => match (simplify_(a), simplify_(b)) {
					(Val(x), Val(y)) => Val(x + y),
					(a, b) => a + b,
				},
				Mul(a, b) => match (simplify_(a), simplify_(b)) {
					(Val(x), Val(y)) => Val(x * y),
					(Val(x), Var(y, s)) | (Var(y, s), Val(x)) => Var(x * y, s),
					(Add(a, b), c) | (c, Add(a, b)) => simplify_(&((*a * c.clone()) + (*b * c))),
					(a, b) => a * b,
				},
				Inv(a) => match simplify_(a) {
					Val(x) => Val(x.recip()),
					a => Inv(Box::new(a)),
				},
				e => e.clone(),
			}
		}

		let mut flattened = vec![];
		fn flattener(out: &mut Vec<Expression>, e: &Expression) {
			if let Add(a, b) = e {
				flattener(out, a);
				flattener(out, b);
			} else {
				out.push(e.clone());
			}
		}
		flattener(&mut flattened, &simplify_(self));

		let mut val_sum: num::Rational32 = num::Zero::zero();
		let mut var_sums = std::collections::BTreeMap::new();

		let mut other: Vec<Expression> = flattened
			.into_iter()
			.filter(|e| match e {
				Val(x) => {
					val_sum += x;
					false
				}
				Var(x, s) => {
					*var_sums.entry(s.to_string()).or_insert(num::Zero::zero()) += x;
					false
				}
				_ => true,
			})
			.collect();

		other.extend(
			var_sums.iter().filter_map(|(k, v): (&String, &num::Rational32)| {
				if !v.is_zero() {
					Some(Var(*v, k.to_string()))
				} else {
					None
				}
			}),
		);

		if !val_sum.is_zero() {
			other.push(Val(val_sum));
		}

		other.into_iter().reduce(|a, b| a + b).unwrap_or(Val(num::Zero::zero()))
	}
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Val(r) => write!(f, "{}", r),
			Var(r, s) => {
				if (-r).is_one() {
					write!(f, "-")?;
				} else if !r.is_one() {
					write!(f, "{}", r)?;
				}
				write!(f, "{}", s)
			}
			Add(a, b) => write!(f, "{}+{}", a, b),
			Mul(a, b) => write!(f, "{}*{}", a, b),
			Inv(e) => write!(f, "1/{}", e),
		}
	}
}

impl std::ops::Add for Expression {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Add(Box::new(self), Box::new(other))
	}
}

impl std::ops::Sub for Expression {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		self + (other * Val(num::rational::Ratio::from_integer(-1)))
	}
}

impl std::ops::Mul for Expression {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		Mul(Box::new(self), Box::new(other))
	}
}

impl std::ops::Div for Expression {
	type Output = Self;
	fn div(self, other: Self) -> Self {
		self * Inv(Box::new(other))
	}
}

impl Zero for Expression {
	fn zero() -> Self {
		Val(num::Zero::zero())
	}

	fn is_zero(&self) -> bool {
		match self.simplify() {
			Val(r) => r.is_zero(),
			_ => false,
		}
	}
}

pub type Row = Vec<Expression>;
pub type Matrix = Vec<Row>;

pub fn new_matrix(v: Vec<Vec<i32>>) -> Matrix {
	v.iter()
		.map(|v| v.iter().map(|x| Val(num::rational::Ratio::from_integer(*x))).collect())
		.collect()
}

fn scale(r: &Row, s: Expression) -> Row {
	r.iter().map(|t| s.clone() * t.clone()).collect()
}

fn subtract(r: &Row, s: &Row) -> Row {
	r.iter().enumerate().map(|(i, x)| x.clone() - s[i].clone()).collect()
}

pub fn reduce(m: Matrix) -> Matrix {
	let mut m: Matrix = m;
	for i in 0..m.len() {
		match m[i].iter().position(|s| !s.is_zero()) {
			Some(j) => {
				m[i] = scale(&m[i], Val(num::One::one()) / m[i][j].clone());
				m = m
					.iter()
					.enumerate()
					.map(|(k, r)| {
						if k <= i || r[j].is_zero() {
							r.to_vec()
						} else {
							subtract(&scale(r, Val(num::One::one()) / r[j].clone()), &m[i])
						}
					})
					.collect();
			}
			None => (),
		}
	}
	for i in (1..m.len()).rev() {
		match m[i].iter().position(|s| !s.is_zero()) {
			Some(j) => {
				m = m
					.iter()
					.enumerate()
					.map(|(k, r)| {
						if i <= k {
							r.to_vec()
						} else {
							subtract(&r, &scale(&m[i], r[j].clone()))
						}
					})
					.collect();
			}
			None => (),
		}
	}
	m
}
