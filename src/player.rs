use graphics::Context;
use opengl_graphics::GlGraphics;
use vecmath::*;

use world;
use physics;

pub struct PlayerSystem;

impl world::System for PlayerSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {

        for entity in entities.iter() {
            if let (Some(player_id), Some(body_id)) = (entity.player_controller, entity.dynamic_body) {
                {
                    // Update state ..
                    update_state(entity, components, entities);
                }

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

fn update_state(player_entity: &world::Entity, components: &mut world::Components, entities: &Vec<world::Entity>) {

    use world::PlayerState;

    let new_state = get_state(player_entity, components, entities);

    let sprite = &mut components.sprite_renderer.get_mut(player_entity.sprite_renderer.unwrap()).sprite;

    match new_state {
        PlayerState::Flying => { sprite.set_rotation(0.0) },
        PlayerState::OnLeftWall => { sprite.set_rotation(90.0) },
        PlayerState::OnRightWall => { sprite.set_rotation(270.0) },
        PlayerState::OnCeiling => { sprite.set_rotation(180.0) },
        PlayerState::OnFloor => { sprite.set_rotation(0.0) },
    }
}

fn get_state(player_entity: &world::Entity, components: &world::Components, entities: &Vec<world::Entity>) -> world::PlayerState {

    let player = components.player_controller.get(player_entity.player_controller.unwrap());
    let position = components.position.get(player_entity.position.unwrap());
    let ground_check = &player.ground_check;

    // find all intersections of ground_check with static geometry
    let mut neighbouring_tiles = Vec::new();

    for entity_2 in entities.iter() {

        if let (Some(collider_2_id), Some(position_2_id)) = (entity_2.collider, entity_2.position) {

            // Don't check for collisions with self!
            if player_entity.collider.unwrap() == collider_2_id {
                continue;
            }

            let collider_2 = components.collider.get(collider_2_id);
            let position_2 = components.position.get(position_2_id);

            if physics::aabb_intersect(ground_check, position, collider_2, position_2) {
                neighbouring_tiles.push(position_2.clone());
            }
        }
    }

    if neighbouring_tiles.len() > 0 {

        let up = [0.0, -1.0];
        let down = [0.0, 1.0];
        let left = [-1.0, 0.0];
        let right = [1.0, 0.0];

        let directions = vec![
            (world::PlayerState::OnFloor, down),
            (world::PlayerState::OnCeiling, up),
            (world::PlayerState::OnRightWall, right),
            (world::PlayerState::OnLeftWall, left),
        ];

        let sum = neighbouring_tiles.iter().fold([0.0, 0.0], |v, p| [v[0] + p.x, v[1] + p.y]);
        let avg = vec2_scale(sum, 1.0 / neighbouring_tiles.len() as f32);

        let wall_to_player = vec2_normalized(vec2_sub([position.x, position.y], avg));

        let (state, _) = directions.iter()
            .map(|&(ref state, dir)| (state, vec2_len(vec2_sub(wall_to_player, dir))))
            .max_by(|&(_, dist)| (dist * 10000.0) as i32)
            .unwrap();

        return state.clone();

    }
    world::PlayerState::Flying
}

