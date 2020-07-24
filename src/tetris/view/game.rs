use super::super::model::world::World;
use super::subgame::Player;
use super::subgame::SubGame;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{clear, DrawParam, Drawable, MeshBuilder, Rect};
use ggez::{event, graphics, timer, Context, ContextBuilder, GameResult};

pub struct Game {
	pub world: World,
	games: Vec<SubGame>,
}

impl Game {
	pub fn new(world: World) -> Self {
		let mut games = vec![];

		let col_offset = world.config.window_mode.width / world.ncols as f32;
		let row_offset = world.config.window_mode.height / world.nrows as f32;

		for i in 0..world.nrows {
			for j in 0..world.ncols {
				let rect = Rect {
					x: col_offset * j as f32,
					y: row_offset * i as f32,
					w: col_offset,
					h: row_offset,
				};
				if world.has_player && i == 0 && j == 0 {
					games.push(SubGame::new(rect, world.seed, Player::Human));
				}
				else {
					games.push(SubGame::new(rect, world.seed, Player::Bot));
				}
			}
		}

		Self { world, games }
	}

	pub fn start(&mut self) {
		let (ctx, event_loop) = &mut ContextBuilder::new("Tetris", "Mr.Robb")
			.conf(self.world.config.clone())
			.with_conf_file(true)
			.build()
			.expect(" ._. Could not create ggez context");

		event::run(ctx, event_loop, self).expect("Dirty exit");
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		const FPS: u32 = 2;

		while timer::check_update_time(ctx, FPS) {
			for game in &mut self.games {
				game.update();
			}
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		clear(ctx, (28.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0, 1.0).into());

		let mut builder = MeshBuilder::new();

		for game in &mut self.games {
			game.draw(&mut builder);
		}

		let mesh = builder.build(ctx);

		if mesh.is_ok() {
			mesh.unwrap()
				.draw(ctx, DrawParam::default())
				.expect("Could not draw the mesh");
		}

		graphics::present(ctx).expect("Could not present the scene");

		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
		for game in &mut self.games {
			game.key_down_event(ctx, keycode, keymods, repeat);
		}
	}
}
