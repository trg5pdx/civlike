//! Made by: Thomas Gardner, 2022
//!
//! The code for this comes from my work with src/player.rs and src/unit.rs, but adjusted for
//! for working with forts; contains controls for creating units through forts, and a function
//! to add the unit to the game
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::spawner::*;
use crate::{
    teleport_player, xy_idx, Fort, GameLog, Map, MessageType, Player, PlayerOrder, Position,
    RunState, Selected, State, World,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
    Using scoping in this function to prevent errors from the borrow checker since I'm moving
    ecs into unit, and inserting the unit into the world. Got the idea from the rust
    roguelike tutorial.

    Section: User Interface; Notifying of Deaths
    Link: https://bfnightly.bracketproductions.com/rustbook/chapter_8.html
*/
/// Builds a unit at the current fort is a unit isn't already present
fn build_unit(ecs: &mut World) -> Option<(i32, i32)> {
    let mut player_order: Option<PlayerOrder> = None;
    let mut new_unit_pos: Option<(i32, i32)> = None;

    {
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        let positions = ecs.read_storage::<Position>();
        let forts = ecs.read_storage::<Fort>();
        let selects = ecs.read_storage::<Selected>();
        let map = ecs.fetch::<Map>();

        for (player, _entity) in (&players, &entities).join() {
            player_order = Some(player.order);
            if player.unit_count > 25 {
                return None;
            }
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

    new_unit_pos
}

/// Used for removing the moving marker from a unit struct so it won't move the next time a unit gets moved
/// Returns a position if it was successful to teleport the player to the location of the recently unmarked unit
fn unmark_selected_fort(ecs: &mut World) -> Option<Position> {
    let entities = ecs.entities();
    let forts = ecs.read_storage::<Fort>();
    let positions = ecs.read_storage::<Position>();
    let mut selected_marker = ecs.write_storage::<Selected>();

    let mut curr_pos = None;

    for (entity, _fort, pos) in (&entities, &forts, &positions).join() {
        match selected_marker.remove(entity) {
            None => {}
            Some(_select) => {
                curr_pos = Some(Position { x: pos.x, y: pos.y });
            }
        }
    }
    curr_pos
}

/// Lets the player build a unit or exit back to cursor mode
pub fn fort_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    let mut order = String::new();
    {
        let entities = gs.ecs.entities();
        let players = gs.ecs.read_storage::<Player>();

        for (player, _entity) in (&players, &entities).join() {
            match player.order {
                PlayerOrder::NoPlayer => order = "No Player".to_string(),
                PlayerOrder::PlayerOne => order = "Player1".to_string(),
                PlayerOrder::PlayerTwo => order = "Player2".to_string(),
            }
        }
    }

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::B => {
                let new_unit_location = build_unit(&mut gs.ecs);

                {
                    let mut log = gs.ecs.fetch_mut::<GameLog>();
                    match new_unit_location {
                        Some((x, y)) => {
                            log.entries
                                .push(format!("{} has built a unit at ({}, {})", order, x, y));
                            log.message_type.push(MessageType::Build);
                        }
                        None => {
                            log.entries.push("Unable to build a unit".to_string());
                            log.message_type.push(MessageType::Error);
                        }
                    }
                }

                match unmark_selected_fort(&mut gs.ecs) {
                    None => {
                        panic!("ERROR: Failed to unmark selected fort")
                    }
                    Some(pos) => {
                        teleport_player(pos, &mut gs.ecs);
                    }
                }
                return RunState::MoveCursor;
            }
            VirtualKeyCode::I => match unmark_selected_fort(&mut gs.ecs) {
                None => {
                    panic!("ERROR: Failed to unmark selected fort")
                }
                Some(pos) => {
                    teleport_player(pos, &mut gs.ecs);
                    return RunState::MoveCursor;
                }
            },
            _ => {}
        },
    }
    RunState::SelectedFort
}
