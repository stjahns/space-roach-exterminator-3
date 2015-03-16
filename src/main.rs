#![feature(old_path)]

#[macro_use]

extern crate ecs;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate uuid;

use std::cell::RefCell;
use std::rc::Rc;

use ecs::{World, BuildData};
use sprite::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};

use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct SpriteRenderer {
    sprite_id: Uuid,
}

impl SpriteRenderer {
    pub fn from_texture_region(scene: &mut Scene<Texture>, texture: Rc<Texture>, region: [i32; 4]) -> SpriteRenderer {
        let mut sprite = Sprite::from_texture(texture.clone());
        sprite.set_src_rect(region);
        let id = scene.add_child(sprite);
        SpriteRenderer { sprite_id: id }
    }

    pub fn update_position(&self, scene: &mut Scene<Texture>, position: Position) {
        if let Some(sprite) = scene.child_mut(self.sprite_id.clone()) {
            sprite.set_position(position.x as f64, position.y as f64);
        }
    }
}

components! {
    SRComponents {
        #[hot] position: Position,
        #[hot] sprite_renderer: SpriteRenderer,
    }
}

systems! {
    SRSystems<SRComponents, ()>;
}

fn main() {

    let (width, height) = (300, 300);
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

    let mut scene = Scene::new();
    let player_sprite_sheet = Rc::new(Texture::from_path(&Path::new("./assets/player.png")).unwrap());

    let mut world = World::<SRSystems>::new();

    let player_entity = world.create_entity(
        |entity: BuildData<SRComponents>, data: &mut SRComponents| {
            data.position.add(&entity, Position { x: width as f32 / 2.0, y: height as f32 / 2.0 });
            data.sprite_renderer.add(&entity, SpriteRenderer::from_texture_region(
                &mut scene,
                player_sprite_sheet.clone(),
                [0, 0, 32, 32])
            );

            // TODO physics?
            // TODO player controller?
        }
    );

    let ref mut gl = GlGraphics::new(opengl);
    let window = RefCell::new(window);

    for e in piston::events(&window) {
        use piston::event::{ RenderEvent };

        scene.event(&e);

        if let Some(args) = e.render_args() {
            use graphics::*;

            world.with_entity_data(&player_entity, |entity, data| {
                data.sprite_renderer[entity].update_position(&mut scene, data.position[entity]);
            });

            gl.draw([0, 0, args.width as i32, args.height as i32], |context, gl| {
                graphics::clear([0.3, 0.3, 0.3, 1.0], gl);
                scene.draw(context.view, gl);
            });
        }
    }
}
