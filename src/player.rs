use graphics::Context;
use opengl_graphics::GlGraphics;
use vecmath::*;

use world;

pub struct PlayerSystem;

impl world::System for PlayerSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        for entity in entities.iter() {
            if let (Some(player_id), Some(body_id)) = (entity.player_controller, entity.dynamic_body) {
                let player = components.player_controller.get(player_id);
                let body = components.dynamic_body.get_mut(body_id);

                let mut velocity: Vector2<f32> = [0.0, 0.0];

                if control_state.up {
                    velocity = vec2_add(velocity, [0.0, -1.0]);
                }

                if control_state.down {
                    velocity = vec2_add(velocity, [0.0, 1.0]);
                }

                if control_state.left {
                    velocity = vec2_add(velocity, [-1.0, 0.0]);
                }

                if control_state.right {
                    velocity = vec2_add(velocity, [1.0, 0.0]);
                }

                if velocity != [0.0, 0.0] {
                    velocity = vec2_normalized(velocity);
                    velocity = vec2_scale(velocity, player.move_speed);
                }

                body.vx = velocity[0];
                body.vy = velocity[1];
            }
        }
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }

}
