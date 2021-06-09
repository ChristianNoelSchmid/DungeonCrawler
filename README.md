
### Christian Schmid
### Doctor Barton Massey
### Rust Programming - Spring 2021 <br><br>

## Final Project

This is an extensive project, encompassing several crates with different functionality. Combined, these crates create a simple, network-multiplayer Dungeon Escape game. It features the players controlling a character, running away from server-controlled goblins which threaten to kill them. The players can either escape to the trap-door on the other side of the procedurally generated dungeon, or die!

This game was created with hybrid Rust / C#. The dungeon generator, udp datagram manager, and event / state manager were all built with Rust, while the client-side game was built with the Unity Engine using C#. For this course, I would (this is probably obvious, but I want to make it clear) like to submit the Rust crates as my work. These crates, I believe, are sufficient for this term's final project, and although I could have limited my development to just the server-side, it would have been less fun.

## Project Layout
- Rust Applications
    - dungeon_generator (`/crates/dungeon_generator/`)
    - simple_serializer (`/crates/simple_serializer/`)
    - udp_server (`/crates/udp_server/`)
    - dungeon_crawler_server (`/dungeon_crawler_server/`)

- C# / Unity Applications
    - Dungeon Crawler (`/Dungeon Crawler/`)

## Playing the Game

The game can be played by running two programs - the server and client.

The server can be run by navigating to `/dungeon_crawler_server/` and running the command `cargo run` in the terminal. This will begin the event server / state manager on ip address `0.0.0.0`, port `2000`. The server can be closed with `CTRL-C`.

The client can be built with the UnityEngine, but there have also been two os-builds created in the client directory (`/Dungeon Crawler/build/`). They can run on Windows 10, or Ubuntu 20.04 (and possibly earlier versions, but I'm not 100% certain).

Upon starting the client, the UI will ask for a name, and the server's ip address. Those running the server on their local computer can simply use `127.0.0.1` to connect. If an external server is being used, the client can connect via its public ip address. This game communicates over UDP, non-encrypted channels.

Controls for the game are `WASD` to move.

## Description of Project Implementation

With the extent of this project, I will seek to only provide the overarching concepts for each crate, to condense this document. If you wish to know more about a particular system, please look at the documentation in the code, or email me at `christian.noel.schmid@gmail.com`. I had a great time building this and would love to talk about it more!

Each crate will be explained in turn.<br><br>

### `dungeon_generator` (`/crates/dungeon_generator/`)

The dungeon_generator is a procedural generation of paths, using simple random traversal and perlin noise. The `Dungeon` struct (impl. in `/src/inst.rs/`) holds a collection of said paths, a width/height bounds of the dungeon, a point of entrance and a point of exit.

The generation of the paths (impl. in `/src/gen.rs`) first creates a random entrance-point (using the `rand` crate: `https://docs.rs/rand/0.8.3/rand/`) located on a random x-position between 0 and `width`, and chooses one of the opposite extremes for the y-position (0 or `height`). The exit is then generated on the opposite side of the `Dungeon`. 

At this point, a one-width path is randomly generated from the entrance to the exit. The path can travel left, right, or in the vertical direction towards the exit. In this way it is always heading towards the exit, eventually reaching it and completing the main path.

After this the program uses the `noise` crate (`https://docs.rs/noise/0.7.0/noise/`) to layer the main path with perlin noise. For each square in the dungeon, the program retrieves a new perlin-noise value based on X-Y coordinates, and checks if the 0-1 value is at a certain threshold (> 0.05). If so, a path is formed on that square. Finally, after all this, the generator runs a series of tests on its paths to determine if each perlin path is indeed connected to the main path. If not, the path is destroyed. If so, the path is officially added to the `Dungeon`'s path.

The `Dungeon` struct is used in the `dungeon_crawler_server` crate (the main server crate), and generates the world dungeon each time a level is completed.

**`dungeon_generator tests` (`./tests/`)**

- `test_100` - because the dungeons are meant to be randomly generated, there is little that can be determined from unit-tests. However, this particular test determines that every dungeon generated does, in fact, have a path from the entrance point to exit point of the given `Dungeon`, using a series of breadth-first searches on about 50 dungeons. If any of these dungeons do not have a path from entrance to exit, the whole test fails.

### `simple_serializer` (`/crates/simple_serializer`)

This is a very straightforward crate, that simply creates two traits: `Serialize` and `Deserialize`. These traits are assigned to several structs in the project, and server to help define how communications between client and server are handled. Both traits allow a generic type assigned to what the struct is serialized *to* for maximum flexibility.

**`simple_serializer tests`** - there are no tests associated with this crate, as it doesn't implement anything on its own.

### `udp_server` (`/crates'/udp_server`)

This crate starts to get into the "meat and potatoes" of the project, so to speak. It implements a UDP socket communication with both reliable and unreliable datagram types. This is the datagram manager that is used to communicate from the server to the client, and receive messages from the client to the server. For the client-side, a C# Udp implementation is used, which I built in a previous project (with minor tweaking).

