use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::error::Error;
mod player;

struct Level<'a> {
    blocks: Vec<player::sprite::Sprite>,
    enemies: Vec<player::sprite::Sprite>,
    goal: player::sprite::Sprite,
    player: &'a player::Player,
    win: Box<dyn Fn(&mut Level)>,
    next : Option<Box<Level<'a>>>,
}
fn load_level(path: &str) -> Result<Box<Level>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let level: Level = serde_json::from_str(&contents).expect("wrong format");
    Ok(Box::new(level))
}
impl Level<'_>{
    fn win (&mut self) {
        if(self.next.is_some()){
             let newLevel = self.next.unwrap();
            self.player.sprite.x = 0.0;
            self.player.sprite.y = 0.0;
            self.player.force_y = 0.0;
            self.player.force_x = 0.0;
            self.blocks = newLevel.blocks;
            self.enemies = newLevel.enemies;
            self.goal = newLevel.goal;
            self.player = newLevel.player;
            self.win = newLevel.win;
            self.next = newLevel.next;
        } else{
            println!("You win!");
        }
    }
}