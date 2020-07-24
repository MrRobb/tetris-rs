use super::super::model::world::World;

pub struct Menu {
	pub world: World,
}

impl Menu {
	pub fn new(world: World) -> Self {
		Self { world }
	}
}
