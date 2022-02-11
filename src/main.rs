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

mod heightmap;
mod gui;

mod visibility_system;
use visibility_system::VisibilitySystem;

pub struct State {
    pub ecs: World
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
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        map.draw_map(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

		gui::draw_ui(&self.ecs, ctx);
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
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
   	
	let map = Map::new_map();
 
    gs.ecs.insert(map);
	
	let mut range = 8;
	
	// Added this to let me play with world generation later and not deal with being unable to see the whole map	
	if !cmd_args.is_empty()	{
		if cmd_args.remove(0).as_str() == "-godmode" {
			range = 60;
		}
	}

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('☺'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player{})
		.with(Viewshed{ visible_tiles: Vec::new(), range, dirty: true })
        .build();
    
    main_loop(context, gs)
}
