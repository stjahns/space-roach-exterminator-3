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
use std::sync::mpsc;
use std::path::Path;

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};

use graphics::Context;

use time::*;

use input::Button::{Keyboard};
use input::keyboard::Key;

use std::fs::File;

use sprite::*;

use tiled::parse as tiled_parse;

mod world;
mod player;
mod sprites;

///
/// Load Tiled level, creating entities for each tile instance
///
fn init_level(data: &mut world::Components, entities: &mut Vec<world::Entity>) {

    let tmx_file = File::open(&Path::new("./assets/level.tmx")).unwrap();
    let map = tiled_parse(tmx_file).unwrap();

    println!("Map Loaded! Width: {}, Height: {}", map.width, map.height);

    // TODO - tileset-specific - keep in tile representation
    let mut tile_width = 32;
    let mut tile_height = 32;

    // Load tileset textures and create shared SpriteRenderer components for each
    // TODO modify to handle tilesheets - need some representation of a texture subregion
    // TODO is it OK for entities to share components? What happens when we destroy an entity?
    // Maybe just have multiple sprite renderers that share the same Sprite instance

    let mut sprite_renderer_ids = Vec::new();

    for tileset in map.tilesets.iter() {
        for tileset_image in tileset.images.iter() {
            let path_string = format!("./assets/{}", tileset_image.source);
            let path = Path::new(&path_string[..]);
            let texture = Rc::new(Texture::from_path(&path).unwrap());
            let sprite_renderer = world::SpriteRenderer::from_texture_region(
                texture.clone(),
                [0, 0, 32, 32],
            );
            sprite_renderer_ids.push(data.sprite_renderer.add(sprite_renderer));
        }
    }

    for layer in map.layers.iter() {
        for (column, tile_column) in layer.tiles.iter().enumerate() {
            for (row, tile) in tile_column.iter().enumerate() {

                if *tile < 1u32 {
                    continue;
                }

                let tile_entity = world::Entity {
                    position: Some(data.position.add(world::Position {
                        x: (row * tile_width) as f32,
                        y: (column * tile_height) as f32,
                    })),
                    sprite_renderer: Some(sprite_renderer_ids[(tile - 1) as usize]),
                    sprite_animator: None,
                    player_controller: None,
                    camera_target: None,

                };

                // TODO add collision for each tile ...

                entities.push(tile_entity);

            }
        }
    }
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

    let player_controller = world::PlayerController { move_speed: 1.0 };

    // TODO physics

    world::Entity {
        position: Some(data.position.add(world::Position { x: 0.0, y: 0.0 })),
        sprite_renderer: Some(data.sprite_renderer.add(sprite_renderer)),
        sprite_animator: Some(data.sprite_animator.add(sprite_animator)),
        player_controller: Some(data.player_controller.add(player_controller)),
        camera_target: Some(data.camera_target.add(world::CameraTarget)),
    }
}

fn get_camera_context(world: &world::World, viewport_width: u32, viewport_height: u32) -> Context {

    let mut camera = [0.0, 0.0]; // position of camera relative to world origin (in pixels)
    // Set camera to Position of first entity with a CameraTarget component
    for entity in world.entities.iter() {
        match (entity.position, entity.camera_target) {
            (Some(position_id), Some(target_id)) => {
                let position = world.data.position.get(position_id);
                camera[0] = position.x as f64;
                camera[1] = position.y as f64;
                break;
            },
            _ => {},
        }
    }

    // Y axis needs to be down, otherwise `sprite` gets messed...
    // TODO consider flipping + scaling every sprite?
    // Also would be nice to have 1 world unit = 1 tile, instead of pixel

    let sx = 2.0 / viewport_width as f64;
    let sy = 2.0 / viewport_height as f64;
    let tx = -camera[0] * 2.0 / viewport_width as f64;
    let ty = -camera[1] * 2.0 / viewport_height as f64;
    let mat = [[ sx,  0.0, tx],
        [ 0.0, -sy, -ty]];

    let mut context = Context::new();
    context.view = mat;
    context.transform = mat;
    context

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

    init_level(&mut world.data, &mut world.entities);
    let player_entity = spawn_player(&mut world.data);
    world.entities.push(player_entity);

    let ref mut gl = GlGraphics::new(opengl);
    let window = RefCell::new(window);

    let mut control_state = world::ControlState {
        left: false,
        right: false,
        up: false,
        down: false,
    };

    let mut systems: Vec<Box<world::System>> = vec![
        Box::new(player::PlayerSystem),
        Box::new(sprites::SpriteSystem),
    ];

    for e in piston::events(&window) {
        use piston::event::{ RenderEvent, PressEvent, ReleaseEvent };
        use world::System;

        e.press(|button| {
            match button {
                Keyboard(Key::Left) => control_state.left = true,
                Keyboard(Key::Right) => control_state.right = true,
                Keyboard(Key::Up) => control_state.up = true,
                Keyboard(Key::Down) => control_state.down = true,
                _ => {}
            }
        });

        e.release(|button| {
            match button {
                Keyboard(Key::Left) => control_state.left = false,
                Keyboard(Key::Right) => control_state.right = false,
                Keyboard(Key::Up) => control_state.up = false,
                Keyboard(Key::Down) => control_state.down = false,
                _ => {}
            }
        });

        // TODO where should this go?
        for system in systems.iter_mut() {
            // TODO need some kind of delta time
            system.update(&control_state, &mut world.data, &mut world.entities);
        }

        if let Some(args) = e.render_args() {

            use graphics::*;

            gl.draw([0, 0, args.width as i32, args.height as i32], |_, mut gl| {
                graphics::clear([0.3, 0.3, 0.3, 1.0], gl);

                let context = get_camera_context(&world, args.width, args.height);

                // TODO - probably want sprite sorting orders ..
                // Currently just draw in order of creation ..

                for system in systems.iter_mut() {
                    system.render(&context, gl, &mut world.data, &mut world.entities);
                }
            });
        }
    }
}
