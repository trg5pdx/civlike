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
	let count = fort_list.count() as u32;
	
    let mut player_forts: Vec<(Entity, String)> = Vec::new();
    let y = 15;
	
	let height = 10;
	let y_cord = y - 2;	

    ctx.draw_box(14, y - 2, 30, height, RGB::named(WHITE), bg);
    ctx.print_color(18, y_cord, RGB::named(YELLOW), bg, "Fort List");
    ctx.print_color(18, y_cord + height, RGB::named(YELLOW), bg, "ESCAPE to cancel");
	

    for (_fort, name, entity) in (&forts, &names, &entities)
        .join()
        .filter(|fort| fort.0.owner == player_enum)
    {
		player_forts.push((entity, name.name.to_string()));	
    }

	let current_option = gs.last_option;
	let mut offset = 0;
	if count > 7 {
		offset = count - 7;
	}	
	for i in 0..7 {
		let mut index = current_option + i;
		if current_option > offset {
			index = offset + i;
		}
		let width = index.to_string().len();
		if index < count as u32 {
			ctx.set(17, y + i, RGB::named(WHITE), bg, to_cp437('('));
			ctx.print_color(18, y + i, RGB::named(YELLOW), bg, format!("{}", index + 1));
				
			ctx.set(18 + width, y + i, RGB::named(WHITE), bg, to_cp437(')'));
			ctx.print(20 + width, y + i, player_forts[index as usize].1.clone());
		}
		if index == current_option {
			ctx.set(16, y + i, RGB::named(WHITE), bg, to_cp437('>')); 
		}
	}
	
    {
        ctx.draw_box(14, y + 9, 30, 3, RGB::named(WHITE), bg);
        ctx.print_color(16, y + 10, RGB::named(YELLOW), bg, format!("Selection: {}", gs.selected));
    }
    match ctx.key {
        None => { MenuResult::NoResponse }, 
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => MenuResult::Cancel,
                VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
					let result = gs.selected.parse::<u32>().unwrap();	
					gs.selected = "1".to_string();
 
                    if result <= count {
                        let mut selected = gs.ecs.write_storage::<Selected>();
                        selected
                            .insert(player_forts[result as usize - 1].0, Selected {})
                            .expect("Unable to mark fort as selected");
                        gs.selected = String::new();
                        MenuResult::Selected
                    } else {
                        gs.selected = String::new();
                        MenuResult::NoResponse
                    }
                },
                VirtualKeyCode::Back => {
                    gs.selected.pop();
					MenuResult::NoResponse
                },
				VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => { gs.selected.push('0'); MenuResult::NoResponse },
				VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => { gs.selected.push('1'); MenuResult::NoResponse },
				VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => { gs.selected.push('2'); MenuResult::NoResponse },
				VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => { gs.selected.push('3'); MenuResult::NoResponse },
				VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4 => { gs.selected.push('4'); MenuResult::NoResponse },
				VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5 => { gs.selected.push('5'); MenuResult::NoResponse },
				VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6 => { gs.selected.push('6'); MenuResult::NoResponse },
				VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7 => { gs.selected.push('7'); MenuResult::NoResponse },
				VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8 => { gs.selected.push('8'); MenuResult::NoResponse },
				VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9 => { gs.selected.push('9'); MenuResult::NoResponse },
				VirtualKeyCode::Up => {
					if gs.last_option > 0 {
						gs.last_option -= 1;
						gs.selected = format!("{}", gs.last_option + 1);
					}
					MenuResult::NoResponse
				},
				VirtualKeyCode::Down => {
					if gs.last_option < count - 1 {
						gs.last_option += 1;
						gs.selected = format!("{}", gs.last_option + 1);
					}
					MenuResult::NoResponse
				},
                _ => { 
					MenuResult::NoResponse
                },
            }
		},
    }
}
