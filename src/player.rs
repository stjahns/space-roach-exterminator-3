use graphics::Context;
use opengl_graphics::GlGraphics;

use world;

pub struct PlayerSystem;

impl world::System for PlayerSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        for entity in entities.iter() {
            if let (Some(player_id), Some(position_id)) = (entity.player_controller, entity.position) {
                let player = components.player_controller.get(player_id);
                let position = components.position.get_mut(position_id);

                if control_state.up {
                    position.y -= player.move_speed;
                }

                if control_state.down {
                    position.y += player.move_speed;
                }

                if control_state.left {
                    position.x -= player.move_speed;
                }

                if control_state.right {
                    position.x += player.move_speed;
                }
            }
        }
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }

}
