use crate::{BlocksTile, Name, Player, Position, Renderable, Unit, UnitControl, Viewshed};
use crate::PlayerOrder::*;
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
        .with(Player {
            order: PlayerOne,
        })
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

pub fn unit(ecs: &mut World, position: (i32, i32), name: String, range: i32) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: position.0,
            y: position.1,
        })
        .with(Renderable {
            glyph: to_cp437('i'),
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
        .build()
}

pub fn own_unit(ecs: &mut World, pos: (i32, i32)) {
    let mut controlled_by = ecs.write_storage::<UnitControl>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();

    for (unit_entity, _unit, position, name) in (&entities, &units, &positions, &names).join() {
        if position.x == pos.0 && position.y == pos.1 {
            println!("found: {}", name.name);
            let res = controlled_by
                .insert(
                    *player_entity,
                    UnitControl {
                        owned_by: *player_entity,
                        unit: unit_entity,
                    },
                )
                .expect("Unable to add unit");

            println!("res: {:?}", res);
        }
    }
}
