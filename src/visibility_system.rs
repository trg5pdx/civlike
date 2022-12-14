//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. This section of code comes from section 2.4: Field of View
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_5.html

use crate::{xy_idx, Map, Position, Viewshed};
use bracket_lib::prelude::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

/// A system for handling how the units see the tiles around them
impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos) = data;

        for t in map.visible_tiles.iter_mut() {
            *t = false
        }

        for (_ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            }
            // If this is a unit, reveal what they can see
            for vis in viewshed.visible_tiles.iter() {
                let idx = xy_idx(vis.x, vis.y);
                map.revealed_tiles[idx] = true;
                map.visible_tiles[idx] = true;
            }
        }
    }
}
