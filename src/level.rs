use std::{error::Error, fs, vec};
pub mod levelSettings;
pub mod player;

 struct Level{
    pub blocks: Vec<player::sprite::Sprite>,
    pub enemies: Vec<player::sprite::Sprite>,
    pub goal: player::sprite::Sprite,
    pub player:  player::Player,

}
pub struct LevelList{
    level : Vec<Level>,
    curIndex : u32,
    pub curLevel : Level
    
}
fn createTempLevel(settings: levelSettings::LevelSettings) -> Level{
    let  blocks = Vec::new();
    let enemies = Vec::new();
    let goal = player::sprite::Sprite {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
        color: sdl2::pixels::Color::RGB(0, 0, 0),
    };
    let player = player::Player {
        sprite: player::SpriteWithFloat {
            x: settings.player.x,
            y: settings.player.y,
            w: settings.player.w,
            h: settings.player.h,
            color: settings.player.color,
        },
        bounce: settings.bounce,
        force_y: 0.0,
        force_x: 0.0,
        max_gravity: settings.max_gravity,
        max_force_x: settings.max_force_x,
        min_force: settings.min_force,
        speed: settings.speed,
        jump_force: settings.jump_force,
        friction: settings.friction,
        gravity: settings.gravity,
        has_jump: false,
        colliders: Vec::new(),
        enemies: Vec::new(),
        goal: goal,
        win: Box::new(|_| {
            println!("You win!");
        }),
    };
    return Level {
        blocks,
        enemies,
        goal,
        player: player,
    };
}
pub fn load_level(
    path: &str,
    settings: levelSettings::LevelSettings,
) -> Result<LevelList, Box<dyn Error>> {
    let map_string = fs::read_to_string(path)?;
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();
    let mut curLevel: Level = createTempLevel(settings);
    let mut levels: LevelList = LevelList { level: Vec::new(), curIndex:0, curLevel:curLevel };

    for row in rows.iter() {
        let firstChar = row.chars().nth(0).unwrap();
        match firstChar {
            's' => {
                levels.level.push(curLevel);
               curLevel = createTempLevel(settings)
            }
           
            'e' => {
                return Ok(levels)
            }
            'b' => {              
                    let values: Vec<&str> =
                        map_string.trim().split(' ').map(|x| x.trim()).collect();

                    curLevel.blocks.push(player::sprite::Sprite {
                        x: values[0].parse::<i32>().unwrap(),
                        y: values[1].parse::<i32>().unwrap(),
                        w: values[3].parse::<u32>().unwrap(),
                        h: values[4].parse::<u32>().unwrap(),
                        color: settings.blockColor,
                    });

                
            }
            'g' => {
                let values: Vec<&str> =
                map_string.trim().split(' ').map(|x| x.trim()).collect();

            curLevel.goal = player::sprite::Sprite {
                x: values[0].parse::<i32>().unwrap(),
                y: values[1].parse::<i32>().unwrap(),
                w: values[3].parse::<u32>().unwrap(),
                h: values[4].parse::<u32>().unwrap(),
                color: settings.goalColor,
            };
            }
            'd' => {
                let values: Vec<&str> =
                map_string.trim().split(' ').map(|x| x.trim()).collect();
                unsafe{
            curLevel.enemies.push(player::sprite::Sprite {
                x: values[0].parse::<i32>().unwrap(),
                y: values[1].parse::<i32>().unwrap(),
                w: values[3].parse::<u32>().unwrap(),
                h: values[4].parse::<u32>().unwrap(),
                color: settings.enemyColor,
            });
        }
            }
            _ => {
                panic!()
            }
        }
    
}
    Ok(levels)
}
impl LevelList {
    fn win(&mut self) {
        if self.curIndex < self.level.len().try_into().unwrap() {
            self.curLevel = self.level[self.curIndex as usize];
        } else {
            print!("won")
        }
    }
    
}
