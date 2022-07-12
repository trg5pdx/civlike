//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. This submodule is for handling the fuctions of drawing the
//! gui, with there being two seperate files for handling drawing the gui for
//! the fort and unit menus
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::PlayerOrder;
use crate::{
    xy_idx, Fort, GameLog, Map, MessageType, Moving, Name, Player, Position, State, TileType, Unit,
    VIEW_HEIGHT, VIEW_WIDTH,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub mod fort;
pub mod unit;
pub mod window;

#[derive(PartialEq, Copy, Clone)]
pub enum MenuResult {
    Cancel,
    NoResponse,
    Selected,
}

// Not that useful atm, will be better when more types of units/forts exist
#[derive(PartialEq, Copy, Clone)]
pub enum SelectionType {
    Unit,
    Fort,
}

pub fn draw_ui(ecs: &World, ctx: &mut BTerm, turns: u32) {
    draw_sidebar(ecs, ctx, turns);
    draw_message_box(ecs, ctx);
}

pub fn select_player(ecs: &World) -> Option<PlayerOrder> {
    let players = ecs.read_storage::<Player>();
    let entities = ecs.entities();

    let mut player_enum: Option<PlayerOrder> = None;

    for (_entity, player) in (&entities, &players).join() {
        player_enum = Some(player.order);
    }

    player_enum
}

fn draw_sidebar(ecs: &World, ctx: &mut BTerm, turns: u32) {
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

        let tile_str = match tile.0 {
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
            y + 4,
            RGB::named(CYAN),
            bg,
            format!("Food: {}", tile.1.food),
        );
        ctx.print_color(
            x + 1,
            y + 5,
            RGB::named(CYAN),
            bg,
            format!("Production: {}", tile.1.prod),
        );
        ctx.print_color(
            x + 1,
            y + 6,
            RGB::named(CYAN),
            bg,
            format!("Gold: {}", tile.1.gold),
        );
        ctx.print_color(
            x + 1,
            y + 7,
            RGB::named(VIOLET),
            bg,
            format!("Current Turn: {}", turns),
        );

        ctx.print_color(
            x + 1,
            y + 8,
            RGB::named(WHITE),
            bg,
            "You have: ".to_string(),
        );
        ctx.print_color(
            x + 1,
            y + 9,
            RGB::named(CYAN),
            bg,
            format!("{} units", player.unit_count),
        );
        ctx.print_color(
            x + 1,
            y + 10,
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

fn draw_selection_box(ctx: &mut BTerm, title: String) {
    let y = 15;
    let height = 10;
    let y_cord = y - 2;
    let bg = RGB::named(BLACK);

    ctx.draw_box(14, y - 2, 30, height, RGB::named(WHITE), bg);
    ctx.print_color(18, y_cord, RGB::named(YELLOW), bg, title);
    ctx.print_color(
        18,
        y_cord + height,
        RGB::named(YELLOW),
        bg,
        "ESCAPE to cancel",
    );
}

fn draw_selection_options(gs: &mut State, ctx: &mut BTerm, selection_list: &Vec<(Entity, String)>) {
    let y = 15;
    let bg = RGB::named(BLACK);

    let count = selection_list.len() as u32;
    let current_option = gs.last_option;
    let mut offset = 0;
    if count > 7 {
        offset = count - 7;
    }
    for i in 0..7 {
        let mut index = current_option + i;
        if current_option > offset {
            index = offset + i;
        }
        let width = index.to_string().len();
        if index < count as u32 {
            ctx.set(17, y + i, RGB::named(WHITE), bg, to_cp437('('));
            ctx.print_color(18, y + i, RGB::named(YELLOW), bg, format!("{}", index + 1));

            ctx.set(18 + width, y + i, RGB::named(WHITE), bg, to_cp437(')'));
            ctx.print(20 + width, y + i, selection_list[index as usize].1.clone());
        }
        if index == current_option {
            ctx.set(16, y + i, RGB::named(WHITE), bg, to_cp437('>'));
        }
    }
}
