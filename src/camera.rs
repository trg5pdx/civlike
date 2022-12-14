//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. This code was pulled from section 4.19: Decouple map size
//! from screen size.
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_41.html

use crate::{xy_idx, Fort, Map, PlayerOrder, Position, Renderable, TileType};
use bracket_lib::prelude::*;
use specs::prelude::*;

const SHOW_BOUNDARIES: bool = true;

pub fn render_camera(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    let player_pos = ecs.fetch::<Point>();
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let (x_chars, y_chars) = ctx.get_char_size();

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;

    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    for (y, ty) in (min_y..max_y).enumerate() {
        for (x, tx) in (min_x..max_x).enumerate() {
            if tx >= 0 && tx < map.width && ty >= 0 && ty < map.height {
                let idx = xy_idx(tx, ty);
                if map.revealed_tiles[idx] {
                    let (glyph, fg, bg) = get_tile_glyph(idx, &*map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            } else if SHOW_BOUNDARIES {
                ctx.set(x, y, RGB::named(GRAY), RGB::named(BLACK), to_cp437(','));
            }
        }
    }

    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
    for (pos, render) in data.iter() {
        let idx = xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            let entity_screen_x = pos.x - min_x;
            let entity_screen_y = pos.y - min_y;

            if entity_screen_x >= 0
                && entity_screen_x < map.width
                && entity_screen_y >= 0
                && entity_screen_y < map.height
            {
                ctx.set(
                    entity_screen_x,
                    entity_screen_y,
                    render.fg,
                    render.bg,
                    render.glyph,
                );
            }
        }
        if map.revealed_tiles[idx] {
            // Used for rendering forts in revealed tiles
            let entities = ecs.entities();
            let forts = ecs.read_storage::<Fort>();
            for (_fort, _entity) in (&forts, &entities).join() {
                let entity_screen_x = pos.x - min_x;
                let entity_screen_y = pos.y - min_y;

                if entity_screen_x >= 0
                    && entity_screen_x < map.width
                    && entity_screen_y >= 0
                    && entity_screen_y < map.height
                {
                    ctx.set(
                        entity_screen_x,
                        entity_screen_y,
                        render.fg,
                        render.bg,
                        render.glyph,
                    );
                }
            }
        }
    }
}

fn get_tile_glyph(idx: usize, map: &Map) -> (FontCharType, RGB, RGB) {
    let glyph;
    let mut fg;

    let bg = match map.claimed_tiles[idx] {
        PlayerOrder::NoPlayer => RGB::named(BLACK),
        PlayerOrder::PlayerOne => RGB::named(PINK),
        PlayerOrder::PlayerTwo => RGB::named(RED),
    };

    match map.tiles[idx] {
        TileType::Mountain => {
            fg = RGB::named(GREY);
            glyph = to_cp437('A');
        }
        TileType::Forest => {
            fg = RGB::named(DARKGREEN);
            glyph = to_cp437('t');
        }
        TileType::Grasslands => {
            fg = RGB::named(GREEN);
            glyph = to_cp437('w');
        }
        TileType::Coast => {
            fg = RGB::named(YELLOW);
            glyph = to_cp437('s');
        }
        TileType::Water => {
            fg = RGB::named(BLUE);
            glyph = to_cp437('~');
        }
        TileType::Ice => {
            fg = RGB::named(WHITE);
            glyph = to_cp437('#');
        }
    }
    if !map.visible_tiles[idx] {
        fg = fg.to_greyscale()
    }

    (glyph, fg, bg)
}
