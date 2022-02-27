//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::{
    xy_idx, Map, Moving, Name, OwnedBy, Player, Position, State, TileType, Unit, VIEW_HEIGHT,
    VIEW_WIDTH,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let start_x = VIEW_WIDTH;
    let start_y = 0;
    let width = 19;
    let height = VIEW_HEIGHT - 1;

    // Draws the side box
    ctx.draw_box(
        start_x,
        start_y,
        width,
        height,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );

    let position = ecs.read_storage::<Position>();
    let player = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let unit = ecs.read_storage::<Unit>();

    for (_player, pos) in (&player, &position).join() {
        let location = format!("Pos: ({}. {})", pos.x, pos.y);
        let tile = map.tiles[xy_idx(pos.x, pos.y)]; // COME BACK TO THIS

        let tile_str = match tile {
            TileType::Mountain => "Mountain".to_string(),
            TileType::Forest => "Forest".to_string(),
            TileType::Grasslands => "Grasslands".to_string(),
            TileType::Coast => "Coast".to_string(),
            TileType::Water => "Water".to_string(),
            TileType::Ice => "Ice".to_string(),
        };

        // Write out the tile type and the current position to the gui box
        ctx.print_color(
            start_x + 1,
            start_y + 1,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            &location,
        );
        ctx.print_color(
            start_x + 1,
            start_y + 2,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            &tile_str.to_string(),
        );

        for (unit, unit_pos) in (&unit, &position).join() {
            if (unit_pos.x == pos.x) && (unit_pos.y == pos.y) {
                let unit_stats = format!("Hlth: {} Str: {}", unit.health, unit.strength);
                ctx.print_color(
                    start_x + 1,
                    start_y + 4,
                    RGB::named(YELLOW),
                    RGB::named(BLACK),
                    unit_stats,
                );
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum UnitMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

// Rename this function to be something more descriptive
pub fn show_units(gs: &mut State, ctx: &mut BTerm) -> UnitMenuResult {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let owner = gs.ecs.read_storage::<OwnedBy>();
    let entities = gs.ecs.entities();

    let unit_list = (&owner, &names)
        .join()
        .filter(|unit| unit.0.owner == *player_entity);
    let count = unit_list.count();

    let mut owned_units: Vec<Entity> = Vec::new();
    let mut y = (25 - (count / 2)) as i32;

    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "Unit List",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "ESCAPE to cancel",
    );

    for (i, (_owned, name, entity)) in (&owner, &names, &entities)
        .join()
        .filter(|unit| unit.0.owner == *player_entity)
        .enumerate()
    {
        ctx.set(17, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('('));
        ctx.set(
            18,
            y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            97 + i as FontCharType,
        );
        ctx.set(19, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        owned_units.push(entity);
        y += 1;
    }

    match ctx.key {
        None => UnitMenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => UnitMenuResult::Cancel,
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    let mut moving = gs.ecs.write_storage::<Moving>();
                    moving
                        .insert(owned_units[selection as usize], Moving {})
                        .expect("Unable to mark unit as moving");
                    return UnitMenuResult::Selected;
                }
                UnitMenuResult::NoResponse
            }
        },
    }
}
