//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project. The unit_list and fort_list functions were built with help
//! from the rust roguelike tutorial, with them being modified versions of the
//! inventory menu provided by the tutorial here:
//! https://bfnightly.bracketproductions.com/rustbook/chapter_9.html
//!
//! The tutorial has it's own system it uses for the inventory, but in my case I instead rely on
//! enums in the units/forts to denote ownership and use marker traits to tell other functions
//! which forts/units are currently selected by the player

use crate::gui::{draw_selection_box, draw_selection_options, MenuResult, SelectionType};
use crate::PlayerOrder;
use crate::{selection, Fort, Name, State};
use bracket_lib::prelude::*;
use specs::prelude::*;

use super::select_player;

/// Used for printing out a list of the units a player currently has and is able to move
pub fn fort_list(gs: &mut State, ctx: &mut BTerm) -> MenuResult {
    let bg = RGB::named(BLACK);
    let y = 15;

    let mut player_forts: Vec<(Entity, String)> = Vec::new();
    {
        let forts = gs.ecs.read_storage::<Fort>();
        let names = gs.ecs.read_storage::<Name>();
        let entities = gs.ecs.entities();

        let player_enum: Option<PlayerOrder> = select_player(&gs.ecs);

        let player_enum = player_enum.unwrap();

        draw_selection_box(ctx, "Fort List".to_string());

        for (_fort, name, entity) in (&forts, &names, &entities)
            .join()
            .filter(|fort| fort.0.owner == player_enum)
        {
            player_forts.push((entity, name.name.to_string()));
        }
    }
    ctx.draw_box(14, y + 9, 30, 3, RGB::named(WHITE), bg);
    ctx.print_color(
        16,
        y + 10,
        RGB::named(YELLOW),
        bg,
        format!("Selection: {}", gs.selected),
    );
    draw_selection_options(gs, ctx, &player_forts);

    selection(gs, ctx, player_forts, SelectionType::Fort)
}
