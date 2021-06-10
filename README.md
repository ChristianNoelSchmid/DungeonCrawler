
### Christian Schmid
### Doctor Barton Massey
### Rust Programming - Spring 2021 <br><br>

## Final Project - Dungeon Crawler

This is a bigger project, encompassing several crates with different functionality. Combined, these crates create a simple, network-multiplayer Dungeon Escape game. It features the players controlling a character, running away from server-controlled goblins which threaten to kill them. The players can either escape to the trap-door on the other side of the procedurally generated dungeon, or die!

This game was created with hybrid Rust / C#. The dungeon generator, udp datagram manager, and event / state manager were all built with Rust, while the client-side game was built with the Unity Engine using C#. For this course, I would (this is probably obvious, but I want to make it clear) like to submit just the Rust crates as my work (excluding the Unity project). These crates, I believe, are sufficient for this term's final project, and although I could have limited my development to just the server-side, it would have been less fun.<br><br>

## Project Layout
- Rust Applications
    - dungeon_generator (`/crates/dungeon_generator/`)
    - simple_serializer (`/crates/simple_serializer/`)
    - udp_server (`/crates/udp_server/`)
    - dungeon_crawler_server (`/dungeon_crawler_server/`)

- C# / Unity Applications
    - Dungeon Crawler (`/Dungeon Crawler/`)<br><br>

## Example of Project

To see an example of the project in action, with two players on two separate computers, please view the .mp4 in the root directory of this project (`Schmid Christian - Dungeon Crawler.mp4`). I apologize for the quality!

## Building and Running the Project

The game can be played by running two programs - the server and client.

The server can be run by navigating to `/dungeon_crawler_server/` in a terminal from the home directory, and entering the command `cargo run`. This will begin the event server / state manager on ip address `0.0.0.0`, port `2000`. The server can be closed with `CTRL-C`.

