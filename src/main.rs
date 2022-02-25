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

mod heightmap;
mod gui;
mod spawner;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;

pub mod camera;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
	Paused,
	MoveCursor,
	MoveUnit,
	ShowUnits,
}

pub struct State {
    pub ecs: World,
	pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {       
		let mut mapindex = MapIndexingSystem{};
		mapindex.run_now(&self.ecs);

		let mut vis = VisibilitySystem{};
		vis.run_now(&self.ecs);
	
		let mut owned = UnitOwnershipSystem{};
		owned.run_now(&self.ecs);

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
				self.runstate = RunState::Paused;	
			}
			RunState::MoveUnit => {
				self.run_systems();
				self.runstate = unit_input(self, ctx);
			}
			RunState::ShowUnits => {
				if gui::show_units(self, ctx) == gui::UnitMenuResult::Cancel {
					self.runstate = RunState::Paused;
				}				
			}
			_ => {
				self.runstate = player_input(self, ctx);
			}
		}

    }
}

use std::env;

fn main() -> BError {
	let mut cmd_args = Vec::new();	

	for arg in env::args().skip(1) {
		cmd_args.push(arg.clone());
	}
	
    let context = BTermBuilder::simple80x50()
            .with_title("Civlike")
            .build()?;

    let mut gs = State {
        ecs: World::new(),
		runstate: RunState::MoveCursor,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Unit>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<OwnedBy>();
    gs.ecs.register::<UnitControl>();
   		
	let map = Map::new_map();
 
    gs.ecs.insert(map);
	
	let mut range = 8;
	
	// Added this to let me play with world generation later and not deal with being unable to see the whole map	
	if !cmd_args.is_empty()	{
		if cmd_args.remove(0).as_str() == "-godmode" {
			range = 60;
		}
	}
	
	let player_pos = (40, 25);
	
	gs.ecs.insert(Point::new(player_pos.0, player_pos.1));
	
	let player_entity = spawner::player(&mut gs.ecs, player_pos);
	gs.ecs.insert(player_entity);
	
	// currently used for unit testing
	let unit_entity = spawner::unit(&mut gs.ecs, player_pos, range);		
	gs.ecs.insert(unit_entity);	
	let unit_entity2 = spawner::unit(&mut gs.ecs, (40, 26), range);		
	gs.ecs.insert(unit_entity2);	
	
	// let _unit_entity_2 = spawner::unit(&mut gs.ecs, (45, 26), range);		
	/* let mut units = UnitQueue { queue: Vec::new() };
	for i in 0..3 {
		let player_pos = (40 + i, 25);
		let unit_entity = spawner::unit(&mut gs.ecs, player_pos, range);		
		units.queue.push(unit_entity);		
	} */
	
    main_loop(context, gs)
}
