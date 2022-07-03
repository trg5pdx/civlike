//! Made by: Thomas Gardner, 2022

use crate::{State, Unit};
use specs::prelude::*;

pub fn next_turn(gs: &mut State) {
    let mut units = gs.ecs.write_storage::<Unit>();
    let entities = gs.ecs.entities();

    for (_entity, unit) in (&entities, &mut units).join() {
        unit.stamina = 8;
    }

    gs.turns += 1;
}
