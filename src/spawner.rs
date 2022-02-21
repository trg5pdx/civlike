use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::{Player, Unit, Renderable, Position, Viewshed};

pub fn player(ecs: &mut World, position: (i32, i32)) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: position.0, y: position.1 })
        .with(Renderable {
            glyph: to_cp437('+'),
            fg: RGB::named(PURPLE),
            bg: RGB::named(BLACK),
			render_order: 0,
        })
        .with(Player{})
        .build()
}

pub fn unit(ecs: &mut World, position: (i32, i32), range: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: position.0, y: position.1 })
        .with(Renderable {
            glyph: to_cp437('☺'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
			render_order: 1,
        })
		.with(Unit {
			health: 20,
			strength: 8,
			owner: "Player1".to_string(),
		})
		.with(Viewshed{ visible_tiles: Vec::new(), range, dirty: true})
        .build()
}
