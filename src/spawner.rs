use crate::{BlocksTile, Name, Player, Position, Renderable, Unit, UnitControl, Viewshed};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn player(ecs: &mut World, position: (i32, i32)) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: position.0,
            y: position.1,
        })
        .with(Renderable {
            glyph: to_cp437('+'),
            fg: RGB::named(PURPLE),
            bg: RGB::named(BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 0, // Setting cursor range to 0 to allow the cursor to walk through revealed tiles & not reveal new territory
            dirty: true,
        })
        .with(Name {
            name: "Player1".to_string(),
        })
        .build()
}

pub fn unit(
    ecs: &mut World,
    position: (i32, i32),
    name: String,
    range: i32,
    _player: &Entity,
) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: position.0,
            y: position.1,
        })
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
        .with(BlocksTile {})
        .with(Name { name })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range,
            dirty: true,
        })
        .with(BlocksTile {})
        .build()
    /*
    let mut owned_units = ecs.write_storage::<UnitControl>();

    owned_units.insert(
        *player,
        UnitControl {
            owned_by: *player,
            unit,
        }).expect("failed to mark unit as owned");

    println!("test");

    unit */
}
