use super::super::model::world::World;

pub struct GameOver {
	pub world: World,
}

impl GameOver {
	pub fn new(world: World) -> Self {
		Self { world }
	}
}
