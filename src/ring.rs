use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;
use num::complex::Complex;

const VARIABLE: &str = "x";

#[derive(Debug, PartialEq)]
pub struct Polynomial<T: Debug + Clone + Add + Mul> {
	coefficients: Vec<T>
}

impl<T: Debug + Clone + Add<Output = T> + Mul<Output = T>> Add for Polynomial<T> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let mut to_return: Vec<T> = Vec::new();
		let self_coefficients = self.coefficients.clone();
		let other_coefficients = other.coefficients.clone();
		let longer_coefficients = if self_coefficients.len() >= other_coefficients.len() { self_coefficients.clone() } else { other_coefficients.clone() };
		let shorter_coefficients = if self_coefficients.len() < other_coefficients.len() { self_coefficients } else { other_coefficients };
		for (a, b) in longer_coefficients.iter().zip(shorter_coefficients.iter()) {
			let sum: T = a.clone() + b.clone();
			to_return.push(sum);
		}
		for i in (longer_coefficients.len() - shorter_coefficients.len() + 1)..(longer_coefficients.len()) {
			to_return.push(longer_coefficients[i].clone());
		}
		Polynomial::new(to_return)
	}
}

impl<T: Debug + Clone + Add + Mul> Polynomial<T> {

	pub fn new(coefficients: Vec<T>) -> Self {
		Self {
			coefficients
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
	fn test_polynomial_adds() {

		let p = Polynomial::new(vec![3, 2, 1]);
		let q = Polynomial::new(vec![9, 5, 4, 2, 2]);

		let r = p + q;
		assert_eq!(r, Polynomial::new(vec![12, 7, 5, 2, 2]));
	}
}