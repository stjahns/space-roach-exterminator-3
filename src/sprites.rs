use graphics::Context;
use opengl_graphics::GlGraphics;
use time;

use world;

pub struct SpriteSystem;

impl world::System for SpriteSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        for entity in entities.iter() {
            if let (Some(s_id), Some(p_id)) = (entity.sprite_renderer, entity.position) {
                let sprite_renderer = components.sprite_renderer.get_mut(s_id);
                let position = components.position.get(p_id);

                // Update position
                sprite_renderer.sprite.set_position(position.x as f64, position.y as f64);

                // Update animation frame if animated
                if let Some(a_id) = entity.sprite_animator {
                    let sprite_animator = components.sprite_animator.get(a_id);
                    let frame = sprite_animator.get_frame(time::precise_time_s());
                    sprite_renderer.sprite.set_src_rect(frame);
                }

                // Draw
                sprite_renderer.sprite.draw(context.view, gl);
            }
        }
    }
}
