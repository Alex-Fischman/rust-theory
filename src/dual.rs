use std::ops::*;

#[derive(Debug)]
pub struct Dual<T> {
    x: T,
    d: T,
}

impl<T> Dual<T> {
    pub fn new(x: T, d: T) -> Dual<T> {
        Dual { x: x, d: d }
    }
}

impl<T: num::Zero> Dual<T> {
    pub fn value(x: T) -> Dual<T> {
        Dual::new(x, num::Zero::zero())
    }
}

impl<T: num::One> Dual<T> {
    pub fn variable(x: T) -> Dual<T> {
        Dual::new(x, num::One::one())
    }
}

impl<T: Add<Output = T>> Add for Dual<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Dual::new(self.x + other.x, self.d + other.d)
    }
}

impl<T: Sub<Output = T>> Sub for Dual<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Dual::new(self.x - other.x, self.d - other.d)
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Mul for Dual<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Dual::new(self.x * other.x, self.d * other.x + self.x * other.d)
    }
}

impl<T: Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy> Div for Dual<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Dual::new(
            self.x / other.x,
            (self.d * other.x - self.x * other.d) / (other.x * other.x),
        )
    }
}

impl<T: Neg<Output = T>> Neg for Dual<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Dual::new(-self.x, -self.d)
    }
}
