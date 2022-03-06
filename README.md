# Civlike

A Civ-like 4X game made in Rust using the Bracket-lib roguelike library

# How to build and run
First, install rust before trying to build the game if you don't have rust already.
Clone this repo in a directory where you want to run the game in using the command below:

`git clone https://gitlab.cecs.pdx.edu/trg5/civlike.git`

`cd civlike`

Afterwards, you can run the command below to build the game and run it

`cargo run`

The game should launch once it's done compiling and you should see the map, the player cursor, and a few of your units

# Key for symbols
+: player cursor; lets you move over the tiles and get information about the game world
i: units
M: Player forts

Pink background tiles: tiles that are claimed by player one
Red background tiles: tiles that are claimed by player two

# Controls

[W/A/S/D] Movement controls for the cursor and units

Cursor Mode:
[I] Opens a menu listing the units a player currently owns, from there press the letter corresponding to the unit you wish to take over, and then you will switch to unit mode
[F] Opens a menu listing the forts a player currently owns, from there press the letter corresponding to the fort you wish to take over, and then you will switch to fort mode

Unit Mode:
[I] Switch back to cursor mode
[G] Claim a tile for the player
[B] Build a fort on the current tile (Tile needs to be claimed)

Fort Mode:
[B] Build a unit at the current fort, if the tile isn't currently occupied by another unit
[I] Switch back to cursor mode

# Credits

A big thank you to Herbert Wolverson for writing the tutorial on making a roguelike in rust      
[Rust Roguelike Tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html)
[Bracket Noise examples used in src/heightmap.rs](https://github.com/amethyst/bracket-lib/tree/master/bracket-noise)
[Tutorial on generating 2D maps using heightmaps](https://gillesleblanc.wordpress.com/2012/10/16/creating-a-random-2d-game-world-map/)
