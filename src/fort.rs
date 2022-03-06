//! Made by: Thomas Gardner, 2022
//!
//! The code for this comes from my work with src/player.rs and src/unit.rs, but adjusted for
//! for working with forts; contains controls for creating units through forts, and a function
//! to add the unit to the game
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::spawner::*;
use crate::{xy_idx, Fort, Map, Player, PlayerOrder, Position, RunState, Selected, State, World};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn build_unit(ecs: &mut World) -> RunState {
    let mut player_order: Option<PlayerOrder> = None;
    let mut new_unit_pos: Option<(i32, i32)> = None;

    /*
        Scoping this to prevent errors from the borrow checker since I'm moving ecs into unit,
        and inserting the unit into the world. Got the idea from the rust roguelike tutorial

        Section: User Interface; Notifying of Deaths
        Link: https://bfnightly.bracketproductions.com/rustbook/chapter_8.html
    */
    {
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        let positions = ecs.read_storage::<Position>();
        let forts = ecs.read_storage::<Fort>();
        let selects = ecs.read_storage::<Selected>();
        let map = ecs.fetch::<Map>();

        for (player, _entity) in (&players, &entities).join() {
            player_order = Some(player.order);
        }

        if let Some(ref owner) = player_order {
            for (fort, pos, _selected) in (&forts, &positions, &selects).join() {
                if !map.blocked[xy_idx(pos.x, pos.y)] && *owner == fort.owner {
                    new_unit_pos = Some((pos.x, pos.y));
                }
            }
        }
    }

    if let Some(pos) = new_unit_pos {
        if let Some(player) = player_order {
            let mut unit_counter = 0;
            {
                let mut players = ecs.write_storage::<Player>();
                let entities = ecs.entities();

                for (player, _entity) in (&mut players, &entities).join() {
                    player.unit_count += 1;
                    unit_counter = player.unit_count;
                }
            }
            let name = format!("Unit{}", unit_counter);
            let unit = unit(ecs, pos, name, 8, player);
            ecs.insert(unit);
        }
    }

    RunState::Paused
}

pub fn fort_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    // Unit actions
    match ctx.key {
        None => return RunState::SelectedFort, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::B => return build_unit(&mut gs.ecs), // Will be used for building a unit
            VirtualKeyCode::I => {
                return RunState::Paused;
            }
            _ => {}
        },
    }
    RunState::SelectedFort
}
