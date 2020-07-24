extern crate ggez;
extern crate leg;
extern crate na;
extern crate rand;

use super::super::ai::bot::Bot;
use super::super::model::board::Board;
use super::super::model::board::*;
use super::super::model::shape::Shape;
use ggez::event::*;
use ggez::graphics::*;
use ggez::*;
use na::*;

struct TetrisDisplayConfig {
	x: f32,
	y: f32,
	w: f32,
	h: f32,
	block_size: f32,
}

pub struct SubGame {
	config: TetrisDisplayConfig,
	board: Board,
	bot: Option<Bot>,
}

#[derive(Clone, Copy)]
pub enum Player {
	Human,
	Bot,
}

impl SubGame {
	pub fn new(view: Rect, seed: u64, player: Player) -> Self {
		// Calculate values
		let block_size = view.h / 22.0;
		let x = view.x + view.w / 2.0 - block_size * 6.0;
		let y = view.y + view.h / 2.0 - block_size * 11.0;
		let w = view.w;
		let h = view.h;

		// Build bot
		let bot = match player {
			Player::Human => None,
			Player::Bot => Bot::new([0.0; 4]).into(),
		};

		// Build state
		Self {
			config: TetrisDisplayConfig { x, y, w, h, block_size },
			bot,
			board: Board::new(seed),
		}
	}

	pub fn update(&mut self) {
		match &self.bot {
			Some(bot) => {
				let (x, rotation) = bot.ask(&self.board);
				self.board.rotate_current(rotation);
				self.board.move_current_to(x);
			},
			None => (),
		}

		self.down();
	}

	pub fn draw(&self, builder: &mut MeshBuilder) -> bool {
		// Check if you lost <3
		if self.board.is_gameover() {
			return true;
		}

		// Draw board
		for (index, cell) in self.board.grid.slice_range(1..21, 1..11).iter().enumerate() {
			let j = 1 + index / 20;
			let i = 1 + index % 20;

			// println!("j: {}, i: {} -> cell: {}", j, i, *cell);

			let pos = &self.pt_from_world_to_wnd([j as f32, i as f32].into());
			let sz = self.config.block_size;

			// Get color
			let color: Color;
			let shape = Shape::from_index(*cell);
			if shape.is_some() {
				color = shape.unwrap().color();
			}
			else if *cell == 8_u8 {
				color = Color::new(33.0 / 255.0, 33.0 / 255.0, 35.0 / 255.0, 1.0);
			}
			else {
				color = Color::new(0.0, 0.0, 0.0, 0.0);
			}

			builder.circle(
				DrawMode::fill(),
				Point2::new(pos.x + sz / 2.0, pos.y + sz / 2.0),
				2.0,
				0.01,
				Color::new(37.0 / 255.0, 37.0 / 255.0, 39.0 / 255.0, 0.6),
			);

			builder.rectangle(DrawMode::fill(), Rect::new(pos.x, pos.y, sz, sz), color);
		}

		// panic!();

		// Draw current piece
		let position = &self.board.current.position;
		let sz = self.config.block_size;
		let shape = &self.board.current.shape.value();
		let color = self.board.current.shape.color();

		for (index, cell) in shape.iter().enumerate() {
			let j = (index / shape.nrows()) + position.x;
			let i = (index % shape.nrows()) + position.y;
			let pos = &self.pt_from_world_to_wnd([j as f32, i as f32].into());

			// Draw cell
			builder.rectangle(
				DrawMode::fill(),
				Rect::new(pos.x, pos.y, sz, sz),
				if *cell == 0_u8 {
					Color::new(0.0, 0.0, 0.0, 0.0)
				}
				else {
					color
				},
			);
		}

		false
	}

	pub fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
		// TODO: Change order of the match keycode and put first the self.bot.is_none()

		match keycode {
			KeyCode::Down => match down(&self.board, &self.board.current) {
				Ok(piece) => self.board.current = piece,
				Err(BoardError::TouchingGround) => self.down(),
				_ => unreachable!(),
			},
			KeyCode::Left => {
				if self.bot.is_none() {
					if let Ok(piece) = left(&self.board, &self.board.current) {
						self.board.current = piece
					}
				}
			},
			KeyCode::Right => {
				if self.bot.is_none() {
					if let Ok(piece) = right(&self.board, &self.board.current) {
						self.board.current = piece
					}
				}
			},
			KeyCode::Up => {
				if self.bot.is_none() {
					if let Ok(piece) = rotate(&self.board, &self.board.current) {
						self.board.current = piece
					}
				}
			},
			_ => (),
		}
	}

	fn down(&mut self) {
		self.board.current = down(&self.board, &self.board.current).unwrap_or_else(|_| {
			self.board.place_current_piece();
			self.board.remove_full_lines();
			self.board.collector.next();
			let current = self.board.collector.get_current();
			Piece::new(self.board.grid.ncols() / 2 - current.w() / 2, current)
		});
	}

	// Helpers

	fn pt_from_wnd_to_world(&self, point: Vec2) -> Vec2 {
		let x = (point.x - self.config.x) / self.config.block_size;
		let y = (point.y - self.config.y) / self.config.block_size;
		Vec2::new(x, y)
	}

	fn pt_from_world_to_wnd(&self, point: Vec2) -> Vec2 {
		let x = self.config.x + point.x * self.config.block_size;
		let y = self.config.y + point.y * self.config.block_size;
		Vec2::new(x, y)
	}
}
