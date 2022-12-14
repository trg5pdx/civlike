//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. This section of code comes from section 2.6: Dealing Damage
//! Link: 
//! https://bfnightly.bracketproductions.com/rustbook/chapter_7.html#blocking-access

use crate::{xy_idx, BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

/// System for keeping track of what entities are where, currently only used for
/// checking and updating tiles as being blocked on the map
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.clear_blocked();
        map.populate_blocked();
        map.clear_content_index();
        for (entity, position) in (&entities, &position).join() {
            let idx = xy_idx(position.x, position.y);

            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(entity);
        }
    }
}
