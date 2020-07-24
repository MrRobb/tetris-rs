use super::shape::{self, Shape};
use na::{zero, MatrixMN, Vec2, U12, U22};
use rand::prelude::*;
use rand::Rng;

// Struct: Piece

#[derive(Clone, Copy)]
pub struct Piece {
	pub position: Vec2<usize>,
	pub shape: Shape,
}

impl Piece {
	pub fn new(x: usize, shape: Shape) -> Self {
		const Y: usize = 2;

		Self {
			position: Vec2::new(x, Y),
			shape,
		}
	}

	fn x(self, x: usize) -> Self {
		Self {
			position: Vec2::new(x, self.position.y),
			shape: self.shape,
		}
	}

	fn y(self, y: usize) -> Self {
		Self {
			position: Vec2::new(self.position.x, y),
			shape: self.shape,
		}
	}

	fn shape(self, shape: Shape) -> Self {
		Self {
			position: self.position,
			shape,
		}
	}
}

// Struct: PieceCollector

#[derive(Clone)]
pub struct PieceCollector {
	current_shape: u8,
	next_shape: u8,
	rng: StdRng,
}

impl PieceCollector {
	fn new(seed: u64) -> Self {
		let mut rng = StdRng::seed_from_u64(seed);
		let current_shape = rng.gen_range(1_u8, 8_u8);
		let next_shape = rng.gen_range(1_u8, 8_u8);

		Self {
			current_shape,
			next_shape,
			rng,
		}
	}

	pub fn get_current(&self) -> Shape {
		Shape::from_index(self.current_shape).unwrap()
	}

	fn get_next(&self) -> Shape {
		Shape::from_index(self.next_shape).unwrap()
	}

	pub fn next(&mut self) {
		self.current_shape = self.next_shape;
		self.next_shape = self.rng.gen_range(1_u8, 8_u8);
	}
}

// Struct: Board

#[derive(Clone)]
pub struct Board {
	pub collector: PieceCollector,
	pub grid: MatrixMN<u8, U22, U12>,
	pub current: Piece,
}

impl Board {
	/// Creates a new empty board with a random current piece
	pub fn new(seed: u64) -> Self {
		// Initialize grid
		let mut grid: MatrixMN<u8, U22, U12> = zero();
		grid.fill_row(1, 8_u8);
		grid.fill_row(20, 8_u8);
		grid.fill_column(1, 8_u8);
		grid.fill_column(10, 8_u8);

		// Set collector with seed
		let collector = PieceCollector::new(seed);

		// Set current piece
		let current_piece = Piece {
			position: [4, 2].into(),
			shape: collector.get_current(),
		};

		// Build state
		Self {
			grid,
			collector,
			current: current_piece,
		}
	}

	/// Transfer current piece to grid
	pub fn place_current_piece(&mut self) {
		let offset_y: usize = self.current.position.y;
		let offset_x: usize = self.current.position.x;
		let x = self.current.shape.x();
		let y = self.current.shape.y();
		let w = self.current.shape.w();
		let h = self.current.shape.h();

		let mut slice_grid = self.grid.slice_mut((offset_y + y, offset_x + x), (h, w));

		for (x1, x2) in slice_grid
			.iter_mut()
			.zip(self.current.shape.value().slice((y, x), (h, w)).iter())
		{
			if *x2 != 0 {
				*x1 = *x2;
			}
		}
	}

	/// Remove full lines and return the number of lines removed
	pub fn remove_full_lines(&mut self) -> usize {
		let mut count_lines = 0_usize;

		for i in 2..self.grid.nrows() - 2 {
			if self.grid.row(i).iter().filter(|x| **x == 0).count() == 2 {
				// Log success
				leg::success("Line completed", "\u{1f37b}".into(), None);

				// Count
				count_lines += 1;

				// Update values
				for ii in (3_usize..=i).rev() {
					self.grid.swap_rows(ii, ii - 1);
				}
				self.grid.fill_row(2, 0);
				self.grid[(2, 1)] = 8_u8;
				self.grid[(2, 10)] = 8_u8;
			}
		}

		count_lines
	}

	/// Moves the current piece to the x specified
	pub fn move_current_to(&mut self, x: usize) {
		let mut tries = 0;

		while self.current.position.x < x && tries < 5 {
			if let Ok(piece) = right(&self, &self.current) {
				self.current = piece
			}
			tries += 1;
		}

		while self.current.position.x > x && tries < 5 {
			if let Ok(piece) = left(&self, &self.current) {
				self.current = piece
			}
			tries += 1;
		}
	}

