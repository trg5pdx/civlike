use crate::{ExpectedFuzzState, FailedMoveReason, GameLog, MessageType, RunState};
use bracket_lib::prelude::*;
use rand::{thread_rng, Rng};
use specs::prelude::*;

pub fn handle_move_result(
    ecs: &mut World,
    res: Result<(i32, i32), FailedMoveReason>,
    verbose: bool,
) {
    let mut log = ecs.fetch_mut::<GameLog>();
    match res {
        Ok((x, y)) => {
            if verbose {
                log.entries.push(format!("Moved entity to ({}, {})", x, y));
                log.message_type.push(MessageType::Move);
            }
        }
        Err(e) => match e {
            FailedMoveReason::TileBlocked => {
                log.entries
                    .push("Tile entity tried to move on is blocked".to_string());
                log.message_type.push(MessageType::Error);
            }
            FailedMoveReason::UnableToGrabEntity => {
                log.entries.push("Failed to grab entity".to_string());
                log.message_type.push(MessageType::Error);
            }
            FailedMoveReason::UnitOutOfMoves => {
                log.entries.push("Unit out of stamina, can't make more moves".to_string());
                log.message_type.push(MessageType::Error);
            }	
        },
    }
}

/// Generate a key for a test, returns back the expected state after the command is run
pub fn generate_key(initial_state: RunState, ctx: &mut BTerm) -> ExpectedFuzzState {
    let mut rng = thread_rng();
    let key: i32 = rng.gen_range(0..27);

    let mut expected_state = ExpectedFuzzState {
        first: initial_state,
        second: None,
    };

    // Yes, this is very dumb. I couldn't really do anything else for generating input since
    // I can't add derive traits on external enums. I generated the match statement using
    // a small rust prorgram that wrote this out to a file so I didn't have to write it manually
    let gen_key = match key {
        0 => VirtualKeyCode::A,
        1 => VirtualKeyCode::B,
        2 => VirtualKeyCode::C,
        3 => VirtualKeyCode::D,
        4 => VirtualKeyCode::E,
        5 => VirtualKeyCode::F,
        6 => VirtualKeyCode::G,
        7 => VirtualKeyCode::H,
        8 => VirtualKeyCode::I,
        9 => VirtualKeyCode::J,
        10 => VirtualKeyCode::K,
        11 => VirtualKeyCode::L,
        12 => VirtualKeyCode::M,
        13 => VirtualKeyCode::N,
        14 => VirtualKeyCode::O,
        15 => VirtualKeyCode::P,
        16 => VirtualKeyCode::Q,
        17 => VirtualKeyCode::R,
        18 => VirtualKeyCode::S,
        19 => VirtualKeyCode::T,
        20 => VirtualKeyCode::U,
        21 => VirtualKeyCode::V,
        22 => VirtualKeyCode::W,
        23 => VirtualKeyCode::X,
        24 => VirtualKeyCode::Y,
        25 => VirtualKeyCode::Z,
        _ => VirtualKeyCode::Escape,
    };

    ctx.key = Some(gen_key);

    // These first two cases are for the fort/unit menus, it returns two different types
    // to signal those two are the acceptable states for the game to be in
    match initial_state {
        RunState::ShowUnits => {
            expected_state.first = RunState::ShowUnits;
            expected_state.second = Some(RunState::MoveUnit);
        }
        RunState::ShowForts => {
            expected_state.first = RunState::ShowForts;
            expected_state.second = Some(RunState::SelectedFort);
        }
        RunState::MoveUnit => {
            if key == 8 {
                expected_state.first = RunState::MoveCursor;
            }
        }
        RunState::SelectedFort => {
            if (key == 1) || (key == 8) {
                expected_state.first = RunState::MoveCursor;
            }
        }
        RunState::MoveCursor => {
            if key == 8 {
                expected_state.first = RunState::ShowUnits;
            } else if key == 5 {
                expected_state.first = RunState::ShowForts;
            }
        }
		_ => { expected_state.first = initial_state; }
    }

    expected_state
}
