//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::PlayerOrder;
use crate::{
    xy_idx, Fort, GameLog, Map, Moving, Name, Player, Position, TileType, Unit, VIEW_HEIGHT,
    VIEW_WIDTH,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub mod fort;
pub mod unit;

#[derive(PartialEq, Copy, Clone)]
pub enum MenuResult {
    Cancel,
    NoResponse,
    Selected,
}

/*
The draw_ui function was provided by the rust roguelike tutorial, with it being tweaked for my usage.
Link to the tutorial at the top of this file and in the README
*/
pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let x = VIEW_WIDTH;
    let y = 0;
    let width = 19;
    let height = VIEW_HEIGHT + 9;
    let bg = RGB::named(BLACK);

    // Draws the side box
    ctx.draw_box(x, y, width, height, RGB::named(WHITE), bg);

    let position = ecs.read_storage::<Position>();
    let player = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let forts = ecs.read_storage::<Fort>();
    let units = ecs.read_storage::<Unit>();
    let moving = ecs.read_storage::<Moving>();
    let names = ecs.read_storage::<Name>();

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

        // Need to come back to this, the compiler wanted me to make the enum snake case but here
        // it wanted it to be camel case, and they complained about not using them
        let claims = match controlled {
            PlayerOrder::NoPlayer => "Unclaimed".to_string(),
            PlayerOrder::PlayerOne => "Owner: Player1".to_string(),
            PlayerOrder::PlayerTwo => "Owner: Player2".to_string(),
        };

        // Write out the tile type and the current position to the gui box
        ctx.print_color(x + 1, y + 1, RGB::named(YELLOW), bg, &location);
        ctx.print_color(x + 1, y + 2, RGB::named(YELLOW), bg, &tile_str.to_string());
        ctx.print_color(x + 1, y + 3, RGB::named(YELLOW), bg, &claims);

        for (unit, unit_pos) in (&units, &position).join() {
            if (unit_pos.x == cursor_pos.x) && (unit_pos.y == cursor_pos.y) {
                let unit_stats = format!("Hlth: {} Str: {}", unit.health, unit.strength);
                ctx.print_color(x + 1, y + 4, RGB::named(YELLOW), bg, unit_stats);
            }
        }
        for (fort, fort_pos, fort_name) in (&forts, &position, &names).join() {
            if (fort_pos.x == cursor_pos.x) && (fort_pos.y == cursor_pos.y) {
                let fort_info;
                match fort.owner {
                    PlayerOrder::NoPlayer => fort_info = "Not Owned".to_string(),
                    PlayerOrder::PlayerOne => fort_info = "Player1's Fort".to_string(),
                    PlayerOrder::PlayerTwo => fort_info = "Player2's Fort".to_string(),
                }
                ctx.print_color(x + 1, y + 5, RGB::named(YELLOW), bg, fort_info);

                let fort_option_name = format!("Fort name: {}", fort_name.name);
                ctx.print_color(x + 1, y + 6, RGB::named(YELLOW), bg, fort_option_name);
            }
        }
    }

    {
        ctx.draw_box(0, VIEW_HEIGHT, VIEW_WIDTH - 1, 9, RGB::named(WHITE), bg);

        let log = ecs.fetch::<GameLog>();
        let mut y = VIEW_HEIGHT + 1;
        for s in log.entries.iter().rev() {
            if y < 49 {
                ctx.print(2, y, s)
            }
            y += 1;
        }
    }
}
