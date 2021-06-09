
### Christian Schmid
### Doctor Barton Massey
### Rust Programming - Spring 2021 <br><br>

## Final Project
## Description of Project Implementation

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

The server can be run by navigating to `/dungeon_crawler_server/` and running the command `cargo run` in the terminal. This will begin the event server / state manager on ip address `0.0.0.0`, port `2000`. 

The client can be built with the UnityEngine, but there have also been two os-builds created in the client directory (`/Dungeon Crawler/build/`). They can run on Windows 10, or Ubuntu 20.04 (and possibly earlier versions, but I'm not 100% certain).

## Unit Tests

* `test_wrk_dir_eq()` - this function tests that a directory created with a substantial tree structure has all the expected leaves upon calling `paths` on the `OsState`. It compares an `OsState`'s returned `paths` with a `HashSet` collection, which contains all the leaf directory `String`s. 

* `test_subdir_leafs()` - this function is similar to `test_wrk_dir_eq()`, with the exception that while it is checking each leaf, it also traverses down sub-directories to ensure that the expected `paths` collection is appropriately updated. This both checks `chdir` and `paths` in a single test.

* `test_err_inputs()` - this function simply runs all three different scenarios on an `OsState` that can lead to an `Err` result, ensuring that the `OsState` is successfully capturing improper inputs.
