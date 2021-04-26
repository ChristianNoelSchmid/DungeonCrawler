use rpg_server::dungeons::inst::Dungeon;

fn main() {
    let dun = Dungeon::new(20, 20);
    println!("{:?}", dun);
}
