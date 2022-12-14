//! Made by: Thomas Gardner, 2022
//!
//! Most of the code for this comes froms the rust roguelike tutorial linked
//! below, and it was pulled from src/player.rs and tweaked to work for units
//! rather than for moving player units.
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use crate::spawner::*;
use crate::{
    error_handling, teleport_player, xy_idx, FailedMoveReason, Fort, GameLog, Map, MessageType,
    Moving, Player, PlayerOrder, Position, RunState, State, Unit, Viewshed, World,
};
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

/// Attempts to move a unit, checking if the tile the unit will end up on is blocked or not
fn try_move_unit(
    delta_x: i32,
    delta_y: i32,
    ecs: &mut World,
) -> Result<(i32, i32), FailedMoveReason> {
    let mut positions = ecs.write_storage::<Position>();
    let mut units = ecs.write_storage::<Unit>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let moving_marker = ecs.read_storage::<Moving>();
    let map = ecs.fetch::<Map>();

    if let Some((unit, pos, viewshed, _moving)) =
        (&mut units, &mut positions, &mut viewsheds, &moving_marker)
            .join()
            .next()
    {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if !map.blocked[destination_idx] && unit.stamina > 0 {
            let mut ppos = ecs.write_resource::<Point>();
            pos.x = min(map.width, max(0, pos.x + delta_x));
            pos.y = min(map.height, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;
            viewshed.dirty = true;
			unit.stamina -= 1;

            return Ok((pos.x, pos.y));
        } else if unit.stamina == 0 {
			return Err(FailedMoveReason::UnitOutOfMoves);
		}else {
            return Err(FailedMoveReason::TileBlocked);
        }
    }
    Err(FailedMoveReason::UnableToGrabEntity)
}

/// Used for removing the moving marker from a unit struct so it won't move the next time a unit gets moved
/// Returns a position if it was successful to teleport the player to the location of the recently unmarked unit
fn unmark_moving_unit(ecs: &mut World) -> Option<Position> {
    let entities = ecs.entities();
    let units = ecs.read_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();
    let mut moving_marker = ecs.write_storage::<Moving>();

    let mut curr_pos = None;

    for (entity, _unit, pos) in (&entities, &units, &positions).join() {
        match moving_marker.remove(entity) {
            None => {}
            Some(_move) => {
                curr_pos = Some(Position { x: pos.x, y: pos.y });
            }
        }
    }
    curr_pos
}

/// Lets the player move a unit around, claim a tile, build a fort, or exit back to cursor mode
pub fn unit_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
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
        None => return RunState::MoveUnit,
        Some(key) => match key {
            VirtualKeyCode::A => {
                let res = try_move_unit(-1, 0, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::D => {
                let res = try_move_unit(1, 0, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::W => {
                let res = try_move_unit(0, -1, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::S => {
                let res = try_move_unit(0, 1, &mut gs.ecs);
                error_handling::handle_move_result(&mut gs.ecs, res, gs.verbose);
            }
            VirtualKeyCode::G => {
                let claimed = claim_tile(&mut gs.ecs);
                {
                    let mut log = gs.ecs.fetch_mut::<GameLog>();

                    match claimed {
                        Some((x, y)) => {
                            log.entries
                                .push(format!("{} has claimed a tile at ({}, {})", order, x, y));
                            log.message_type.push(MessageType::Claim);
                        }
                        None => {
                            log.entries.push("Unable to claim tile".to_string());
                            log.message_type.push(MessageType::Error);
                        }
                    }
                }
            }
            VirtualKeyCode::B => {
                let new_fort_location = build_fort(&mut gs.ecs);
                {
                    let mut log = gs.ecs.fetch_mut::<GameLog>();

                    match new_fort_location {
                        Some((x, y)) => {
                            log.entries
                                .push(format!("{} has built a fort at ({}, {})", order, x, y));
                            log.message_type.push(MessageType::Build);
                        }
                        None => {
                            log.entries.push("Unable to build fort".to_string());
                            log.message_type.push(MessageType::Error);
                        }
                    }
                }
            }
            VirtualKeyCode::I => match unmark_moving_unit(&mut gs.ecs) {
                None => {
                    panic!("Failed to unmark moving unit")
                }
                Some(pos) => {
                    teleport_player(pos, &mut gs.ecs);
                    return RunState::MoveCursor;
                }
            },
            _ => {}
        },
    }
    RunState::MoveUnit
}

/// Grabs the currently moving unit and claims the tile it's currently on if it isn't claimed already
fn claim_tile(ecs: &mut World) -> Option<(i32, i32)> {
    let mut units = ecs.write_storage::<Unit>();
    let positions = ecs.read_storage::<Position>();
    let moving = ecs.read_storage::<Moving>();
    let players = ecs.read_storage::<Player>();
    let entities = ecs.entities();
    let mut map = ecs.fetch_mut::<Map>();
    let mut claim_pos: Option<(i32, i32)> = None;

		for (unit, pos, _move) in (&mut units, &positions, &moving).join() {
			if unit.stamina > 0 {
				for (_player_entity, player) in (&entities, &players).join() {
					let idx = xy_idx(pos.x, pos.y);
					if map.claimed_tiles[idx] == PlayerOrder::NoPlayer {
						map.claimed_tiles[idx] = player.order;
						claim_pos = Some((pos.x, pos.y));
					}
				}	
				unit.stamina -= 1;			
			}
		}
    claim_pos
}

/*
    Using scoping in this function to prevent errors from the borrow checker since I'm moving
    ecs into unit, and inserting the unit into the world. Got the idea from the rust
    roguelike tutorial.

    Section: User Interface; Notifying of Deaths
    Link: https://bfnightly.bracketproductions.com/rustbook/chapter_8.html
*/
/// Gets the curret location of a unit and if it's claimed by the current player, builds a fort there
fn build_fort(ecs: &mut World) -> Option<(i32, i32)> {
    let mut player_order: Option<PlayerOrder> = None;
    let mut new_fort_pos: Option<(i32, i32)> = None;

    {
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        let positions = ecs.read_storage::<Position>();
        let mut units = ecs.write_storage::<Unit>();
        let forts = ecs.read_storage::<Fort>();
        let moving_units = ecs.read_storage::<Moving>();
        let mut map = ecs.fetch_mut::<Map>();

        for (player, _entity) in (&players, &entities).join() {
            player_order = Some(player.order);
        }
        if let Some(ref owner) = player_order {
            for (unit, pos, _moving) in (&mut units, &positions, &moving_units).join() {
                let idx = xy_idx(pos.x, pos.y);
                let mut fort_at_pos = false;
				
				if unit.stamina > 4 {
					for (_fort, entity) in (&forts, &entities).join() {
						let entities_at_location = &map.tile_content[idx];

						for current_entity in entities_at_location.iter() {
							if *current_entity == entity {
								fort_at_pos = true;
							}
						}
					}

					if (map.claimed_tiles[idx] == *owner) && !fort_at_pos {
						new_fort_pos = Some((pos.x, pos.y));

						// Claiming the tiles surrounding this tile if a fort can be built here
						for x in pos.x - 1..=pos.x + 1 {
							for y in pos.y - 1..=pos.y + 1 {
								let idx = xy_idx(x, y);
								map.claimed_tiles[idx] = *owner;
							}
						}
					}
					unit.stamina = 0;
				} else {
					return None;
				}
            }
        }
    }

    if let Some(pos) = new_fort_pos {
        if let Some(player) = player_order {
            let mut fort_counter = 0;
            {
                let mut players = ecs.write_storage::<Player>();
                let entities = ecs.entities();

                for (player, _entity) in (&mut players, &entities).join() {
                    player.fort_count += 1;
                    fort_counter = player.fort_count;
                }
            }
            let name = format!("Fort{}", fort_counter);
            let fort = fort(ecs, pos, name, player);
            ecs.insert(fort);
        }
    }

    new_fort_pos
}
