use ggez::graphics::Color;
use na::DMatrix;

#[derive(PartialEq, Clone, Copy)]
pub enum Rotation {
	Rotate0,
	Rotate90,
	Rotate180,
	Rotate270,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
	I(u8),
	J(u8),
	L(u8),
	O(u8),
	S(u8),
	T(u8),
	Z(u8),
}

impl Shape {
	pub fn value(self) -> DMatrix<u8> {
		match self {
			Shape::I(0) => DMatrix::from_row_slice(4, 4, &[0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
			Shape::I(1) => DMatrix::from_row_slice(4, 4, &[0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0]),
			Shape::I(2) => DMatrix::from_row_slice(4, 4, &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]),
			Shape::I(3) => DMatrix::from_row_slice(4, 4, &[0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0]),
			Shape::J(0) => DMatrix::from_row_slice(3, 3, &[2, 0, 0, 2, 2, 2, 0, 0, 0]),
			Shape::J(1) => DMatrix::from_row_slice(3, 3, &[0, 2, 2, 0, 2, 0, 0, 2, 0]),
			Shape::J(2) => DMatrix::from_row_slice(3, 3, &[0, 0, 0, 2, 2, 2, 0, 0, 2]),
			Shape::J(3) => DMatrix::from_row_slice(3, 3, &[0, 2, 0, 0, 2, 0, 2, 2, 0]),
			Shape::L(0) => DMatrix::from_row_slice(3, 3, &[0, 0, 3, 3, 3, 3, 0, 0, 0]),
			Shape::L(1) => DMatrix::from_row_slice(3, 3, &[0, 3, 0, 0, 3, 0, 0, 3, 3]),
			Shape::L(2) => DMatrix::from_row_slice(3, 3, &[0, 0, 0, 3, 3, 3, 3, 0, 0]),
			Shape::L(3) => DMatrix::from_row_slice(3, 3, &[3, 3, 0, 0, 3, 0, 0, 3, 0]),
			Shape::O(_) => DMatrix::from_row_slice(2, 2, &[4, 4, 4, 4]),
			Shape::S(0) => DMatrix::from_row_slice(3, 3, &[0, 5, 5, 5, 5, 0, 0, 0, 0]),
			Shape::S(1) => DMatrix::from_row_slice(3, 3, &[0, 5, 0, 0, 5, 5, 0, 0, 5]),
			Shape::S(2) => DMatrix::from_row_slice(3, 3, &[0, 0, 0, 0, 5, 5, 5, 5, 0]),
			Shape::S(3) => DMatrix::from_row_slice(3, 3, &[5, 0, 0, 5, 5, 0, 0, 5, 0]),
			Shape::T(0) => DMatrix::from_row_slice(3, 3, &[0, 6, 0, 6, 6, 6, 0, 0, 0]),
			Shape::T(1) => DMatrix::from_row_slice(3, 3, &[0, 6, 0, 0, 6, 6, 0, 6, 0]),
			Shape::T(2) => DMatrix::from_row_slice(3, 3, &[0, 0, 0, 6, 6, 6, 0, 6, 0]),
			Shape::T(3) => DMatrix::from_row_slice(3, 3, &[0, 6, 0, 6, 6, 0, 0, 6, 0]),
			Shape::Z(0) => DMatrix::from_row_slice(3, 3, &[7, 7, 0, 0, 7, 7, 0, 0, 0]),
			Shape::Z(1) => DMatrix::from_row_slice(3, 3, &[0, 0, 7, 0, 7, 7, 0, 7, 0]),
			Shape::Z(2) => DMatrix::from_row_slice(3, 3, &[0, 0, 0, 7, 7, 0, 0, 7, 7]),
			Shape::Z(3) => DMatrix::from_row_slice(3, 3, &[0, 7, 0, 7, 7, 0, 7, 0, 0]),
			_ => unreachable!(),
		}
	}

	pub fn color(self) -> Color {
		match self {
			Shape::I(_) => Color::from_rgb(249, 35, 56),
			Shape::J(_) => Color::from_rgb(201, 115, 255),
			Shape::L(_) => Color::from_rgb(28, 118, 188),
			Shape::O(_) => Color::from_rgb(254, 227, 86),
			Shape::S(_) => Color::from_rgb(83, 213, 4),
			Shape::T(_) => Color::from_rgb(54, 224, 255),
			Shape::Z(_) => Color::from_rgb(248, 147, 29),
		}
	}

	pub fn rotate_clockwise(self) -> Self {
		match self {
			Shape::I(i) => Shape::I((i + 1) % 4),
			Shape::J(i) => Shape::J((i + 1) % 4),
			Shape::L(i) => Shape::L((i + 1) % 4),
			Shape::O(i) => Shape::O((i + 1) % 4),
			Shape::S(i) => Shape::S((i + 1) % 4),
			Shape::T(i) => Shape::T((i + 1) % 4),
			Shape::Z(i) => Shape::Z((i + 1) % 4),
		}
	}

	pub fn x(self) -> usize {
		for (i, col) in self.value().column_iter().enumerate() {
			for cell in col.iter() {
				if *cell > 0_u8 {
					return i;
				}
			}
		}
		0_usize
	}

	pub fn y(self) -> usize {
		for (i, row) in self.value().row_iter().enumerate() {
			for cell in row.iter() {
				if *cell > 0_u8 {
					return i;
				}
			}
		}
		0_usize
	}

	pub fn w(self) -> usize {
		let mut width = self.value().ncols();
		for col in self.value().column_iter() {
			let mut count = 0;
			for cell in col.iter() {
				count += if *cell == 0 { 0 } else { 1 };
			}
			if count == 0 {
				width -= 1;
			}
		}
		width
	}

	pub fn h(self) -> usize {
		let mut height = self.value().ncols();
		for row in self.value().row_iter() {
			let mut count = 0;
			for cell in row.iter() {
				count += if *cell == 0 { 0 } else { 1 };
			}
			if count == 0 {
				height -= 1;
			}
		}
		height
	}

	pub fn from_index(index: u8) -> Option<Self> {
		match index {
			1 => Some(Shape::I(0)),
			2 => Some(Shape::J(0)),
			3 => Some(Shape::L(0)),
			4 => Some(Shape::O(0)),
			5 => Some(Shape::S(0)),
			6 => Some(Shape::T(0)),
			7 => Some(Shape::Z(0)),
			_ => None,
		}
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn shape_x_test() {
		assert_eq!(0, Shape::I(0).x());
		assert_eq!(0, Shape::J(0).x());
		assert_eq!(0, Shape::L(0).x());
		assert_eq!(0, Shape::S(0).x());
	}

	#[test]
	fn shape_y_test() {
		assert_eq!(1, Shape::I(0).y());
		assert_eq!(0, Shape::J(0).y());
		assert_eq!(0, Shape::L(0).y());
		assert_eq!(0, Shape::S(0).y());
	}

	#[test]
	fn shape_w_test() {
		assert_eq!(4, Shape::I(0).w());
		assert_eq!(3, Shape::J(0).w());
		assert_eq!(3, Shape::L(0).w());
		assert_eq!(3, Shape::S(0).w());
	}

	#[test]
	fn shape_h_test() {
		assert_eq!(1, Shape::I(0).h());
		assert_eq!(2, Shape::J(0).h());
		assert_eq!(2, Shape::L(0).h());
		assert_eq!(2, Shape::S(0).h());
	}
}
