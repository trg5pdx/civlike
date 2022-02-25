//! Made by: Thomas Gardner, 2022
//!
//! Most of the code for this comes froms the rust roguelike tutorial linked below, and it was pulled from src/player.rs
//! and tweaked to work for units rather than for moving player units
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use bracket_lib::prelude::*;
use std::cmp::{max, min};
use specs::prelude::*;
use crate::{xy_idx, State, Map, Viewshed, World, Position, Unit, TileType, RunState, OwnedBy, UnitControl, Name};

// Doing this a dumb way, I copy pasted this from try_move_player, go back and fix this you idiot
pub fn try_move_unit(delta_x: i32, delta_y: i32, sel_unit_name: &String, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut units = ecs.write_storage::<Unit>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let names = ecs.read_storage::<Name>();
    let map = ecs.fetch::<Map>();
    
    for (_moving_unit, pos, viewshed, name) in (&mut units, &mut positions, &mut viewsheds, &names).join() {
        if &name.name == sel_unit_name {
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
}

pub fn unit_input(gs: &mut State, unit: &String, ctx: &mut BTerm) -> RunState {
    // Unit actions
    match ctx.key {
        None => { return RunState::MoveUnit }, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => { try_move_unit(-1, 0, unit, &mut gs.ecs) },
            VirtualKeyCode::D => { try_move_unit(1, 0, unit, &mut gs.ecs) },
            VirtualKeyCode::W => { try_move_unit(0, -1, unit, &mut gs.ecs) },
            VirtualKeyCode::S => { try_move_unit(0, 1, unit, &mut gs.ecs) },
            VirtualKeyCode::C => { return RunState::Paused }, // Need to come back and teleport cursor to unit's last location
            _ => {}
        },
    }
    RunState::MoveUnit
}

pub struct UnitOwnershipSystem {}

// Using the roguelike tutorial section on the inventory system as a guide for setting
// up the ability for players to control multiple units
impl<'a> System<'a> for UnitOwnershipSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteStorage<'a, UnitControl>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, OwnedBy>
                      );
    fn run(&mut self, data: Self::SystemData) {
        let (_player_entity, mut unit_control, _positions, mut owner) = data;
        
        for owned in unit_control.join() {
            owner.insert(owned.unit, OwnedBy{ owner: owned.owned_by }).expect("unable to own");
        }

        unit_control.clear();
    }
}

pub fn get_unit(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();

    let mut target_unit: Option<Entity> = None;

    for (unit_entity, _unit, position) in (&entities, &units, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_unit = Some(unit_entity);
        }
    }

    match target_unit {
        None => println!("no unit"),
        Some(unit) => {
            let mut controlled_by = ecs.write_storage::<UnitControl>();
            controlled_by.insert(*player_entity, UnitControl { owned_by: *player_entity, unit}).expect("Unable to add unit");
        }
    }
}
