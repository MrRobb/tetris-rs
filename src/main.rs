extern crate ggez;
extern crate na;
extern crate rand;

mod tetris;

use ggez::conf::{Backend, Conf, ModuleConf, NumSamples, WindowMode, WindowSetup};
use tetris::model::world::World;
use tetris::view::window::Window;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	// Rows

	let nrows = if args.len() >= 3 {
		std::cmp::max(1, args[1].parse().unwrap_or(5_usize))
	}
	else {
		1_usize
	};

	// Columns

	let ncols = if args.len() >= 3 {
		std::cmp::max(1, args[2].parse().unwrap_or(7_usize))
	}
	else {
		1_usize
	};

	// Player

	let has_player: bool = if args.len() >= 4 {
		args[3].parse().unwrap_or(false)
	}
	else {
		true
	};

	// Config

	let window_mode = WindowMode::default()
		.dimensions(800.0, 600.0)
		.resizable(true);

	let window_setup = WindowSetup::default()
		.title("Tetris")
		.vsync(true)
		//.transparent(false)
		.samples(NumSamples::Zero);

	let config = Conf {
		window_mode,
		window_setup,
		backend: Backend::default(),
		modules: ModuleConf::default(),
	};

	// Seed

	let seed: u64 = rand::random();

	// World

	let world = World {
		nrows,
		ncols,
		has_player,
		config,
		seed,
	};

	Window::new(world).run()
}
