use super::game::Game;
use super::gameover::GameOver;
use super::menu::Menu;
use super::window::Window;

// State Container

pub trait Scene {
	fn run(self) -> Window;
}

// States

impl Scene for Menu {
	fn run(self) -> Window {
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		leg::head("Tetris", Some("\u{1f579}\u{fe0f}"), Some(VERSION));
		leg::info("Menu", "Scene".into(), None);
		Window::Game(self.into())
	}
}

impl Scene for Game {
	fn run(mut self) -> Window {
		leg::info("Game", "Scene".into(), None);
		self.start();
		Window::GameOver(self.into())
	}
}

impl Scene for GameOver {
	fn run(self) -> Window {
		leg::info("Game over", "Scene".into(), None);
		Window::GameOver(self)
	}
}

// Transitions

impl From<Menu> for Game {
	fn from(value: Menu) -> Self {
		Self::new(value.world)
	}
}

impl From<Game> for GameOver {
	fn from(value: Game) -> Self {
		Self::new(value.world)
	}
}
