use std::{
	cmp::PartialEq,
	fmt::Display,
	ops::{
		Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
		BitXorAssign, Not, Shl, Shr, SubAssign,
	},
};

#[derive(Clone, Copy)]
pub struct JsInt(i64);

impl Display for JsInt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<i32> for JsInt {
	fn from(value: i32) -> Self {
		Self(value as i64)
	}
}

impl From<u32> for JsInt {
	fn from(value: u32) -> Self {
		Self(value as i64)
	}
}

impl BitOr for JsInt {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self::from(self.0 as i32 | rhs.0 as i32)
	}
}

impl BitOr<usize> for JsInt {
	type Output = Self;

	fn bitor(self, rhs: usize) -> Self::Output {
		Self::from(self.0 as i32 | rhs as i32)
	}
}

impl BitOrAssign for JsInt {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 = (self.0 as i32 | rhs.0 as i32) as i64;
	}
}

impl Shl<usize> for JsInt {
	type Output = Self;

	fn shl(self, rhs: usize) -> Self::Output {
		Self::from((self.0 as i32).wrapping_shl(rhs as u32))
	}
}

impl Shr<usize> for JsInt {
	type Output = Self;

	fn shr(self, rhs: usize) -> Self::Output {
		Self::from(self.0 as u32 >> ((rhs as u32) % 32))
	}
}

impl Add for JsInt {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl BitAnd for JsInt {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self::from(self.0 as i32 & rhs.0 as i32)
	}
}

impl BitAnd<i64> for JsInt {
	type Output = Self;

	fn bitand(self, rhs: i64) -> Self::Output {
		Self::from(self.0 as i32 & rhs as i32)
	}
}

impl BitAndAssign for JsInt {
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 = (self.0 as i32 & rhs.0 as i32) as i64;
	}
}

impl BitXor for JsInt {
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self::Output {
		Self::from(self.0 as i32 ^ rhs.0 as i32)
	}
}

impl BitXorAssign for JsInt {
	fn bitxor_assign(&mut self, rhs: Self) {
		self.0 = (self.0 as i32 ^ rhs.0 as i32) as i64;
	}
}

impl Not for JsInt {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self::from(!(self.0 as i32))
	}
}

impl PartialEq<i64> for JsInt {
	fn eq(&self, other: &i64) -> bool {
		self.0 == *other
	}
}

impl AddAssign<JsInt> for usize {
	fn add_assign(&mut self, rhs: JsInt) {
		*self += rhs.0 as usize;
	}
}

impl SubAssign<JsInt> for usize {
	fn sub_assign(&mut self, rhs: JsInt) {
		*self -= rhs.0 as usize;
	}
}

impl JsInt {
	pub fn new(v: i64) -> Self {
		Self(v)
	}

	pub fn u32_assign(&mut self) {
		self.0 = self.0 as u32 as i64;
	}
}
