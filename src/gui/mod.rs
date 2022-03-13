//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html
//!
//! This submodule is for handling the fuctions of drawing the gui, with there being two seperate
//! files for handling drawing the gui for the fort and unit menus

use crate::PlayerOrder;
use crate::{
    xy_idx, Fort, GameLog, Map, MessageType, Moving, Name, Player, Position, TileType, Unit,
    VIEW_HEIGHT, VIEW_WIDTH,
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
    draw_sidebar(ecs, ctx);
    draw_message_box(ecs, ctx);
}

fn draw_sidebar(ecs: &World, ctx: &mut BTerm) {
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
    let units = ecs.read_storage::<Unit>();
    let moving = ecs.read_storage::<Moving>();
    let mut pos: Position;

    for (_player, cursor_pos) in (&player, &position).join() {
        let location;
        let tile;
        let controlled;
        pos = *cursor_pos;

        for (_mover, _unit, unit_pos) in (&moving, &units, &position).join() {
            pos = *unit_pos;
        }

        location = format!("Pos: ({}, {})", pos.x, pos.y);
        tile = &map.tiles[xy_idx(pos.x, pos.y)];
        controlled = &map.claimed_tiles[xy_idx(pos.x, pos.y)];

        let tile_str = match tile {
            TileType::Mountain => "Mountain".to_string(),
            TileType::Forest => "Forest".to_string(),
            TileType::Grasslands => "Grasslands".to_string(),
            TileType::Coast => "Coast".to_string(),
            TileType::Water => "Water".to_string(),
            TileType::Ice => "Ice".to_string(),
        };

        let claims = match controlled {
            PlayerOrder::NoPlayer => "Unclaimed".to_string(),
            PlayerOrder::PlayerOne => "Owner: Player1".to_string(),
            PlayerOrder::PlayerTwo => "Owner: Player2".to_string(),
        };

        // Write out the tile type and the current position to the gui box
        ctx.print_color(x + 1, y + 1, RGB::named(YELLOW), bg, &location);
        ctx.print_color(x + 1, y + 2, RGB::named(GREEN), bg, &tile_str.to_string());
        ctx.print_color(x + 1, y + 3, RGB::named(ORANGE), bg, &claims);

        display_unit_info(ecs, ctx, x, y, pos, bg);
        display_fort_info(ecs, ctx, x, y, pos, bg);
    }
}

fn display_unit_info(
    ecs: &World,
    ctx: &mut BTerm,
    x: usize,
    y: usize,
    cursor_pos: Position,
    bg: RGB,
) {
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();

    for (unit, unit_pos, unit_name) in (&units, &positions, &names).join() {
        if (unit_pos.x == cursor_pos.x) && (unit_pos.y == cursor_pos.y) {
            ctx.print_color(
                x + 1,
                y + 46,
                RGB::named(CYAN),
                bg,
                format!("{} stats:", unit_name.name),
            );
            let unit_stats = format!("Hlth: {} Str: {}", unit.health, unit.strength);
            ctx.print_color(x + 1, y + 48, RGB::named(CYAN), bg, unit_stats);
        }
    }
}

fn display_fort_info(
    ecs: &World,
    ctx: &mut BTerm,
    x: usize,
    y: usize,
    cursor_pos: Position,
    bg: RGB,
) {
    let forts = ecs.read_storage::<Fort>();
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();

    for (fort, fort_pos, fort_name) in (&forts, &positions, &names).join() {
        if (fort_pos.x == cursor_pos.x) && (fort_pos.y == cursor_pos.y) {
            let fort_info;
            match fort.owner {
                PlayerOrder::NoPlayer => fort_info = "Not Owned".to_string(),
                PlayerOrder::PlayerOne => fort_info = "Player1's Fort".to_string(),
                PlayerOrder::PlayerTwo => fort_info = "Player2's Fort".to_string(),
            }
            ctx.print_color(x + 1, y + 6, RGB::named(WHITE), bg, fort_info);

            let fort_option_name = format!("Fort name: {}", fort_name.name);
            ctx.print_color(x + 1, y + 7, RGB::named(WHITE), bg, fort_option_name);
        }
    }
}

fn draw_message_box(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(
        0,
        VIEW_HEIGHT,
        VIEW_WIDTH - 1,
        9,
        RGB::named(WHITE),
        RGB::named(BLACK),
    );

    ctx.print_color(
        2,
        VIEW_HEIGHT,
        RGB::named(WHITE),
        RGB::named(BLACK),
        "[Message Log]".to_string(),
    );

    let log = ecs.fetch::<GameLog>();
    let mut y = VIEW_HEIGHT + 1;
    for (message, message_type) in log.entries.iter().rev().zip(log.message_type.iter().rev()) {
        if y < 49 {
            let fg = match message_type {
                MessageType::Build => RGB::named(YELLOW),
                MessageType::Claim => RGB::named(SEAGREEN),
                MessageType::Move => RGB::named(LIGHTBLUE),
                MessageType::Error => RGB::named(SALMON),
                MessageType::Other => RGB::named(WHITE),
            };
            ctx.print_color(2, y, fg, RGB::named(BLACK), message);
        }
        y += 1;
    }
}
