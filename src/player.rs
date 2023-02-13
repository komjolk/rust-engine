struct Sprite_with_float {
    x: f64,
    y: f64,
    w: u32,
    h: u32,
    color: Color,
}

pub struct Player{
    sprite : Sprite_with_float,
    force_y : f64,
    force_x: f64,
    max_gravity: f64,
    max_force_x: f64,
    speed : f64,
    jump_force : f64,
    friction : f64,
    gravity: f64,
    has_jump: bool,
    colliders : Vec<Sprite>,
}

trait Gravity {
    fn gravity(&mut self);
}
impl Gravity for Player {
    fn gravity(&mut self) {
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
trait Move {
    pub fn move_left(&mut self);
    pub fn move_right(&mut self);
    pub fn jump(&mut self);
    pub fn update(&mut self);
}
impl Move for Player {
    fn update(&mut self){
        self.gravity();
        self.sprite.y += self.force_y;
        self.sprite.x += self.force_x;
        for colliders in self.colliders.iter() {
            if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                && self.sprite.x < colliders.x as f64 + colliders.w as f64
                && self.sprite.y + 1.0 + self.sprite.h as f64  > colliders.y  as f64
                && self.sprite.y < colliders.y as f64 + colliders.h as f64
            {
                print!("1 , \n");
                self.sprite.y = colliders.y as f64 - self.sprite.h as f64 - 1.0;
            }
            if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                && self.sprite.x < colliders.x as f64 + colliders.w as f64
                && self.sprite.y + self.sprite.h as f64  > colliders.y  as f64
                && self.sprite.y - 1.0 < colliders.y as f64 + colliders.h as f64
            {
                print!("2, \n");
                self.sprite.y = colliders.y as f64 - 1.0;
            }
            if self.sprite.x + self.sprite.w as f64 > colliders.x as f64
                && self.sprite.x - 1.0 < colliders.x as f64 + colliders.w as f64
                && self.sprite.y + self.sprite.h as f64  > colliders.y  as f64
                && self.sprite.y < colliders.y as f64 + colliders.h as f64
            {
                print!("3, \n");
                self.sprite.x = colliders.x as f64 + colliders.w as f64 + 1.0;
            }
            if self.sprite.x + 1.0 + self.sprite.w as f64 > colliders.x as f64
                && self.sprite.x < colliders.x as f64 + colliders.w as f64
                && self.sprite.y + self.sprite.h as f64  > colliders.y  as f64
                && self.sprite.y  < colliders.y as f64 + colliders.h as f64
            {
                print!("4, \n");
                self.sprite.x = colliders.x as f64 - 1.0;
            }
        }
    }
    fn move_left(&mut self) {
        if self.force_x > -self.max_force_x  {
                self.force_x -= self.speed;
        }
        
    }
    fn move_right(&mut self) {
        if self.force_x > -self.max_force_x  {
            self.force_x += self.speed;
        }
    }
    fn jump(&mut self){
        if  self.has_jump {

                self.has_jump = false;
                self.force_y -= self.jump_force;

        }
    }
}
