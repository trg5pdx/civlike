//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use specs::prelude::*;
use bracket_lib::prelude::*;
use crate::{MAPHEIGHT, MAPWIDTH, Position, Player, TileType, xy_idx};


pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let start_x = MAPWIDTH;
    let start_y = 0;
    let width = 19;
    let height = MAPHEIGHT - 1;

    // Draws the side box
    ctx.draw_box(start_x, start_y, width, height, RGB::named(WHITE), RGB::named(BLACK));   
    
    let position = ecs.read_storage::<Position>();
    let player = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&player, &position).join() {
        let current = format!("Pos: ({}. {})", pos.x, pos.y);
        let tile = map[xy_idx(pos.x, pos.y)]; 
        
        // let mut tile_str = String::new(); 

        let tile_str = match tile {
            TileType::Mountain => { "Mountain".to_string() },        
            TileType::Forest => { "Forest".to_string() },        
            TileType::Grasslands => {  "Grasslands".to_string() },        
            TileType::Coast => {  "Coast".to_string() },        
            TileType::Water => {  "Water".to_string() },        
            TileType::Ice => {  "Ice".to_string() },        
        };

        let current2 = format!("{}", tile_str);

        ctx.print_color(start_x + 1, start_y + 1, RGB::named(YELLOW), RGB::named(BLACK), &current);
        ctx.print_color(start_x + 1, start_y + 2, RGB::named(YELLOW), RGB::named(BLACK), &current2);

    }


}
