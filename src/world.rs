use std::cell::RefCell;
use std::rc::Rc;
use sprite::*;
use opengl_graphics::Texture;
use id;

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

pub struct PlayerController {
    pub move_speed: f32,
}

#[secs(id)]
pub struct EntityPrototype {
    position: Position,
    sprite_renderer: SpriteRenderer,
    sprite_animator: SpriteAnimator,
    player_controller: PlayerController,
}
