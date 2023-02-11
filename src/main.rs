extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Sprite {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: Color,
}
struct Sprite_with_float {
    x: f64,
    y: f64,
    w: u32,
    h: u32,
    color: Color,
}

struct Object{
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
impl Gravity for Object {
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
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn jump(&mut self);
    fn update(&mut self);
}
impl Move for Object {
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

fn  main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 resource-manager demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let mut sprites = Vec::new();
    sprites.push(Sprite {
        x: 0,
        y: 200,
        w: 300,
        h: 20,
        color: Color::RGBA(0, 0, 255, 0),
    });
    sprites.push(Sprite {
        x: 400,
        y: 250,
        w: 100,
        h: 20,
        color: Color::RGBA(0, 0, 255, 0),
    });
    let mut sprite: Object = Object {
        sprite: Sprite_with_float 
        { x: 50.0, y: 0.0, w: 10, h: 10, color: Color::RGBA(0, 0, 255, 0) },
        force_x: 0.1,
        force_y: 0.0,
        max_force_x: 10.0,
        max_gravity: 10.0,
        speed: 0.1,
        jump_force: 4.0,
        gravity: 0.1,
        has_jump: false,
        colliders: sprites,
        friction: 0.01,
    };
    let interval = Duration::from_millis(10);
    let mut next_time = Instant::now() + interval;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 800, 600)
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut old_keys: HashSet<Keycode> = HashSet::new();

    'mainloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
    sleep(next_time - Instant::now());
    next_time += interval;
        let keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        if keys.contains(&Keycode::Right) {
            sprite.move_right()
        }
        if keys.contains(&Keycode::Left) {
            sprite.move_left();
        }
        if keys.contains(&Keycode::Space) && !old_keys.contains(&Keycode::Space) {
            sprite.jump();
        }
        old_keys = keys;
        sprite.update();
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                for collider in sprite.colliders.iter() {
                    texture_canvas.set_draw_color(collider.color);
                    texture_canvas
                        .fill_rect(Rect::new(
                            collider.x,
                            collider.y,
                            collider.w,
                            collider.h,
                        ))
                        .expect("could not fill rect");
                }
                texture_canvas.set_draw_color(sprite.sprite.color);
                texture_canvas
                .fill_rect(Rect::new(sprite.sprite.x as i32, sprite.sprite.y as i32, sprite.sprite.w, sprite.sprite.h))
                .expect("could not fill rect");
            })
            .map_err(|e| e.to_string())?;
            
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, 800, 600));
        canvas.clear();
        canvas.copy_ex(
            &texture,
            None,
            dst,
            0.0,
            Some(Point::new(800, 600)),
            false,
            false,
        )?;
        canvas.present();
    }

    Ok(())
}