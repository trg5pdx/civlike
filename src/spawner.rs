use bracket_lib::prelude::*;
use rand::{thread_rng, Rng};
use specs::prelude::*;

use crate::{
    xy_idx, BlocksTile, Fort, Map, Name, Player, PlayerOrder, Position, Renderable, Unit, Viewshed,
};

fn player(ecs: &mut World, position: (i32, i32), order: PlayerOrder) -> Entity {
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
            order,
            unit_count: 0,
            fort_count: 0,
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
    player: PlayerOrder,
) -> Entity {
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
            owner: player,
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

pub fn fort(ecs: &mut World, position: (i32, i32), name: String, owner: PlayerOrder) -> Entity {
    ecs.create_entity()
        .with(Position {
            x: position.0,
            y: position.1,
        })
        .with(Renderable {
            glyph: to_cp437('M'),
            fg: RGB::named(BROWN1),
            bg: RGB::named(BLACK),
            render_order: 1,
        })
        .with(Fort { owner, defense: 10 })
        .with(Name { name })
        .build()
}

pub fn spawn_player_entities(
    ecs: &mut World,
    spawn_point: (i32, i32),
    range: i32,
    player_num: PlayerOrder,
) {
    let mut unit_counter = 0;
    {
        // Adding the player to the game using the spawn_point established outside this scope
        ecs.insert(Point::new(spawn_point.0, spawn_point.1));
        let player_entity = player(ecs, spawn_point, player_num);
        ecs.insert(player_entity);
    }

    {
        // Claiming the tiles surrounding the fort being placed down
        let mut map = ecs.fetch_mut::<Map>();
        let (low_x, low_y) = (spawn_point.0 - 1, spawn_point.1 - 1);
        let (high_x, high_y) = (spawn_point.0 + 1, spawn_point.1 + 1);
        for x in low_x..=high_x {
            for y in low_y..=high_y {
                let idx = xy_idx(x, y);
                map.claimed_tiles[idx] = player_num;
            }
        }
    }

    {
        // Building the fort
        let fort_entity = fort(ecs, spawn_point, "Fort1".to_string(), player_num);
        ecs.insert(fort_entity);
    }

    for _ in 0..3 {
        unit_counter += 1;
        let x_range = (spawn_point.0 - 2, spawn_point.0 + 2);
        let y_range = (spawn_point.1 - 2, spawn_point.1 + 2);
        let pos = generate_coordinates(ecs, x_range, y_range);
        let unit_entity = unit(ecs, pos, format!("Unit{}", unit_counter), range, player_num);
        ecs.insert(unit_entity);
    }

    {
        // Getting the player struct and updating the unit and fort counts
        let mut players = ecs.write_storage::<Player>();
        let entities = ecs.entities();

        for (player, _entity) in (&mut players, &entities).join() {
            if player.order == player_num {
                player.unit_count = unit_counter;
                player.fort_count = 1;
            }
        }
    }
}

pub fn generate_coordinates(ecs: &World, x_range: (i32, i32), y_range: (i32, i32)) -> (i32, i32) {
    let map = ecs.fetch::<Map>();
    let mut position: Option<(i32, i32)> = None;

    while position.is_none() {
        let mut rng = thread_rng();
        let x: i32 = rng.gen_range(x_range.0..x_range.1);
        let y: i32 = rng.gen_range(y_range.0..y_range.1);

        let idx = xy_idx(x, y);

        if !map.blocked[idx] {
            position = Some((x, y));
            break;
        }
    }

    position.unwrap()
}
