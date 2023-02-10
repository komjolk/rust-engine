extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use std::collections::HashSet;
use std::time::Duration;

struct Sprite {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: Color,
}
struct Object{
    sprite : Sprite,
    extray: f64,
    extrax: f64,
    colliders : Vec<Sprite>,
}

trait Gravity {
    fn gravity(&mut self);
}
impl Gravity for Object {
    fn gravity(&mut self) {
        let mut on_ground = false;
        for collider in self.colliders.iter() {
            if self.sprite.x + self.sprite.w as i32 > collider.x
                && self.sprite.x < collider.x + collider.w as i32
                && self.sprite.y + self.sprite.h as i32 > collider.y
                && self.sprite.y < collider.y + collider.h as i32
            {
                on_ground = true;
            }
        }
        if on_ground{
            self.extray = 0.0;
            return;
        }
        self.extray += 0.01;
        if self.extray > 1.0 {
            self.extray = 0.0;
            self.sprite.y += 1;
        }
    }
}
trait Move {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn jump(&mut self);
}
impl Move for Object {
    fn move_left(&mut self) {
        let mut is_clear_to_left = true;
        for collider in self.colliders.iter() {
            if self.sprite.x < collider.x + collider.w as i32
            {
                is_clear_to_left = false;
            }
        }
        if is_clear_to_left{
            self.sprite.x -= 1;
        }
    }
    fn move_right(&mut self) {
        let mut is_clear_to_right = true;
        for collider in self.colliders.iter() {
            if self.sprite.x + self.sprite.w as i32 > collider.x
            {
                is_clear_to_right = false;
            }
        }
        if is_clear_to_right {
            self.sprite.x += 1;
        }
    }
    fn jump(&mut self){
        let mut is_clear_to_top = true;
        for collider in self.colliders.iter() {
            if self.sprite.y + self.sprite.h as i32 > collider.y
            {
                is_clear_to_top = false;
            }
        }
        if is_clear_to_top {
            self.sprite.y -= 10;
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
        x: 100,
        y: 200,
        w: 100,
        h: 20,
        color: Color::RGBA(0, 0, 255, 0),
    });
    let mut sprite: Object = Object {
        sprite: Sprite {
            x: 100,
            y: 100,
            w: 10,
            h: 10,
            color: Color::RGBA(0, 255, 255, 0),
        },
        extray: 0.0,
        extrax: 0.0,
        colliders: sprites,
    };

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
        sprite.gravity();
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
                .fill_rect(Rect::new(sprite.sprite.x, sprite.sprite.y, sprite.sprite.w, sprite.sprite.h))
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