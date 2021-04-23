use rpg_server::dungeons::inst::Dungeon;

fn main() {
    let dun = Dungeon::new(30, 50);
    println!("{:?}", dun);
}
