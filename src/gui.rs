//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use specs::prelude::*;
use bracket_lib::prelude::*;
use crate::{VIEW_HEIGHT, VIEW_WIDTH, Position, Player, Map, TileType, xy_idx, Unit, State, OwnedBy, Name};


pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let start_x = VIEW_WIDTH;
    let start_y = 0;
    let width = 19;
    let height = VIEW_HEIGHT - 1;

    // Draws the side box
    ctx.draw_box(start_x, start_y, width, height, RGB::named(WHITE), RGB::named(BLACK));   
    
    let position = ecs.read_storage::<Position>();
    let player = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let unit = ecs.read_storage::<Unit>();

    for (_player, pos) in (&player, &position).join() {
        let location = format!("Pos: ({}. {})", pos.x, pos.y);
        let tile = map.tiles[xy_idx(pos.x, pos.y)]; // COME BACK TO THIS
        
        let tile_str = match tile {
            TileType::Mountain => { "Mountain".to_string() },        
            TileType::Forest => { "Forest".to_string() },        
            TileType::Grasslands => {  "Grasslands".to_string() },        
            TileType::Coast => {  "Coast".to_string() },        
            TileType::Water => {  "Water".to_string() },        
            TileType::Ice => {  "Ice".to_string() },        
        };

        let terrain = format!("{}", tile_str);
        
        // Write out the tile type and the current position to the gui box
        ctx.print_color(start_x + 1, start_y + 1, RGB::named(YELLOW), RGB::named(BLACK), &location);
        ctx.print_color(start_x + 1, start_y + 2, RGB::named(YELLOW), RGB::named(BLACK), &terrain);

        for (unit, unit_pos) in (&unit, &position).join() {
            if (unit_pos.x == pos.x) &&
               (unit_pos.y == pos.y) {
                let unit_stats = format!("Hlth: {} Str: {}", unit.health, unit.strength);
                ctx.print_color(start_x + 1, start_y + 4, RGB::named(YELLOW), RGB::named(BLACK), unit_stats);
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum UnitMenuResult { Cancel, NoResponse, Selected }

pub fn show_units(gs: &mut State, ctx: &mut BTerm) -> UnitMenuResult {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let owner = gs.ecs.read_storage::<OwnedBy>();
    
    let unit_list = (&owner, &names).join().filter(|unit| unit.0.owner == *player_entity );
    let count = unit_list.count();

    let mut y = (25 - (count / 2)) as i32;
    let mut j = 0;

    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_color(18, y - 2, RGB::named(YELLOW), RGB::named(BLACK), "Unit List");
    ctx.print_color(18, y + count as i32 + 1, RGB::named(YELLOW), RGB::named(BLACK), "ESCAPE to cancel");

    for (_owned, name) in (&owner, &names).join().filter(|unit| unit.0.owner == *player_entity) {
        ctx.set(17, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('('));
        ctx.set(18, y, RGB::named(YELLOW), RGB::named(BLACK), 97 + j as FontCharType);
        ctx.set(19, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437(')'));
        
        ctx.print(21, y, &name.name.to_string());
        y += 1;
        j += 1;
    }
    
    match ctx.key {
        None => UnitMenuResult::NoResponse,
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { UnitMenuResult::Cancel }
                _ => UnitMenuResult::NoResponse
            }
        }
    }

}

