use bracket_lib::prelude::*;
use std::cmp::{max, min};
use specs::prelude::*;
use crate::{xy_idx, State, Map, Viewshed, World, Position, Unit, TileType};

// Doing this a dumb way, I copy pasted this from try_move_player, go back and fix this you idiot
pub fn try_move_unit(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut units = ecs.write_storage::<Unit>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_unit, pos, viewshed) in (&mut units, &mut positions, &mut viewsheds).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if (map.tiles[destination_idx] != TileType::Water) && 
		   (map.tiles[destination_idx] != TileType::Mountain) &&
		   (map.tiles[destination_idx] != TileType::Ice) {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map.width, max(0, pos.x + delta_x));
            pos.y = min(map.height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;

			viewshed.dirty = true;
        }
    }
}

pub fn unit_input(gs: &mut State, ctx: &mut BTerm) -> bool {
    // Player movement
    match ctx.key {
        None => { true }, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => { try_move_unit(-1, 0, &mut gs.ecs); true },
            VirtualKeyCode::D => { try_move_unit(1, 0, &mut gs.ecs); true },
            VirtualKeyCode::W => { try_move_unit(0, -1, &mut gs.ecs); true },
            VirtualKeyCode::S => { try_move_unit(0, 1, &mut gs.ecs); true },
            VirtualKeyCode::Q => { false },
            _ => { true }
        },
    }
}