	/// Rotates the current piece the degrees specified (clockwise)
	pub fn rotate_current(&mut self, degrees: shape::Rotation) {
		match degrees {
			shape::Rotation::Rotate0 => (),
			shape::Rotation::Rotate90 => self.current.shape = self.current.shape.rotate_clockwise(),
			shape::Rotation::Rotate180 => self.current.shape = self.current.shape.rotate_clockwise().rotate_clockwise(),
			shape::Rotation::Rotate270 => {
				self.current.shape = self
					.current
					.shape
					.rotate_clockwise()
					.rotate_clockwise()
					.rotate_clockwise()
			},
		}
	}

	/// Checks if you have reached the top
	pub fn is_gameover(&self) -> bool {
		self.grid.row(2).iter().sum::<u8>() > (8 * 2)
	}
}

pub enum BoardError {
	TouchingGround,
	UnableToMove,
	UnableToRotate,
}

/// Moves the piece one position down if possible
pub fn down(board: &Board, piece: &Piece) -> Result<Piece, BoardError> {
	if can_down(board, piece) {
		Ok(piece.clone().y(piece.position.y + 1))
	}
	else {
		Err(BoardError::TouchingGround)
	}
}

/// Moves the piece one position to the right if possible
pub fn right(board: &Board, piece: &Piece) -> Result<Piece, BoardError> {
	if can_right(board, piece) {
		Ok(piece.clone().x(piece.position.x + 1))
	}
	else {
		Err(BoardError::UnableToMove)
	}
}

/// Moves the piece one position to the left if possible
pub fn left(board: &Board, piece: &Piece) -> Result<Piece, BoardError> {
	if can_left(board, piece) {
		Ok(piece.clone().x(piece.position.x - 1))
	}
	else {
		Err(BoardError::UnableToMove)
	}
}

/// Rotates the piece if possible
pub fn rotate(board: &Board, piece: &Piece) -> Result<Piece, BoardError> {
	can_rotate(board, piece)
}

// Conditions

fn can_down(board: &Board, piece: &Piece) -> bool {
	!overlapping(board, piece, 0, 1)
}

fn can_left(board: &Board, piece: &Piece) -> bool {
	!overlapping(board, piece, -1, 0)
}

fn can_right(board: &Board, piece: &Piece) -> bool {
	!overlapping(board, piece, 1, 0)
}

fn can_rotate(board: &Board, piece: &Piece) -> Result<Piece, BoardError> {
	let mut new_piece = piece.clone().shape(piece.shape.rotate_clockwise());

	let mut i = 0;
	while i < 4 && overlapping(board, &new_piece, 0, 0) {
		if !overlapping(board, &new_piece, -1, 0) {
			new_piece = new_piece.x(new_piece.position.x - 1);
		}
		else if !overlapping(board, &new_piece, 1, 0) {
			new_piece = new_piece.x(new_piece.position.x + 1);
		}
		else if (new_piece.shape == Shape::I(0) || new_piece.shape == Shape::I(2))
			&& !overlapping(board, &new_piece, -2, 0)
		{
			new_piece = new_piece.x(new_piece.position.x - 2);
		}
		else if (new_piece.shape == Shape::I(0) || new_piece.shape == Shape::I(2))
			&& !overlapping(board, &new_piece, 2, 0)
		{
			new_piece = new_piece.x(new_piece.position.x + 2);
		}
		else {
			return Err(BoardError::UnableToRotate);
		}
		i += 1;
	}

	Ok(new_piece)
}

pub fn overlapping(board: &Board, piece: &Piece, offset_x: i64, offset_y: i64) -> bool {
	let global_offset_y = piece.position.y;
	let global_offset_x = piece.position.x;
	let x = piece.shape.x();
	let y = piece.shape.y();
	let w = piece.shape.w();
	let h = piece.shape.h();

	if (global_offset_y as i64 + y as i64 + offset_y) < 0
		|| (global_offset_x as i64 + x as i64 + offset_x) < 0
		|| (global_offset_y as i64 + y as i64 + offset_y) as usize >= board.grid.nrows()
		|| (global_offset_x as i64 + x as i64 + offset_x) as usize >= board.grid.ncols()
	{
		return true;
	}

	let slice_grid = board.grid.slice(
		(
			(global_offset_y as i64 + y as i64 + offset_y) as usize,
			(global_offset_x as i64 + x as i64 + offset_x) as usize,
		),
		(h, w),
	);

	slice_grid
		.iter()
		.zip(piece.shape.value().slice((y, x), (h, w)).iter())
		.any(|(x1, x2)| *x1 != 0 && *x2 != 0)
}
