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
                log.entries
                    .push("Unit out of stamina, can't make more moves".to_string());
                log.message_type.push(MessageType::Error);
            }
        },
    }
}

/// Generate a key for a test, returns back the expected state after the command is run
pub fn generate_key(initial_state: RunState, ctx: &mut BTerm) -> ExpectedFuzzState {
    let mut rng = thread_rng();
    let key: i32 = rng.gen_range(0..8);
    // let gen_key: Option<VirtualKeyCode>;

    let mut expected_state = ExpectedFuzzState {
        first: initial_state,
        second: None,
        third: None,
    };

    // Yes, this is very dumb. I couldn't really do anything else for generating input since
    // I can't add derive traits on external enums. I generated the match statement using
    // a small rust prorgram that wrote this out to a file so I didn't have to write it manually
    let gen_key = if initial_state == RunState::ShowUnits || initial_state == RunState::ShowForts {
         match key {
            0 => VirtualKeyCode::Return,
            1 => VirtualKeyCode::Escape,
            2 => VirtualKeyCode::Up,
            _ => VirtualKeyCode::Down,
        }
    } else {
        match key {
            0 => VirtualKeyCode::W,
            1 => VirtualKeyCode::A,
            2 => VirtualKeyCode::S,
            3 => VirtualKeyCode::D,
            4 => VirtualKeyCode::I,
            5 => VirtualKeyCode::F,
            6 => VirtualKeyCode::G,
            _ => VirtualKeyCode::B,
        }
    };

    ctx.key = Some(gen_key);

    // These first two cases are for the fort/unit menus, it returns two different types
    // to signal those two are the acceptable states for the game to be in
    match initial_state {
        RunState::ShowUnits => {
            expected_state.first = RunState::ShowUnits;
            expected_state.second = Some(RunState::MoveUnit);
            expected_state.third = Some(RunState::MoveCursor);
        }
        RunState::ShowForts => {
            expected_state.first = RunState::ShowForts;
            expected_state.second = Some(RunState::SelectedFort);
            expected_state.third = Some(RunState::MoveCursor);
        }
        RunState::MoveUnit => {
            if key == 4 {
                // I
                expected_state.first = RunState::MoveCursor;
            }
        }
        RunState::SelectedFort => {
            if (key == 7) || (key == 4) {
                // B and I
                expected_state.first = RunState::MoveCursor;
            }
        }
        RunState::MoveCursor => {
            if key == 4 {
                // I
                expected_state.first = RunState::ShowUnits;
            } else if key == 5 {
                // F
                expected_state.first = RunState::ShowForts;
            }
        }
        _ => {
            expected_state.first = initial_state;
        }
    }

    expected_state
}
