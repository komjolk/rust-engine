use sdl2::pixels::Color;
pub mod sprite;

pub struct SpriteWithFloat {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
    pub color: Color,
    }

    pub struct Player{
        pub sprite : SpriteWithFloat,
        pub force_y : f64,
        pub force_x: f64,
        pub max_gravity: f64,
        pub max_force_x: f64,
        pub speed : f64,
        pub jump_force : f64,
        pub friction : f64,
        pub gravity: f64,
        pub has_jump: bool,
        pub colliders : Vec<sprite::Sprite>,
    }

    impl Player {
        pub fn gravity(&mut self) {
            let mut on_ground = false;
            for colliders in self.colliders.iter() {
                if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                    && self.sprite.x < colliders.x as f64 + colliders.w as f64
                    && self.sprite.y + 1.1 +  self.sprite.h as f64  > colliders.y  as f64
                    && self.sprite.y < colliders.y as f64 + colliders.h as f64
                {
                    on_ground = true;
                }
            }

            if on_ground{
                if self.force_y > 0.0{
                    self.force_y = 0.0;
                }
                if self.force_x > 0.0{
                    self.force_x -= self.friction;
                    if self.force_x < 0.0{
                        self.force_x = 0.0;
                    }
                } else if self.force_x < 0.0 {
                    self.force_x += self.friction;
                    if self.force_x > 0.0{
                        self.force_x = 0.0;
                    }
                }
                self.has_jump = true;
                return;
            }
                self.has_jump = false;
            
            if self.force_y < self.max_gravity {
                self.force_y += self.gravity;
            }
        }
    }
    impl Player {
        pub fn update(&mut self){
            self.gravity();
            self.sprite.y += self.force_y;
            for colliders in self.colliders.iter() {
                if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                    && self.sprite.x < colliders.x as f64 + colliders.w as f64
                    && self.sprite.y + 1.0 + self.sprite.h as f64  > colliders.y  as f64
                    && self.sprite.y < colliders.y as f64 + colliders.h as f64
                {
                    print!("1 , \n");
                    self.sprite.y = colliders.y as f64 - self.sprite.h as f64 - 1.0;
                    self.force_y = 0.0;
                }
                if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                    && self.sprite.x < colliders.x as f64 + colliders.w as f64
                    && self.sprite.y + self.sprite.h as f64  > colliders.y  as f64
                    && self.sprite.y - 1.0 < colliders.y as f64 + colliders.h as f64
                {
                    print!("2, \n");
                    self.sprite.y = colliders.y as f64 - 1.0;
                    self.force_y = 0.0;
                }
            }
            self.sprite.x += self.force_x;
            for colliders in self.colliders.iter() {

                if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                    && self.sprite.x - 1.0 < colliders.x as f64 + colliders.w as f64
                    && self.sprite.y + self.sprite.h as f64  > colliders.y  as f64
                    && self.sprite.y < colliders.y as f64 + colliders.h as f64
                {
                    print!("3, \n");
                    self.sprite.x = colliders.x as f64 + colliders.w as f64 + 1.0;
                    self.force_x = 0.0;
                }
                if self.sprite.x + 1.0 + self.sprite.w as f64 > colliders.x as f64
                    && self.sprite.x < colliders.x as f64 + colliders.w as f64
                    && self.sprite.y + self.sprite.h as f64  > colliders.y  as f64
                    && self.sprite.y  < colliders.y as f64 + colliders.h as f64
                {
                    print!("4, \n");
                    self.sprite.x = colliders.x as f64 - 1.0;
                    self.force_x = 0.0;
                }
            }
        }
        pub fn move_left(&mut self) {
            if self.force_x > -self.max_force_x  {
                    self.force_x -= self.speed;
            }
            
        }
        pub fn move_right(&mut self) {
            if self.force_x > -self.max_force_x  {
                self.force_x += self.speed;
            }
        }
        pub fn jump(&mut self){
            if  self.has_jump {

                    self.has_jump = false;
                    self.force_y -= self.jump_force;

            }
        }
    }