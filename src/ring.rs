use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;
use num::complex::Complex;

const VARIABLE: &str = "x";

#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T: Debug + Copy + Add + Mul> {
	coefficients: Vec<T>
}

impl<T: Debug + Copy + Add<Output = T> + Mul<Output = T>> Add for Polynomial<T> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let mut to_return: Vec<T> = Vec::new();
		
		let (longer_coefficients, shorter_coefficients) = Self::get_longer_and_shorter(&self, &other);
		for (a, b) in longer_coefficients.iter().zip(shorter_coefficients.iter()) {
			let sum: T = a.clone() + b.clone();
			to_return.push(sum);
		}
		for i in shorter_coefficients.len()..longer_coefficients.len() {
			to_return.push(longer_coefficients[i].clone());
		}
		Polynomial::new(to_return)
	}
}

impl<T: Debug + Copy + Add<Output = T> + Mul<Output = T>> Mul for Polynomial<T> {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		let mut to_return: Vec<T> = Vec::new();
		
		let mut i = 0;
		for a in self.coefficients {
			let mut j = 0;
			for b in &other.coefficients {
				let power = i + j;
				let is_new_coefficient = power >= to_return.len();
				match is_new_coefficient {
					true => to_return.push(a.clone() * b.clone()),
					false => to_return[power] = to_return[power] + (a.clone() * b.clone())
				};
				j = j + 1;
			}
			i = i + 1;
		}

		return Polynomial::new(to_return);
	}
}

impl<T: Debug + Copy + Add + Mul> Polynomial<T> {

	pub fn new(coefficients: Vec<T>) -> Self {
		Self {
			coefficients
		}
	}

	pub fn zero() -> Self {
		Self::new(Vec::new())
	}

	pub fn get_degree(&self) -> i8 {
		match self.coefficients.len() {
			0 => -1,
			_ => (self.coefficients.len() - 1) as i8
		}
	}

	pub fn to_string(&self) -> String {
		match self.coefficients.len() {
			0 => "0".to_string(),
			_ => {
				let delimiter = " + ";
				let mut i = 0;
				let mut coefficients_clone = self.coefficients.clone();
				let mut so_far = Self::to_string_term(coefficients_clone.remove(0), VARIABLE.to_string(), i);
				for a in coefficients_clone {
					i = i + 1;
					so_far = format!("{}{}{}", so_far, delimiter, Self::to_string_term(a, VARIABLE.to_string(), i));
				}
				so_far
			}
		}
	}

	fn to_string_term(coefficient: T, variable: String, power: u16) -> String {
		return format!("{:?}{}", coefficient, Self::to_string_variable(variable, power));
	}

	fn to_string_variable(variable: String, power: u16) -> String {
		match power {
			0 => "".to_string(),
			1 => variable,
			_ => format!("{}^{}", variable, power)
		}
	}

	fn get_longer_and_shorter(left: &Polynomial<T>, right: &Polynomial<T>) -> (Vec<T>, Vec<T>) {
		let self_coefficients = left.coefficients.clone();
		let other_coefficients = right.coefficients.clone();
		let longer_coefficients = if self_coefficients.len() >= other_coefficients.len() { self_coefficients.clone() } else { other_coefficients.clone() };
		let shorter_coefficients = if self_coefficients.len() < other_coefficients.len() { self_coefficients } else { other_coefficients };
		return (longer_coefficients, shorter_coefficients);
	}

}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_polynomial_initializes() {
		let p_over_c = Polynomial::new(vec![Complex::new(2.0, 3.0), Complex::new(-6.5, 2.1)]);
		assert_eq!(p_over_c.to_string(), "Complex { re: 2.0, im: 3.0 } + Complex { re: -6.5, im: 2.1 }x".to_string());

		let p_over_r = Polynomial::new(vec![3.521, 9.0, -12.6, 4.5]);
		assert_eq!(p_over_r.to_string(), "3.521 + 9.0x + -12.6x^2 + 4.5x^3".to_string());

		let p_over_z = Polynomial::new(vec![-7, 4, -100]);
		assert_eq!(p_over_z.to_string(), "-7 + 4x + -100x^2".to_string());

		let same_p_over_z = Polynomial::new(vec![-7, 4, -100]);
		assert_eq!(p_over_z, same_p_over_z);

	}

	#[test]
	fn test_degree() {
		assert_eq!(Polynomial::<u16>::zero().get_degree(), -1);
		assert_eq!(Polynomial::new(vec!(5, 4, 3, 2, 1)).get_degree(), 4);
	}

	#[test]
	fn test_polynomial_adds() {

		let p = Polynomial::new(vec![3, 2, 1]);
		let q = Polynomial::new(vec![9, 5, 4, 2, 2]);

		let r = p.clone() + q.clone();
		assert_eq!(r, Polynomial::new(vec![12, 7, 5, 2, 2]));
		assert_eq!(q + p, r);
	}

	#[test]
	fn test_polynomial_adds_zero() {

		let p = Polynomial::new(vec![3, 2, 1]);
		let q = Polynomial::zero();

		let r = p.clone() + q.clone();
		assert_eq!(r, p);
		assert_eq!(q + p, r);
	}

	#[test]
	fn test_polynomial_multiplies() {

		let p = Polynomial::new(vec![9, 3, 1]); // x^2 + 3x + 9
		let q = Polynomial::new(vec![5, 1]); // x + 5

		let r = p.clone() * q.clone(); // x^3 + 8x^2 + 12x + 45
		assert_eq!(r, Polynomial::new(vec![45, 24, 8, 1]));
		assert_eq!(r, q * p);

	}

	#[test]
	fn test_polynomial_multiplies_zero() {

		let p = Polynomial::new(vec![9, 3, 1]);
		let q = Polynomial::zero();

		let r = p.clone() * q.clone();
		assert_eq!(r, Polynomial::zero());
		assert_eq!(r, q * p);

	}
}