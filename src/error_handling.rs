use specs::prelude::*;
use bracket_lib::prelude::*;
use rand::{thread_rng, Rng};
use crate::{GameLog, FailedMoveReason, RunState};

pub fn handle_move_result(
    ecs: &mut World,
    res: Result<(i32, i32), FailedMoveReason>,
    verbose: bool,
) {
    let mut log = ecs.fetch_mut::<GameLog>();
    match res {
        Ok((x, y)) => {
            if verbose {
                log.entries
                    .push(format!("Moved entity to x: {} y: {}", x, y));
            }
        }
        Err(e) => match e {
            FailedMoveReason::TileBlocked => log
                .entries
                .push("ERROR: Tile entity tried to move on is blocked".to_string()),
            FailedMoveReason::UnableToGrabEntity => {
                log.entries.push("ERROR: Failed to grab entity".to_string())
            }
        },
    }
}

/* 
/// Generate a key for a test, returns back the expected state after the command is run
pub fn generate_key(initial_state: &RunState, ctx: &mut BTerm) -> RunState  {
    let mut rng = thread_rng();
    let key: i32 = rng.gen_range(0..8);

    println!("key: {}", key);
    
    *initial_state
}
*/
