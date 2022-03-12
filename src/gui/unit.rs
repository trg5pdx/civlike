/*
    The unit_list and fort_list functions were built with help from the rust roguelike tutorial,
    with them being modified versions of the inventory menu provided by the tutorial here:
    https://bfnightly.bracketproductions.com/rustbook/chapter_9.html

    The tutorial has it's own system it uses for the inventory, but in my case I instead rely on
    enums in the units/forts to denote ownership and use marker traits to tell other functions
    which forts/units are currently selected by the player
*/

use crate::gui::MenuResult;
use crate::PlayerOrder;
use crate::{Moving, Name, Player, State, Unit};
use bracket_lib::prelude::*;
use specs::prelude::*;

/// Used for printing out a list of the units a player currently has and is able to move
pub fn unit_list(gs: &mut State, ctx: &mut BTerm) -> MenuResult {
    let players = gs.ecs.read_storage::<Player>();
    let units = gs.ecs.read_storage::<Unit>();
    let names = gs.ecs.read_storage::<Name>();
    let entities = gs.ecs.entities();
    let bg = RGB::named(BLACK);

    let mut player_enum: Option<PlayerOrder> = None;

    for (_entity, player) in (&entities, &players).join() {
        player_enum = Some(player.order);
    }

    let player_enum = player_enum.unwrap();

    let unit_list = (&units, &entities)
        .join()
        .filter(|unit| unit.0.owner == player_enum);
    let count = unit_list.count();

    let mut owned_units: Vec<Entity> = Vec::new();
    let mut y = (25 - (count / 2)) as i32;

    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, RGB::named(WHITE), bg);
    ctx.print_color(18, y - 2, RGB::named(YELLOW), bg, "Unit List");
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(YELLOW),
        bg,
        "ESCAPE to cancel",
    );

    for (i, (_unit, name, entity)) in (&units, &names, &entities)
        .join()
        .filter(|unit| unit.0.owner == player_enum)
        .enumerate()
    {
        ctx.set(17, y, RGB::named(WHITE), bg, to_cp437('('));
        ctx.set(18, y, RGB::named(YELLOW), bg, 97 + i as FontCharType);
        ctx.set(19, y, RGB::named(WHITE), bg, to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        owned_units.push(entity);
        y += 1;
    }

    match ctx.key {
        None => MenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => MenuResult::Cancel,
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    let mut moving = gs.ecs.write_storage::<Moving>();
                    moving
                        .insert(owned_units[selection as usize], Moving {})
                        .expect("Unable to mark unit as moving");
                    return MenuResult::Selected;
                }
                MenuResult::NoResponse
            }
        },
    }
}