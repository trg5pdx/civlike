# Civlike

A Civ-like 4X game made in Rust using the Bracket-lib roguelike library. 
Currently, the game generates a map featuring different types of terrain like 
grassland, forests, coasts, mountains, and water. Only one player is in the 
game at this time, which you start with 3 units and a fort and all tiles 
directly around the fort will be claimed for the player. You can open a menu 
for your current units or forts and select them and then do different things 
with them. For the unit, you can walk around the map, claim tiles, and build 
forts on claimed tiles; you can also uncover new territory using a unit. For 
forts, you can build more units with them.

# How to build and run
First, install rust before trying to build the game if you don't have rust 
already. I recommend that you head over to rustup for instructions on how to 
install rust on your computer: [Link to Rustup](https://rustup.rs)       

Clone this repo in a directory where you want to run the game in using the
 command below:

`git clone https://gitlab.cecs.pdx.edu/trg5/civlike.git`

`cd civlike`

Afterwards, you can run the command below to build the game and run it

`cargo run`

The game should launch once it's done compiling and you should see the map, the 
player cursor, and a few of your units. If you wish to run the game again later, 
you can run the executable in target/debug/ called civlike instead of building 
again.

# Flags
You can run the game with some flags to change how the game will run. Currently, 
the only two flags that exist are: 

`-verbose` lets you see all messages in the message log 
(Warning, will print out a lot of messages)      

`-godmode` for now, the only thing it does is extend the view range of your 
units to be very far, so you can get a good look at how the world generates.       

`-fuzz_test` is a test where the game will randomly generate an input and try it,
used as a way to find bugs in the games code. If you run it, the only way you can
exit it is by killing the game, either by hitting Ctrl-c in the terminal window,
or by using Alt-F4 to kill it (If your desktop environment doesn't use Alt-F4,
then use the combination that it uses to kill a window)      


To run these flags, run either:

`cargo run -- <FLAG HERE>`      
or       
`./target/debug/civlike <FLAG HERE>`       

# Key for symbols     
+: player cursor; lets you move over the tiles and get information about the 
game world       
i: units       
M: Player forts      

Pink background tiles: tiles that are claimed by player one      
Red background tiles: tiles that are claimed by player two      

# Controls

[W/A/S/D] Movement controls for the cursor and units      

Cursor Mode:     
[Esc] Closes the game      
[I] Opens a menu listing the units a player currently owns, from there press 
the letter corresponding to the unit you wish to take over, and then you will 
switch to unit mode       
[F] Opens a menu listing the forts a player currently owns, from there press 
the letter corresponding to the fort you wish to take over, and then you will 
switch to fort mode       

Unit Mode:       
[I] Switch back to cursor mode        
[G] Claim a tile for the player       
[B] Build a fort on the current tile (Tile needs to be claimed)       

Fort Mode:       
[B] Build a unit at the current fort, if the tile isn't currently occupied 
by another unit      
[I] Switch back to cursor mode       

Unit/Fort Menu Mode:      
[Esc] Exits the menu without selecting anything       
[A-Z] For selecting something that corresponds to a letter      

# Reflection on developing civlike
Overall, I think the game's development turned out alright considering where I 
started with my knowledge. This is the first actual game I've tried to develop, 
and it's the first time I've tried to build a game using an entity control 
system model, which was difficult to work with at first. The main problem I kept 
running into is that I was approaching building the game from a object oriented 
mindset and so there was a bit of a learning hurdle to get over with 
understanding how to work with an ECS. That was one thing I had a lot of trouble
with, I kept thinking about trying to set it up using datastructures rather than
by doing it the ECS way. If I had a better understanding of ECS from the 
beginning, I think I would have been able to get farther along with the game, 
but I'm happy with where I ended up. Along with following the rust roguelike 
tutorial, I also referred to other documentation online about how to build 
roguelike games, which I came across a guide on how to generate terrain using 
heightmaps, and found that the bracket-lib library had an example using a 
perlin noise map, which I used those to build the map in the game. 

One of the big issues I ran into when trying to build the game was figuring out 
a way to handle moving a unit, which I kept thinking about it through 
switching the player entity to the desired unit so I could reuse the player 
movement code. I eventually got individual unit movement set up through 
keeping the name in the world struct, but Cassaundra introduced me to the idea 
of using marker components for marking a single unit, which ultimately was the 
best way for me to handle that, so I switched to using that instead as it was 
a cleaner way to do it. I also ran into multiple issues when trying to figure 
out a way to handle ownership, which I eventually just set up an enum to keep 
track of the number of players, and then added a field in the player and unit 
structs to keep track of ownership, and during that process set up a seperate 
vector in the map that keeps track of who claims what tile using the same enum.

Thanks to this project, I became more proficient with programming in Rust, 
I learned the basics of how to build a game, and I learned about building games
 using Entity Control Systems, which has helped me think about how to approach 
programming problems in a different way. Overall, I feel like I learned a lot 
and got better with programming while working on this project. I also really 
enjoyed working on it, and once I got over my anxiety and got through working 
on the unit ownership system, I had a lot of fun working on it.   

# Credits

Thank you to Bart Massey and Cassaundra Smith for the help they provided me 
throughout this project.     
A big thank you to Herbert Wolverson for writing the tutorial on making a 
roguelike in rust      
[Rust Roguelike Tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html)     
[Resource I used which led to learning about heightmaps](https://github.com/marukrap/RoguelikeDevResources)      
[Bracket Noise examples used in src/heightmap.rs](https://github.com/amethyst/bracket-lib/tree/master/bracket-noise)      
[Tutorial on generating 2D maps using heightmaps](https://gillesleblanc.wordpress.com/2012/10/16/creating-a-random-2d-game-world-map/)      
