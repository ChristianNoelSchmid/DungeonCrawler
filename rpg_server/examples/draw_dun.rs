use rpg_server::dungeon::inst::Dungeon;

fn main() {
    let dun = Dungeon::new(30, 50);
    println!("{:?}", dun);
}
