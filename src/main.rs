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

pub mod camera;

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
		
		camera::render_camera(&self.ecs, ctx);
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
	
	let (player_x, player_y) = (40, 25);
	
	gs.ecs.insert(Point::new(player_x, player_y));

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: to_cp437('+'),
            fg: RGB::named(BLACK),
            bg: RGB::named(PURPLE),
			render_order: 0,
        })
        .with(Player{})
		.with(Viewshed{ visible_tiles: Vec::new(), range, dirty: true })
        .build();
	
	// currently used for unit testing
    gs.ecs
        .create_entity()
        .with(Position { x: player_x + 3, y: player_y })
        .with(Renderable {
            glyph: to_cp437('☺'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
			render_order: 1,
        })
		.with(Unit {
			health: 20,
			strength: 8,
			owner: "Player1".to_string(),
		})
		.with(Viewshed{ visible_tiles: Vec::new(), range, dirty: true})
        .build();
    
    main_loop(context, gs)
}
