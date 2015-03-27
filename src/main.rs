#![feature(collections, core, custom_attribute, plugin)]
#![plugin(secs)]

extern crate graphics;
extern crate id;
extern crate input;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate time;
extern crate uuid;
extern crate tiled;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};

use time::*;

use input::Button::{Keyboard};
use input::keyboard::Key;

use std::fs::File;

use sprite::*;

use tiled::parse as tiled_parse;

mod world;

/// TODO - level tiles should be entities
fn init_level() -> Vec<Sprite<Texture>> {
    let tmx_file = File::open(&Path::new("./assets/level.tmx")).unwrap();
    let map = tiled_parse(tmx_file).unwrap();

    println!("Map Loaded! Width: {}, Height: {}", map.width, map.height);

    // TODO - tileset-specific - keep in tile representation
    let mut tile_width = 32;
    let mut tile_height = 32;


    // Load tileset textures
    // TODO modify to handle tilesheets - need some representation of a texture subregion
    let mut tile_textures = Vec::new();
    for tileset in map.tilesets.iter() {
        for tileset_image in tileset.images.iter() {
            let path_string = format!("./assets/{}", tileset_image.source);
            let path = Path::new(&path_string[..]);
            let texture = Rc::new(Texture::from_path(&path).unwrap());
            tile_textures.push(texture);
        }
    }

    let mut tile_sprites = Vec::new();

    for layer in map.layers.iter() {
        for (column, tile_column) in layer.tiles.iter().enumerate() {
            for (row, tile) in tile_column.iter().enumerate() {

                if *tile < 1u32 {
                    continue;
                }

                let texture = &tile_textures[(tile - 1) as usize];
                let mut sprite = Sprite::from_texture(texture.clone());
                sprite.set_position((row * tile_width) as f64, (column * tile_height) as f64);
                tile_sprites.push(sprite);

                // TODO add physics for each tile ...

            }
        }
    }

    tile_sprites
}

fn spawn_player(data: &mut world::Components) -> world::Entity {

    let player_sprite_sheet = Rc::new(Texture::from_path(&Path::new("./assets/player.png")).unwrap());

    let walk_anim = world::SpriteAnimation {
        frames: vec![(0,0), (1,0), (2,0), (3,0)],
        frame_size: [32, 32],
        frame_duration: 0.25,
    };

    let walk_anim_aim_up = world::SpriteAnimation {
        frames: vec![(0,1), (1,1), (2,1), (3,1)],
        frame_size: [32, 32],
        frame_duration: 0.25,
    };

    let sprite_renderer = world::SpriteRenderer::from_texture_region(
        player_sprite_sheet.clone(),
        [0, 0, 32, 32],
    );

    let sprite_animator = world::SpriteAnimator {
        animation: walk_anim.clone(),
        start_time: time::precise_time_s(),
    };

    // TODO physics, player controller

    world::Entity {
        position: Some(data.position.add(world::Position { x: 0.0, y: 0.0 })),
        sprite_renderer: Some(data.sprite_renderer.add(sprite_renderer)),
        sprite_animator: Some(data.sprite_animator.add(sprite_animator)),
    }
}

fn main() {

    let (width, height) = (640, 480);
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
            title: "Space Roach Exterminator III".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut world = world::World::new();

    let level_sprites = init_level();
    let player_entity = spawn_player(&mut world.data);
    world.entities.push(player_entity);

    let ref mut gl = GlGraphics::new(opengl);
    let window = RefCell::new(window);

    for e in piston::events(&window) {
        use piston::event::{ RenderEvent, PressEvent, ReleaseEvent };

        /*
        e.press(|button| {
            match button {
                Keyboard(Key::Left) => update_context.borrow_mut().input.left = true,
                Keyboard(Key::Right) => update_context.borrow_mut().input.right = true,
                Keyboard(Key::Up) => update_context.borrow_mut().input.up = true,
                Keyboard(Key::Down) => update_context.borrow_mut().input.down = true,
                _ => {}
            }
        });

        e.release(|button| {
            match button {
                Keyboard(Key::Left) => update_context.borrow_mut().input.left = false,
                Keyboard(Key::Right) => update_context.borrow_mut().input.right = false,
                Keyboard(Key::Up) => update_context.borrow_mut().input.up = false,
                Keyboard(Key::Down) => update_context.borrow_mut().input.down = false,
                _ => {}
            }
        });
        */

        if let Some(args) = e.render_args() {

            use graphics::*;

            //world.update();

            gl.draw([0, 0, args.width as i32, args.height as i32], |context, gl| {
                graphics::clear([0.3, 0.3, 0.3, 1.0], gl);

                // TODO - need sprite sorting orders ..

                // Draw level
                for level_sprite in level_sprites.iter() {
                    level_sprite.draw(context.view, gl);
                }

                for entity in world.entities.iter_mut() {
                    if let (Some(s_id), Some(p_id)) = (entity.sprite_renderer, entity.position) {
                        let sprite_renderer = world.data.sprite_renderer.get_mut(s_id);
                        let position = world.data.position.get(p_id);

                        // Update position
                        sprite_renderer.sprite.set_position(position.x as f64, position.y as f64);

                        // Update animation frame if animated
                        if let Some(a_id) = entity.sprite_animator {
                            let sprite_animator = world.data.sprite_animator.get(a_id);
                            let frame = sprite_animator.get_frame(precise_time_s());
                            sprite_renderer.sprite.set_src_rect(frame);
                        }

                        // Draw
                        sprite_renderer.sprite.draw(context.view, gl);
                    }
                }

            });
        }
    }
}
