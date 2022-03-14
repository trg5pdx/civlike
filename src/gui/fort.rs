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
use crate::{Fort, Name, Player, Selected, State};
use bracket_lib::prelude::*;
use specs::prelude::*;

/// Used for printing out a list of the units a player currently has and is able to move
pub fn fort_list(gs: &mut State, ctx: &mut BTerm) -> MenuResult {
    let players = gs.ecs.read_storage::<Player>();
    let forts = gs.ecs.read_storage::<Fort>();
    let names = gs.ecs.read_storage::<Name>();
    let entities = gs.ecs.entities();
    let bg = RGB::named(BLACK);

    let mut player_enum: Option<PlayerOrder> = None;

    for (_entity, player) in (&entities, &players).join() {
        player_enum = Some(player.order);
    }

    let player_enum = player_enum.unwrap();

    let fort_list = (&forts, &entities)
        .join()
        .filter(|fort| fort.0.owner == player_enum);
    let count = fort_list.count();

    let mut player_forts: Vec<Entity> = Vec::new();
    let mut y;
    let y_check = 25_i32.checked_sub((count / 2).try_into().unwrap());
    
    if y_check.is_some() {
        y = y_check.unwrap();
    } else {
        y = 25
    }

    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, RGB::named(WHITE), bg);
    ctx.print_color(18, y - 2, RGB::named(YELLOW), bg, "Fort List");
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(YELLOW),
        bg,
        "ESCAPE to cancel",
    );

    for (i, (_fort, name, entity)) in (&forts, &names, &entities)
        .join()
        .filter(|fort| fort.0.owner == player_enum)
        .enumerate()
    {
        ctx.set(17, y, RGB::named(WHITE), bg, to_cp437('('));
        ctx.set(
            18,
            y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            97 + i as FontCharType,
        );
        ctx.set(19, y, RGB::named(WHITE), bg, to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        player_forts.push(entity);
        y += 1;
    }

    match ctx.key {
        None => MenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => MenuResult::Cancel,
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    let mut selected = gs.ecs.write_storage::<Selected>();
                    selected
                        .insert(player_forts[selection as usize], Selected {})
                        .expect("Unable to mark fort as selected");
                    return MenuResult::Selected;
                }
                MenuResult::NoResponse
            }
        },
    }
}
