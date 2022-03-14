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
pub use gui::fort::*;
pub use gui::unit::*;

mod error_handling;
use crate::error_handling::generate_key;
mod heightmap;
mod spawner;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;

pub mod camera;

/// Marks what state the games running in to allow the player to open their unit/fort lists
/// and move their cursor around the map
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
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

pub struct ExpectedFuzzState {
    first: RunState,
    second: Option<RunState>,
}

/// Contains the game world and all of it's entities within it, and an enum for denoting
/// what state the game is currently in
pub struct State {
    pub ecs: World,
    pub runstate: RunState,
    pub godmode: bool,
    pub verbose: bool,
    pub fuzz_test: bool,
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
        let mut expected_state: Option<ExpectedFuzzState> = None;

        ctx.cls();
        camera::render_camera(&self.ecs, ctx);
        gui::draw_ui(&self.ecs, ctx);

        if self.fuzz_test {
            expected_state = Some(generate_key(self.runstate, ctx));
        }

        self.run_systems();
        match self.runstate {
            RunState::MoveCursor => {
                self.runstate = player_input(self, ctx);
            }
            RunState::MoveUnit => {
                self.runstate = unit_input(self, ctx);
            }
            RunState::ShowUnits => {
                let result = unit_list(self, ctx);
                match result {
                    gui::MenuResult::Cancel => self.runstate = RunState::MoveCursor,
                    gui::MenuResult::Selected => {
                        self.runstate = RunState::MoveUnit;
                    }
                    gui::MenuResult::NoResponse => {}
                }
            }
            RunState::SelectedFort => {
                self.runstate = fort_input(self, ctx);
            }
            RunState::ShowForts => {
                let result = fort_list(self, ctx);
                match result {
                    gui::MenuResult::Cancel => self.runstate = RunState::MoveCursor,
                    gui::MenuResult::Selected => {
                        self.runstate = RunState::SelectedFort;
                    }
                    gui::MenuResult::NoResponse => {}
                }
            }
        }

        if let Some(state) = expected_state {
            if state.second.is_some() {
                assert!((self.runstate == state.first) || (self.runstate == state.second.unwrap()));
            } else if self.runstate != state.first {
                panic!(
                    "Error: runstates don't match! States: {:?} {:?}; Key: {:?}",
                    self.runstate, state.first, ctx.key
                );
            }
        }
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
        fuzz_test: false,
    };

    for arg in env::args().skip(1) {
        cmd_args.push(arg.clone());
    }

    while !cmd_args.is_empty() {
        let cmd_input = cmd_args.pop();

        if let Some(arg) = cmd_input {
            match arg.as_str() {
                "-godmode" => {
                    range = 400;
                    gs.godmode = true
                }
                "-verbose" => gs.verbose = true,
                "-fuzz_test" => gs.fuzz_test = true,
                _ => {}
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
        message_type: vec![MessageType::Other],
    });

    main_loop(context, gs)
}
