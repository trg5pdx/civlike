//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. This submodule is for handling the fuctions of drawing the 
//! gui, with there being two seperate files for handling drawing the gui for 
//! the fort and unit menus
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

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

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    draw_sidebar(ecs, ctx);
    draw_message_box(ecs, ctx);
}

fn draw_sidebar(ecs: &World, ctx: &mut BTerm) {
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let units = ecs.read_storage::<Unit>();
    let moving = ecs.read_storage::<Moving>();

    let x = VIEW_WIDTH;
    let y = 0;
    let width = 19;
    let height = VIEW_HEIGHT + 9;
    let bg = RGB::named(BLACK);
    let mut pos: Position;

    ctx.draw_box(x, y, width, height, RGB::named(WHITE), bg);

    for (player, cursor_pos) in (&players, &positions).join() {
        pos = *cursor_pos;

        // Grabbing the position of the unit in the case that theres a moving unit
        // So the units information is printed instead of the cursors position
        for (_mover, _unit, unit_pos) in (&moving, &units, &positions).join() {
            pos = *unit_pos;
        }

        let location = format!("Pos: ({}, {})", pos.x, pos.y);
        let tile = &map.tiles[xy_idx(pos.x, pos.y)];
        let controlled = &map.claimed_tiles[xy_idx(pos.x, pos.y)];

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

        ctx.print_color(
            x + 1,
            y + 5,
            RGB::named(WHITE),
            bg,
            "You have: ".to_string(),
        );
        ctx.print_color(
            x + 1,
            y + 6,
            RGB::named(CYAN),
            bg,
            format!("{} units", player.unit_count),
        );
        ctx.print_color(
            x + 1,
            y + 7,
            RGB::named(BURLYWOOD3),
            bg,
            format!("{} forts", player.fort_count),
        );

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
            let fort_info = match fort.owner {
                PlayerOrder::NoPlayer => "Not Owned".to_string(),
                PlayerOrder::PlayerOne => "Player1's Fort".to_string(),
                PlayerOrder::PlayerTwo => "Player2's Fort".to_string(),
            };
            ctx.print_color(x + 1, y + 9, RGB::named(BURLYWOOD3), bg, fort_info);

            let fort_option_name = format!("Fort name: {}", fort_name.name);
            ctx.print_color(x + 1, y + 10, RGB::named(BURLYWOOD3), bg, fort_option_name);
        }
    }
}

// The code for this came from section 2.7: User Interface
// Link: https://bfnightly.bracketproductions.com/rustbook/chapter_8.html#adding-a-message-log
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
