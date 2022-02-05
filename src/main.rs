//! Made by: Thomas Gardner, 2022
//!
//! Big thanks to the Rust roguelike tutorial, which helped quite a bit with
//! this project, will cite the tutorial for code that directly came from it
//! to properly give credit
//!
//! Link: https://bfnightly.bracketproductions.com/rustbook/chapter_0.html

use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

mod map;
pub use map::*;

mod heightmap;

mod gui;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();
	
	let map_width: i32 = MAPWIDTH as i32 - 1;
	let map_height: i32 = MAPHEIGHT as i32 - 1;

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Water && 
		   map[destination_idx] != TileType::Mountain &&
		   map[destination_idx] != TileType::Ice {
            pos.x = min(map_width, max(0, pos.x + delta_x));
            pos.y = min(map_height, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {}, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
			VirtualKeyCode::Escape => std::process::exit(0),
            _ => {}
        },
    }
}

struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

		gui::draw_ui(&self.ecs, ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
            .with_title("Civlike")
            .build()?;

    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    
    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('☺'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player{})
        .build();
    
    main_loop(context, gs)
}
