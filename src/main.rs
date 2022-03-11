//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use bracket_lib::prelude::*;
use specs::prelude::*;

mod map;
pub use map::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod unit;
pub use unit::*;

mod fort;
pub use fort::*;

mod gamelog;
pub use gamelog::*;

mod gui;
mod heightmap;
mod spawner;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;

pub mod camera;

/// Marks what state the games running in to allow the player to open their unit/fort lists
/// and move their cursor around the map
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    MoveCursor,
    MoveUnit,
    ShowUnits,
    SelectedFort,
    ShowForts,
}

/// Used for returning why a move failed to happen
pub enum FailedMoveReason {
    TileBlocked,
    UnableToGrabEntity,
}

/// Contains the game world and all of it's entities within it, and an enum for denoting
/// what state the game is currently in
pub struct State {
    pub ecs: World,
    pub runstate: RunState,
	pub godmode: bool,
	pub verbose: bool,
}

impl State {
    fn run_systems(&mut self) {
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        camera::render_camera(&self.ecs, ctx);
        gui::draw_ui(&self.ecs, ctx);

        match self.runstate {
            RunState::MoveCursor => {
                self.run_systems();
                self.runstate = player_input(self, ctx);
            }
            RunState::MoveUnit => {
                self.run_systems();
                self.runstate = unit_input(self, ctx);
            }
            RunState::ShowUnits => {
                let result = gui::unit_list(self, ctx);
                match result {
                    gui::MenuResult::Cancel => self.runstate = RunState::Paused,
                    gui::MenuResult::Selected => {
                        self.runstate = RunState::MoveUnit;
                    }
                    gui::MenuResult::NoResponse => {}
                }
            }
            RunState::SelectedFort => {
                self.run_systems();
                self.runstate = fort_input(self, ctx);
            }
            RunState::ShowForts => {
                let result = gui::fort_list(self, ctx);
                match result {
                    gui::MenuResult::Cancel => self.runstate = RunState::Paused,
                    gui::MenuResult::Selected => {
                        self.runstate = RunState::SelectedFort;
                    }
                    gui::MenuResult::NoResponse => {}
                }
            }
            _ => {
                self.runstate = player_input(self, ctx);
            }
        }
    }
}

pub fn handle_move_result(ecs: &mut World, res: Result<(i32, i32), FailedMoveReason>, verbose: bool) {
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
                .push(format!("ERROR: Tile entity tried to move on is blocked")),
            FailedMoveReason::UnableToGrabEntity => {
                log.entries.push(format!("ERROR: Failed to grab entity"))
            }
        },
    }
}

use std::env;

fn main() -> BError {
    let mut range = 8;
    let mut cmd_args = Vec::new();

    let context = BTermBuilder::simple80x50().with_title("Civlike").build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::MoveCursor,
		godmode: false,
		verbose: false,
    };

    for arg in env::args().skip(1) {
        cmd_args.push(arg.clone());
    }

	while !cmd_args.is_empty() {
		let cmd_input = cmd_args.pop();

		if let Some(arg) = cmd_input { 
			match arg.as_str() {
				"-godmode" => { range = 400; gs.godmode = true },
				"-verbose" => { gs.verbose = true },
				_ => { },
			}
		}	
	}

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Unit>();
    gs.ecs.register::<Fort>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<Moving>();
    gs.ecs.register::<Selected>();
    gs.ecs.register::<GameLog>();

    let map = Map::new_map();

    gs.ecs.insert(map);

    let x_range: (i32, i32) = (0, (MAPWIDTH - 1) as i32);
    let y_range: (i32, i32) = (0, (MAPHEIGHT - 1) as i32);

    let position: (i32, i32) = spawner::generate_coordinates(&gs.ecs, x_range, y_range);
    spawner::spawn_player_entities(&mut gs.ecs, position, range, PlayerOrder::PlayerOne);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Civlike!".to_string()],
    });

    main_loop(context, gs)
}
