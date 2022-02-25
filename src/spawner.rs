use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::{Player, Unit, Name, Renderable, Position, Viewshed, BlocksTile};

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
        .with(Viewshed{ visible_tiles: Vec::new(), range: 0, dirty: true}) 
        .with(Name{name: "Player1".to_string() })
        .build()
        // Setting the cursor range to 0 so it can be visible while walking through areas that are revealed but not currently
        // visible, and to prevent it from revealing new tiles on it's own
}

pub fn unit(ecs: &mut World, position: (i32, i32), name: String, range: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: position.0, y: position.1 })
        .with(Renderable {
            glyph: to_cp437('¡'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
			render_order: 1,
        })
		.with(Unit {
			health: 20,
			strength: 8,
		})
        .with(BlocksTile{})
        .with(Name{ name })
		.with(Viewshed{ visible_tiles: Vec::new(), range, dirty: true})
        .build()
}
