//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use specs::prelude::*;
use bracket_lib::prelude::*;
use crate::{MAPHEIGHT, MAPWIDTH};


pub fn draw_ui(_ecs: &World, ctx: &mut BTerm) {
    let start_x = MAPWIDTH;
    let start_y = 0;
    let width = 19;
    let height = MAPHEIGHT - 1;

    // Draws the side box
    ctx.draw_box(start_x, start_y, width, height, RGB::named(WHITE), RGB::named(BLACK));   
}
