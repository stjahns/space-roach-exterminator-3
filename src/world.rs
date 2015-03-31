use std::cell::RefCell;
use std::rc::Rc;
use sprite::*;
use opengl_graphics::{Texture, GlGraphics};
use graphics::Context;
use id;
use id::Id;
use vecmath::*;

use openal::al;

/// TODO move
#[derive(Clone, Debug, PartialEq)]
pub struct SpriteAnimation {
    pub frame_size: [i32; 2],
    pub frame_duration: f64,
    pub frames: Vec<(i32, i32)>,
}

/// Components

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct SpriteRenderer {
    pub sprite: Sprite<Texture>,
}

impl SpriteRenderer {
    pub fn from_texture_region(texture: Rc<Texture>, region: [i32; 4]) -> SpriteRenderer {
        let mut sprite = Sprite::from_texture(texture.clone());
        sprite.set_src_rect(region);
        SpriteRenderer { sprite: sprite }
    }
}

pub struct SpriteAnimator {
    pub animation: SpriteAnimation,
    pub start_time: f64,
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

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerState {
    Flying,
    OnFloor,
    OnCeiling,
    OnLeftWall,
    OnRightWall,
}

pub struct PlayerController {
    pub move_speed: f32,
    pub state: PlayerState,
    pub ground_check: AABBCollider,

    pub aim_direction: [f32; 2],

    pub idle_anim: SpriteAnimation,
    pub walk_anim: SpriteAnimation,

    pub walk_anim_aim_up: SpriteAnimation,
    pub walk_anim_aim_down: SpriteAnimation,
    pub walk_anim_aim_up_forward: SpriteAnimation,
    pub walk_anim_aim_down_forward: SpriteAnimation,

    pub idle_anim_aim_up: SpriteAnimation,
    pub idle_anim_aim_down: SpriteAnimation,
    pub idle_anim_aim_up_forward: SpriteAnimation,
    pub idle_anim_aim_down_forward: SpriteAnimation,

    pub jump_sound: al::Buffer,
    pub land_sound: al::Buffer,
}

pub struct CameraTarget;

pub struct DynamicBody {
    pub vx: f32,
    pub vy: f32,
}

pub struct AABBCollider {
    pub width: f32,
    pub height: f32,
}

pub struct AudioSource {
    pub source: al::Source,
    pub buffer: al::Buffer,
}

pub struct Weapon {
    pub fire_delay: f32,
    pub bullet_speed: f32,
    pub fire_timer: f32,
    pub fire_sound: al::Buffer,
    pub firing: bool,
    pub fire_direction: Vector2<f32>,
}

pub struct Bullet;

pub enum Event {
    Collision(Id<AABBCollider>, Id<AABBCollider>)
}

pub struct EventReceiver {
    pub event_queue: Vec<Event>,
}


#[secs(id)]
pub struct EntityPrototype {
    position: Position,
    sprite_renderer: SpriteRenderer,
    sprite_animator: SpriteAnimator,
    player_controller: PlayerController,
    camera_target: CameraTarget,
    collider: AABBCollider,
    dynamic_body: DynamicBody,
    audio_source: AudioSource,
    weapon: Weapon,
    bullet: Bullet,
    event_receiver: EventReceiver,
}

pub struct ControlState {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,

    pub aim_left: bool,
    pub aim_right: bool,
    pub aim_up: bool,
    pub aim_down: bool,
}

pub trait System {
    fn update(&mut self, &ControlState, &mut Components, &mut Vec<Entity>);
    fn render(&mut self, &Context, &mut GlGraphics, &mut Components, &mut Vec<Entity>);
}
