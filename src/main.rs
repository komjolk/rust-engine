extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};
mod level;

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

    let interval = Duration::from_millis(10);
    let mut next_time = Instant::now() + interval;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 800, 600)
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut old_keys: HashSet<Keycode> = HashSet::new();
    let settings = level::levelSettings::LevelSettings{
        player: level::levelSettings::SpriteWithFloat{
            x:0.0,
            y:0.0,
            w:10,
            h:10,
            color: sdl2::pixels::Color{r : 0, g : 0, b : 0, a : 0},
        },
        bounce: 0.5,
        max_gravity: 2.0,
        max_force_x: 2.0,
        min_force: 0.01,
        speed: 0.02,
        jump_force: 0.1,
        friction: 0.01,
        gravity: 0.1,
        blockColor: sdl2::pixels::Color{r : 0, g : 0, b : 0, a : 0},
        goalColor : sdl2::pixels::Color{r : 0, g : 0, b : 0, a : 0},
        enemyColor : sdl2::pixels::Color{r : 0, g : 0, b : 0, a : 0},
    };
    let level = level::load_level("level.txt", settings).unwrap();

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
            level.curLevel.player.move_right()
        }
        if keys.contains(&Keycode::Left) {
            level.curLevel.player.move_left();
        }
        if keys.contains(&Keycode::Space) && !old_keys.contains(&Keycode::Space) {
            level.curLevel.player.jump();
        }
        old_keys = keys;
        level.curLevel.player.update();
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                for collider in level.curLevel.blocks.iter() {
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
                for enemy in level.curLevel.enemies.iter() {
                    texture_canvas.set_draw_color(enemy.color);
                    texture_canvas
                        .fill_rect(Rect::new(
                            enemy.x,
                            enemy.y,
                            enemy.w,
                            enemy.h,
                        ))
                        .expect("could not fill rect");
                }         
                texture_canvas.set_draw_color(level.curLevel.goal.color);
                texture_canvas
                .fill_rect(Rect::new(level.curLevel.goal.x, level.curLevel.goal.y, level.curLevel.goal.w, level.curLevel.goal.h))
                .expect("could not fill rect");
                texture_canvas.set_draw_color(level.curLevel.player.sprite.color);
                texture_canvas
                .fill_rect(Rect::new(level.curLevel.player.sprite.x as i32, level.curLevel.player.sprite.y as i32, level.curLevel.player.sprite.w, level.curLevel.player.sprite.h))
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