The client can be built with the UnityEngine, but there are also two executables available to use in the client application directory. They can run on Windows 10, or Ubuntu 20.04 (and possibly earlier versions, but I'm not 100% certain).

- Windows 10 Client: `/Dungeon Crawler/build/Windows/Dungeon Crawler.exe`
- Ubuntu 20.04 Client: `/Dungeon Crawler/build/Ubuntu 20.04/Dungeon Crawler.x86_64`

Upon starting the client, the UI will ask for a name, and the server's ip address. Those running the server on their local computer can simply use `127.0.0.1` to connect. If an external server is being used, the client can connect via its public ip address. This game communicates over UDP, non-encrypted channels. The game chooses port `2000` automatically. Those connecting to a external server may need to forward their port `2000` on their server computer, protocol UDP.

Controls for the game are `WASD` to move. Try to make it to the exit, and don't get caught!<br><br>

## Testing Done to Ensure Project Works

By and large games tend to be a bit unwieldy, and full of bugs. My game, while having a small focus, is no exception to this. That being said, extensive thought on logic, and unit-testing was implemented to ensure the infrastructure crates were working as they should - especially regarding the UDP datagram server. The game itself was tested by more conventional means - through playing the game itself, testing the multiplayer with my wife and other friends, and working out the bugs until, generally, the final product was as close to bug-free as possible.<br><br>

## Developer Questions

1. **What worked?** - I have much to anwer for this! As a developer who's attempted a deep-dive into Rust before, but never getting it to quite stick, this attempt through was definitely the charm. Although I could've written my code in better ways, I am quite impressed by the final result of my server code. Writing in Rust required me to think critically about each individual stage in my project, which benefited the code-base in great ways. 

Dealing with parallelism in Rust was a breeze. There were a few setbacks, but after comparing it with my brief experience with C++ parallelism (and even broader experience with C# parallelism) in the past, there isn't even a comparison. I created my server to run on 4 independent threads (using `crossbeam` channels: https://docs.rs/crossbeam/0.8.1/crossbeam/), and each one maintains strict control of their own data, while capably communicating with their neighboring threads. Interestingly I found the ownership system made understanding channels even more straightforward than I would have thought. I'm sure I'm not taking Rust's parallelism to its full advantage, but its certainly the best parallel program I've written thus far.

While I'm still wrapping my head around it, the trait system worked perfectly for the `AIPackageManager` system (see below). The sheer amount of flexibility the system offers, at a fraction of the code-base OOP interfaces / inheritance requires is fantastic. 

2. **What didn't work?** - One thing stands out most prominently as I consider this question - `Rc<RefCell>`s. For a while I attempted to use `RefCell`s as a means to populate my `WorldStage` struct (see below) throughout the project. The lack of ownership (for all intents and purposes) that `RefCell`s create is appealing - especially as someone whose main experience is with shared-reference languages. However, I soon realized how convoluted it made my code. `WorldStage` references were everywhere and bugs were harder to track down. In the end, I removed the `RefCell` and decided to simply pass my `WorldStage` as a mutable reference into the functions that required it. This still doesn't seem like the best way to handle it, and I'm going to continue thinking about how I might implement this in the future.

3. **How satisfied are you with the result?** - I am *pretty satisfied*, with some definite caveats. Although I am happy with my server code, I wish that I could implement the client-side in Rust as well (rather than C#). Although I'm a big fan of Unity, I'm finding it difficult synchronizing a client and server of different code-bases. For a little while I attempted to use the new `Bevy` engine written in Rust, which is picking up steam quickly, but is still in development. As it was my first experience with the Entity Component System paradigm, I found it quite confusing. I ended up moving to Unity (which I have a good deal more experience with), and may look at an engine like `Bevy` again later, or maybe a different engine like `Amethyst`. 

4. **What would you like to improve in the future?** - I am an avid hobbyist game developer, and I can see some potential promise in a game like this (if nothing else than it'll be fun to continue to work on). More than that, continuing to work on this will help me understand Rust better, and hone in on my use of traits, parallelism, and ownership. I have a good deal to learn still, but I'm excited at the prospects.

## Description of Project Implementation

With the extent of this project, I will seek to only provide the overarching concepts for each crate, to condense this document. If you wish to know more about a particular system, please look at the documentation in the code, or email me at `christian.noel.schmid@gmail.com`. I had a great time building this and would love to talk about it more!

Each crate will be explained in turn.

### **`dungeon_generator`** (`/crates/dungeon_generator/`)

The dungeon_generator is a procedural generation of paths, using simple random traversal and perlin noise. The `Dungeon` struct (impl. in `/src/inst.rs/`) holds a collection of said paths, a width/height bounds of the dungeon, a point of entrance and a point of exit.

The generation of the paths (impl. in `/src/gen.rs`) first creates a random entrance-point (using the `rand` crate: https://docs.rs/rand/0.8.3/rand/) located on a random x-position between 0 and `width`, and chooses one of the opposite extremes for the y-position (0 or `height`). The exit is then generated on the opposite side of the `Dungeon`. 

At this point, a one-width path is randomly generated from the entrance to the exit. The path can travel left, right, or in the vertical direction towards the exit. In this way it is always heading towards the exit, eventually reaching it and completing the main path.

After this the program uses the `noise` crate (https://docs.rs/noise/0.7.0/noise/) to layer the main path with perlin noise. For each square in the dungeon, the program retrieves a new perlin-noise value based on X-Y coordinates, and checks if the 0-1 value is at a certain threshold (> 0.05). If so, a path is formed on that square. Finally, after all this, the generator runs a series of tests on its paths to determine if each perlin path is indeed connected to the main path. If not, the path is destroyed. If so, the perlin path is officially added to the `Dungeon`'s path.

The `Dungeon` struct is used in the `dungeon_crawler_server` crate (the main server crate), and generates the world dungeon each time a level is completed.

*`dungeon_generator tests`* (`./tests/dungeon_tests.rs`)

- `test_100` - because the dungeons are meant to be randomly generated, there is little that can be determined from unit-tests. However, this particular test determines that every dungeon generated does, in fact, have a path from the entrance point to exit point of the given `Dungeon`, using a series of breadth-first searches on about 50 dungeons. If any of these dungeons do not have a path from entrance to exit, the whole test fails.

### **`simple_serializer`** (`/crates/simple_serializer`)

This is a very straightforward crate, that simply creates two traits: `Serialize` and `Deserialize`. These traits are assigned to several structs in the project, and server to help define how communications between client and server are handled. Both traits allow a generic type assigned to what the struct is serialized *to* for maximum flexibility.

*`simple_serializer tests`* - there are no tests associated with this crate, as it doesn't implement anything on its own.

### **`udp_server`** (`/crates'/udp_server`)

This crate starts to get into the heart of the project, so to speak. It implements a UDP socket communication with both reliable and unreliable datagram types. This is the datagram manager that is used to communicate from the server to the client, and receive messages from the client to the server. For the client-side, a C# Udp implementation is used, which I built in a previous project (with minor tweaking - https://github.com/ChristianNoelSchmid/MultiGardening).

`udp_server` uses and abstracts the standard library's `UdpSocket`, building a framework which allows the parsing of underlying data, before properly forwarding the packet, dropping it, or requesting a resend of data from the client (impl. in `/src/manager/`). It does this by parsing an incoming datagram into one of an enum of `DatagramType`s (impl. in `/src/types.rs`), and responding appropriately to how the client wanted to message to be treated. The types are as follows:

- `Unreliable` - a datagram meant to be simply forwarded
- `Reliable` - first checked to see if it has been received in order, then forwarded. Sends an acknowledgement message back to the client.
- `Acknowledgement` - a message with an associated integer, communicating to the server that a client received a reliable message with the specified index.
- `Resend` - informs the server that the client received a reliable message out of order, and needs the server to resend all outgoing reliable messages. Speeds up communications so server doesn't need to wait until RTT timeout.
- `Drop` - the server did not know how to interpret this message, and drops it.

Datagrams follow the general string-format:

`<DatagramType>::<DatagramValue1>::<DatagramValue2>::<...>::<DatagramMessage>`

Examples of datagram strings the manager can generate and receive include:

`UNR::Moved::2::3::0`<br>
`REL::0::Hello`<br>
`ACK::0`<br>
`RES`<br>

Perhaps the most complex part of this crate is the `AckResolverManager` (impl. in `/src/ack_resolving.rs`). This manager allows the server to ensure that any important message it wishes to send to the client / clients are, in fact, sent. Because UDP does not have a reliable messaging system on its own (like TCP), the `AckResolverManager` handles a simple custom-made one.

Essentially, the server can send a message to a client. Should it choose to send the message reliably, before it does so, it sends the request to the `AckResolverManager`. The `AckResolverManager` does 2 things:

1. Determines what index the reliable datagram should have.
2. Stores the contents of the datagram, its intended recipient, and the index as an `AckResolver`, to be deleted upon receiving an `ACK` datagram from the client.

That index is then appended to the reliable datagram sent to the client.

The server then awaits receiving an `ACK` message from all targets. When the `ACK` message is received, with the same index as the reliable message sent, it removes the `AckResolver` from its cache. If the server has not recieved an `ACK` from the client in a specified amount of time, it resends the reliable datagram, continuing to do so until an `ACK` is received. Each server-to-client connection has a dynamic timeout value which is updated to reflect the approximate RTT time between the server and client.

The `AckResolverManager` can also receive reliable messages from the client. To do so, a map of `SocketAddr`s -> `u32`s are stored, each integer representing the next reliable index the server is expecting from the associated client. When the client sends a reliable message, the server compares its index with its own, and one of three things can occur:

1. The value can be too high, in which case a reliable message was dropped. The server drops the current message and sends a resend datagram to the client.
2. The value is too low, informing the server that the client resent a reliable datagram already received by the server. This can happen if either the server's `ACK` datagram was dropped, or a client-side timeout occured before the `ACK` message reached it. In any case, the server simply resends an `ACK` datagram with the same index, to update the client, and drops the message.
3. The value is equal, informing the server that this is a new, in order datagram. The server forwards the datagram contents to the rest of the program, and sends the client an `ACK` datagram.

This functionality of the `AckResolverManager` creates a simple, straightforward reliable messaging service. This is useful when the server needs to send a syncronization message that *must* reach the clients. A monster moving from one location to another could be considered a low-priority message, and can be sent by conventional means. A player being killed, however, should be sent to all clients, which is where the reliable messaging system comes in.

*`udp_server tests`* (`./tests/datagram_manager_tests.rs`)

- `test_send_recieve` - creates two datagram managers and tests sending a single message between them.
- `test_bulk_send` - the same as `test_send_receive` but sends 100 messages between the two servers.
- `test_reliable_datagram` - sends 50 datagrams, all reliable. Ensures they are sent in order with a counter, which keeps track of what reliable message should be accepted next.
- `test_drop_status` - tests that a `DatagramManager` which has accepted a client does, in fact, drop it after not receiving messages from the client for a specified time (5 seconds).

### **`dungeon_crawler_server`** (`/dungeon_crawler_server/`)

This crate is the largest out of the group. It represents the game itself - it has an active state and logic that is updated every tenth of a second. Along with this it accepts incoming UDP messages from clients and the state updates accordingly. 

This program is, generally, split into two categories: the *EventManager* and the *StateManager*. Each one will be described in detail:
- **`EventManager`** - the `EventManager` (impl. in `/src/events/manager.rs`) connects with a `DatagramManager` (described above), awaiting incoming client messages and sending server messages. These `EventType`s (impl. in `/src/events/types.rs`) are more dependent on the game itself, in contrast to the lower-level messaging the udp datagram service sends and responds to. Both these types layer together to form the complete UDP packet sent to clients. 

    Various `EventType`s include:

    - `Hello` - a client message informing the server they wish to join the game
    - `Welcome` - a server response to a client, informing them of a successful connection, and transmission of the `Dungeon`.
    - `Moved` - client and server packet representing a moved entity, be it a player or server-controlled monster.

    Event messages have the following form, quite similar to datagram messages:

    `<EventType>::<EventValue1>::<EventValue2>::<...>::<EventValueN>`

    With `EventType`s, complete UDP packets can have the following forms:

    `REL::0::Hello::Bob` <br>
    `UNR::Moved::3::23::2::0` <br>
    `REL::46::Escaped::4`

    The `DatagramManager` first cuts off its section and parses the data, followed by the `EventManager` performing the same function. Depending on what message is sent, the `EventManager` may simply relay the message to other clients, or push the message to the `StateManager`, which will update state based on the data given.

- **`StateManager`** - the `StateManager` (impl. in `/src/state/manager.rs`) is the inner-workings of the game itself, handling things like synchronization, enemy searching, pathfinding, and updating enemy AI behaviour. There are a number of different systems associated with the `StateManager`, the most prominent being the `WorldStage`, and the `AIPackageManager`. These both will be discussed below.

    - *`WorldStage`* - the `WorldStage` (impl. in `/src/state/transforms/world_stage.rs`) represents the global representation of all `Actor`s in the game - their position, direction, and certain stats associated with them, such as health, strength, and other qualities. The `StateManager` creates a `WorldStage` upon its instantiation, and its passed from various method to method as a means to keep the game updated to where entities are positioned, and how they interact with each other. The `WorldStage` implementation has no intelligence of its own - it simply accepts or rejects the positioning of `Actor`s, and gives references to those `Actor`s should the need arise. The actual AI of the world is handled in the `AIPackageManager`, which will be described shortly. 

        The `WorldStage` stores both player positions and monster positions. When a monster or player wants to move to a particular position, the world stage first checks to see if the position is currently being used. Although Unity does have 2D collision, the server handles positioning completely, as each grid can either be inhabited or uninhabited. Whether one is being used or not directly determines entity decision making and movement.

    - *`AIPackageManager`* - this system uses traits more than any other system in the game, primarily because its built to be as flexible as possible, as AI can have different implementations depending on what entity its supposed to represent.

        `AIPackageManager`s store a collection of `IndependentPackages` (contrasted to `DependentPackages`, which I meant to implement along with independent, but I ran out of time). These collection of packages represent all the ways in which an entity can react to any particular situation. Three function parameters are required to define a `IndependentPackage`:
        1. `req` - the requirements for the entity to activate this particular `IndependentPackage`. 
        2. `on_start` - the function that runs when this `DependentPackage` activates on the given entity.
        3. `step_next` - the next action the entity will take when it's `AIPackageManager` is updated.

        `IndependentPackage`s have a generic type associated with them, which implements the `?Sized` trait. This allows whatever type is used in the generic to be unsized, which makes implementing the `IndependentPackage` with traits straightforward.

        `IndependentPackage`s are run by `AIPackageManager`s (impl. in `/src/state/ai/ai_package_manager.rs`), which choose a particular package to run at any given instant. When choosing a new package, the `AIPackageManager` runs each `IndependentPackage`'s `req` function on the associated entity. Any package which can run is added to a pool, and one from that pool is chosen, with weight leaning towards packages of higher `pick_count`. This package continues to run, until it either runs out of `interval_time` (a `Duration`), or the package, while running, determines it cannot run anymore via some factor, and revokes its running status itself. The package manager then chooses a new package, and the cycle continues.

*`dungeon_crawler_server tests`* (`./tests/event_handler_tests.rs`)
- `test-new-player` - tests that when a client sends a `Hello` request to the server, the server sends an appropriate `Welcome` packet in return.

Although I've only implemented 1 test for the `dungeon_crawler_server`, I wanted to note that the majority of my testing for this project was through testing the game itself. The majority of the functionality didn't have any real relevance, and the game logic itself is difficult to visualise without a client representing the data on screen. Because of this, I opted in for integration testing, rather than unit testing. I realize that that's probably a cop-out, and I understand if I lose some points on this :)<br><br>

## License

This project is licensed under MIT (https://choosealicense.com/licenses/mit/)