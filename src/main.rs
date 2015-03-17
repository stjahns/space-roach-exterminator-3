#![feature(old_path)]

#[macro_use]

extern crate ecs;
extern crate graphics;
extern crate input;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate time;
extern crate uuid;

use std::cell::RefCell;
use std::rc::Rc;

use ecs::system::{
    EntityProcess,
    EntitySystem,
};

use ecs::{
    World,
    BuildData,
    System,
    EntityIter,
    Entity,
    DataHelper
};

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};

use uuid::Uuid;

use sprite::*;
use time::*;

use input::Button::{Keyboard, Mouse};
use input::keyboard::Key;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct InputState {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

pub struct UpdateContext {
    pub delta_time: f64,
    pub input: InputState,
}

pub struct UpdateContextComponent {
    pub context: Rc<RefCell<UpdateContext>>,
}

pub struct SpriteRenderer {
    pub sprite: Sprite<Texture>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpriteAnimation {
    frame_size: [i32; 2],
    frame_duration: f64,
    frames: Vec<(i32, i32)>,
}

pub struct SpriteAnimator {
    animation: SpriteAnimation,
    start_time: f64,
}

impl SpriteAnimator {
    pub fn get_frame(&self, time: f64) -> [i32; 4] {
        let elapsed_time = time - self.start_time;
        let frame_index = ((elapsed_time / self.animation.frame_duration) as usize) % self.animation.frames.len();
        let (x, y) = self.animation.frames[frame_index];

        let frame = [
            x * self.animation.frame_size[0],
            y * self.animation.frame_size[1],
            (x + 1) * self.animation.frame_size[0],
            (y + 1) * self.animation.frame_size[1],
        ];
        frame
    }
}

impl SpriteRenderer {
    pub fn from_texture_region(texture: Rc<Texture>, region: [i32; 4]) -> SpriteRenderer {
        let mut sprite = Sprite::from_texture(texture.clone());
        sprite.set_src_rect(region);
        SpriteRenderer { sprite: sprite }
    }
}

components! {
    SRComponents {
        #[hot] position: Position,
        #[hot] update_context: UpdateContextComponent,
        #[hot] sprite_renderer: SpriteRenderer,
        #[hot] sprite_animator: SpriteAnimator,
    }
}


systems! {
    SRSystems<SRComponents, ()> {
        sprites: EntitySystem<SpriteProcess> = EntitySystem::new(
             SpriteProcess,
             aspect!(<SRComponents> all: [update_context, position, sprite_renderer, sprite_animator])
        )
    }
}

pub struct SpriteProcess;

impl System for SpriteProcess { type Components = SRComponents; type Services = (); }

impl EntityProcess for SpriteProcess {
    fn process(
        &mut self,
        entities: EntityIter<SRComponents>,
        data: &mut DataHelper<SRComponents, ()>)
    {
        for e in entities {

            // TODO move elsewhere ...

            if data.update_context[e].context.borrow().input.left {
                data.position[e].x -= 1.0;
            }

            if data.update_context[e].context.borrow().input.right {
                data.position[e].x += 1.0;
            }

            if data.update_context[e].context.borrow().input.up {
                data.position[e].y -= 1.0;
            }

            if data.update_context[e].context.borrow().input.down {
                data.position[e].y += 1.0;
            }

            // TODO - convert from world-space to screen-space ...

            let Position {x, y} = data.position[e];
            data.sprite_renderer[e].sprite.set_position(x as f64, y as f64);
            // TODO not all sprites will necessarily be animated ...
            let frame = data.sprite_animator[e].get_frame(time::precise_time_s());
            data.sprite_renderer[e].sprite.set_src_rect(frame);
        }
    }

}


fn spawn_player(world: &mut World<SRSystems>, update_context: Rc<RefCell<UpdateContext>>) -> Entity {

    let player_sprite_sheet = Rc::new(Texture::from_path(&Path::new("./assets/player.png")).unwrap());

    let walk_anim = SpriteAnimation {
        frames: vec![(0,0), (1,0), (2,0), (3,0)],
        frame_size: [32, 32],
        frame_duration: 0.25,
    };

    let walk_anim_aim_up = SpriteAnimation {
        frames: vec![(0,1), (1,1), (2,1), (3,1)],
        frame_size: [32, 32],
        frame_duration: 0.25,
    };

    world.create_entity( |entity: BuildData<SRComponents>, data: &mut SRComponents| {
            data.update_context.add(&entity, UpdateContextComponent { context: update_context.clone() });
            data.position.add(&entity, Position { x: 100.0, y: 100.0 });
            data.sprite_renderer.add(&entity, SpriteRenderer::from_texture_region(
                player_sprite_sheet.clone(),
                [0, 0, 32, 32])
            );
            data.sprite_animator.add(&entity, SpriteAnimator { animation: walk_anim.clone(), start_time: time::precise_time_s() });

            // TODO physics?
            // TODO player controller?
        }
    )

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

    let mut world = World::<SRSystems>::new();

    let mut update_context = Rc::new(RefCell::new(UpdateContext {
        delta_time: 0.0,
        input: InputState {
            left: false,
            right: false ,
            up: false,
            down: false ,
        },
    }));

    let player_entity = spawn_player(&mut world, update_context.clone());

    let ref mut gl = GlGraphics::new(opengl);
    let window = RefCell::new(window);

    for e in piston::events(&window) {
        use piston::event::{ RenderEvent, PressEvent, ReleaseEvent };

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

        if let Some(args) = e.render_args() {

            use graphics::*;

            world.update();

            gl.draw([0, 0, args.width as i32, args.height as i32], |context, gl| {
                graphics::clear([0.3, 0.3, 0.3, 1.0], gl);

                // TODO could iterate through all entities with SpriteRenderer components?
                world.with_entity_data(&player_entity, |entity, data| {
                    data.sprite_renderer[entity].sprite.draw(context.view, gl);
                });
            });

        }
    }
}
