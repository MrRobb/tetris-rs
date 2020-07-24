use super::super::model::board;
use super::super::model::board::Board;
use super::super::model::board::Piece;
use super::super::model::shape;
use std::ops::Range;

pub type Dna = [f64; 4];

pub struct Bot {
	dna: Dna,
}

impl Bot {
	pub fn new(dna: Dna) -> Self {
		Self { dna }
	}

	fn calc_score(&self, board: &Board) -> f64 {
		0.0
	}

	/// Calculates the desired position of the piece
	/// Returns a number in the range [2..9) as the x position and a Rotation.
	pub fn ask(&self, board: &Board) -> (usize, shape::Rotation) {
		(0, shape::Rotation::Rotate0)
	}
}

fn calc_x_range(board: &Board, current: &Piece) -> Range<usize> {
	let mut piece = current.clone();

	// Move max left
	while let Ok(moved_piece) = board::left(board, &piece) {
		piece = moved_piece;
	}

	let min = piece.position.x;

	while let Ok(moved_piece) = board::right(board, &piece) {
		piece = moved_piece;
	}

	let max = piece.position.x;

	min..max
}
