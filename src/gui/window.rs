//! Made by: Thomas Gardner, 2022

use crate::gui::{MenuResult, SelectionType};
use crate::{State, Moving, Selected};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn selection(gs: &mut State, ctx: &mut BTerm, owned: Vec<(Entity, String)>, selected: SelectionType) -> MenuResult 
{ 
    let count = owned.len() as u32;
    match ctx.key {
        None => { MenuResult::NoResponse }, 
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => MenuResult::Cancel,
                VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
					if let Ok(result) = gs.selected.parse::<u32>() {
						gs.selected = "1".to_string();
                        if result < count && result > 0 {
                            match selected {
                                SelectionType::Unit => {
                                    let mut moving = gs.ecs.write_storage::<Moving>();
                                    moving	
                                        .insert(owned[result as usize - 1].0, Moving {})
                                        .expect("Unable to mark unit as selected");
                                    gs.selected = String::new();
                                },
                                SelectionType::Fort => {
                                    let mut selected = gs.ecs.write_storage::<Selected>();
                                    selected
                                        .insert(owned[result as usize - 1].0, Selected {})
                                        .expect("Unable to mark fort as selected");
                                    gs.selected = String::new();
                                },
                            }
                            MenuResult::Selected
                        } else {	
                            gs.selected = String::new();
                            MenuResult::NoResponse
                        }
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
