//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::PlayerOrder;
use crate::{
    xy_idx, Fort, Map, Moving, Name, Player, Position, Selected, State, TileType, Unit,
    VIEW_HEIGHT, VIEW_WIDTH,
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
    let units = ecs.read_storage::<Unit>();
    let moving = ecs.read_storage::<Moving>();

    for (_player, cursor_pos) in (&player, &position).join() {
        let mut pos: Option<Position> = None;
        let location;
        let tile;
        let controlled;

        for (_mover, _unit, unit_pos) in (&moving, &units, &position).join() {
            pos = Some(*unit_pos);
        }

        if let Some(unit_pos) = pos {
            location = format!("Pos: ({}, {})", unit_pos.x, unit_pos.y);
            tile = map.tiles[xy_idx(unit_pos.x, unit_pos.y)]; // COME BACK TO THIS
            controlled = &map.claimed_tiles[xy_idx(unit_pos.x, unit_pos.y)];
        } else {
            location = format!("Pos: ({}. {})", cursor_pos.x, cursor_pos.y);
            tile = map.tiles[xy_idx(cursor_pos.x, cursor_pos.y)]; // COME BACK TO THIS
            controlled = &map.claimed_tiles[xy_idx(cursor_pos.x, cursor_pos.y)];
        }

        let tile_str = match tile {
            TileType::Mountain => "Mountain".to_string(),
            TileType::Forest => "Forest".to_string(),
            TileType::Grasslands => "Grasslands".to_string(),
            TileType::Coast => "Coast".to_string(),
            TileType::Water => "Water".to_string(),
            TileType::Ice => "Ice".to_string(),
        };

        // Need to come back to this, the compiler wanted me ot make the enum snake case but here
        // it wanted it to be camel case, and they complained about not using them
        let claims = match controlled {
            PlayerOrder::NoPlayer => "Unclaimed".to_string(),
            PlayerOrder::PlayerOne => "Owner: Player1".to_string(),
            PlayerOrder::PlayerTwo => "Owner: Player2".to_string(),
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
        ctx.print_color(
            start_x + 1,
            start_y + 3,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            &claims,
        );

        for (unit, unit_pos) in (&units, &position).join() {
            if (unit_pos.x == cursor_pos.x) && (unit_pos.y == cursor_pos.y) {
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
pub enum MenuResult {
    Cancel,
    NoResponse,
    Selected,
}

/// Used for printing out a list of the units a player currently has and is able to move
pub fn unit_list(gs: &mut State, ctx: &mut BTerm) -> MenuResult {
    let players = gs.ecs.read_storage::<Player>();
    let units = gs.ecs.read_storage::<Unit>();
    let names = gs.ecs.read_storage::<Name>();
    let entities = gs.ecs.entities();

    let mut player_enum: Option<PlayerOrder> = None;

    for (_entity, player) in (&entities, &players).join() {
        player_enum = Some(player.order);
    }

    let player_enum = player_enum.unwrap();

    let unit_list = (&units, &entities)
        .join()
        .filter(|unit| unit.0.owner == player_enum);
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

    for (i, (_unit, name, entity)) in (&units, &names, &entities)
        .join()
        .filter(|unit| unit.0.owner == player_enum)
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
        None => MenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => MenuResult::Cancel,
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    let mut moving = gs.ecs.write_storage::<Moving>();
                    moving
                        .insert(owned_units[selection as usize], Moving {})
                        .expect("Unable to mark unit as moving");
                    return MenuResult::Selected;
                }
                MenuResult::NoResponse
            }
        },
    }
}

/// Used for printing out a list of the units a player currently has and is able to move
pub fn fort_list(gs: &mut State, ctx: &mut BTerm) -> MenuResult {
    let players = gs.ecs.read_storage::<Player>();
    let forts = gs.ecs.read_storage::<Fort>();
    let names = gs.ecs.read_storage::<Name>();
    let entities = gs.ecs.entities();

    let mut player_enum: Option<PlayerOrder> = None;

    for (_entity, player) in (&entities, &players).join() {
        player_enum = Some(player.order);
    }

    let player_enum = player_enum.unwrap();

    let fort_list = (&forts, &entities)
        .join()
        .filter(|fort| fort.0.owner == player_enum);
    let count = fort_list.count();

    let mut player_forts: Vec<Entity> = Vec::new();
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
        "Fort List",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "ESCAPE to cancel",
    );

    for (i, (_fort, name, entity)) in (&forts, &names, &entities)
        .join()
        .filter(|unit| unit.0.owner == player_enum)
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
        player_forts.push(entity);
        y += 1;
    }

    match ctx.key {
        None => MenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => MenuResult::Cancel,
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    let mut selected = gs.ecs.write_storage::<Selected>();
                    selected
                        .insert(player_forts[selection as usize], Selected {})
                        .expect("Unable to mark unit as moving");
                    return MenuResult::Selected;
                }
                MenuResult::NoResponse
            }
        },
    }
}
