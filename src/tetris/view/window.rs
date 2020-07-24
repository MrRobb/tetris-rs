use super::super::model::world::World;
use super::game::Game;
use super::gameover::GameOver;
use super::menu::Menu;
use super::scene::Scene;

pub enum Window {
	Menu(Menu),
	Game(Game),
	GameOver(GameOver),
}

impl Window {
	pub fn new(world: World) -> Self {
		Window::Menu(Menu::new(world))
	}

	pub fn run(mut self) {
		loop {
			self = match self {
				Window::Menu(m) => m.run(),
				Window::Game(g) => g.run(),
				Window::GameOver(o) => {
					o.run();
					break;
				},
			}
		}
	}
}
