extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};
mod player;

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
    let mut Sprites = Vec::new();
    Sprites.push(player::sprite::Sprite {
        x: 0,
        y: 200,
        w: 300,
        h: 20,
        color: Color::RGBA(0, 0, 255, 0),
    });
    Sprites.push(player::sprite::Sprite {
        x: 300,
        y: 180,
        w: 30,
        h: 20,
        color: Color::RGBA(0, 0, 255, 0),
    });
    Sprites.push(player::sprite::Sprite {
        x: 400,
        y: 250,
        w: 100,
        h: 20,
        color: Color::RGBA(0, 0, 255, 0),
    });
    let mut player: player::Player = player::Player {
        sprite: player::SpriteWithFloat 
        { x: 50.0, y: 0.0, w: 10, h: 10, color: Color::RGBA(0, 0, 255, 0) },
        force_x: 0.1,
        bounce: 0.4,
        min_force: 0.03,
        force_y: 0.0,
        max_force_x: 10.0,
        max_gravity: 10.0,
        speed: 0.1,
        jump_force: 4.0,
        gravity: 0.1,
        has_jump: false,
        colliders: Sprites,
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
            player.move_right()
        }
        if keys.contains(&Keycode::Left) {
            player.move_left();
        }
        if keys.contains(&Keycode::Space) && !old_keys.contains(&Keycode::Space) {
            player.jump();
        }
        old_keys = keys;
        player.update();
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                for collider in player.colliders.iter() {
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
                texture_canvas.set_draw_color(player.sprite.color);
                texture_canvas
                .fill_rect(Rect::new(player.sprite.x as i32, player.sprite.y as i32, player.sprite.w, player.sprite.h))
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