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

pub mod camera;

pub struct State {
    pub ecs: World,
	pub move_unit: bool,
}

impl State {
    fn run_systems(&mut self) {
		let mut vis = VisibilitySystem{};
		vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) { 
		if self.move_unit {
			ctx.cls();
			let input = unit_input(self, ctx);
			if !input {
				self.move_unit = false;	
			}	
			self.run_systems();
			
			camera::render_camera(&self.ecs, ctx);
			gui::draw_ui(&self.ecs, ctx);
		} else {
			ctx.cls();
			
			let input = player_input(self, ctx);

			if let Some(5) = input {
				self.move_unit = true;
			}

			self.run_systems();
			
			camera::render_camera(&self.ecs, ctx);
			gui::draw_ui(&self.ecs, ctx);
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
		move_unit: false,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Unit>();
   	
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
	
	// currently used for unit testing
	let unit_entity = spawner::unit(&mut gs.ecs, player_pos, range);		
	/* let mut units = Vec::new();
	for i in 0..3 {
		let player_pos = (40 + i, 25);
		let unit_entity = spawner::unit(&mut gs.ecs, player_pos, range);		
		units.push(unit_entity);		
	} */
	
    main_loop(context, gs)
}
