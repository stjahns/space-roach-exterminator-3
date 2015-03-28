use graphics::Context;
use opengl_graphics::GlGraphics;
use vecmath::*;

use world;
use physics;

pub struct PlayerSystem;

fn get_aim_direction(control_state: &world::ControlState) -> Option<Vector2<f32>> {

    let mut aim: Vector2<f32> = [0.0, 0.0];

    if control_state.aim_up {
        aim = vec2_add(aim, [0.0, -1.0]);
    }

    if control_state.aim_down {
        aim = vec2_add(aim, [0.0, 1.0]);
    }

    if control_state.aim_left {
        aim = vec2_add(aim, [-1.0, 0.0]);
    }

    if control_state.aim_right {
        aim = vec2_add(aim, [1.0, 0.0]);
    }

    if aim != [0.0, 0.0] {
        Some(vec2_normalized(aim))
    } else {
        None
    }
}

impl world::System for PlayerSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {

        use world::PlayerState;

        for entity in entities.iter() {
            if let (Some(player_id), Some(body_id), Some(animator_id)) = (entity.player_controller, entity.dynamic_body, entity.sprite_animator) {

                {
                    // Update orientation state ..
                    update_walk_state(entity, components, entities);
                }


                if let Some(aim_direction) = get_aim_direction(control_state) {
                    let player = components.player_controller.get_mut(player_id);
                    player.aim_direction = aim_direction;

                    // TODO shoot stuff

                }

                {
                    update_animation(entity, components, entities);
                }

                let player = components.player_controller.get(player_id);
                let body = components.dynamic_body.get_mut(body_id);

                println!("{:?}", player.state);

                if player.state != PlayerState::Flying {
                    let mut velocity: Vector2<f32> = [0.0, 0.0];

                    if control_state.move_up {
                        velocity = vec2_add(velocity, [0.0, -1.0]);
                    }

                    if control_state.move_down {
                        velocity = vec2_add(velocity, [0.0, 1.0]);
                    }

                    if control_state.move_left {
                        velocity = vec2_add(velocity, [-1.0, 0.0]);
                    }

                    if control_state.move_right {
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
    }
    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }

}

fn aim_up_anim(player: &world::PlayerController, speed: f32) -> world::SpriteAnimation {
    if speed > 0.1 {
        player.walk_anim_aim_up.clone()
    } else {
        player.idle_anim_aim_up.clone()
    }
}

fn aim_up_forward_anim(player: &world::PlayerController, speed: f32) -> world::SpriteAnimation {
    if speed > 0.1 {
        player.walk_anim_aim_up_forward.clone()
    } else {
        player.idle_anim_aim_up_forward.clone()
    }
}

fn aim_down_forward_anim(player: &world::PlayerController, speed: f32) -> world::SpriteAnimation {
    if speed > 0.1 {
        player.walk_anim_aim_down_forward.clone()
    } else {
        player.idle_anim_aim_down_forward.clone()
    }
}

fn aim_down_anim(player: &world::PlayerController, speed: f32) -> world::SpriteAnimation {
    if speed > 0.1 {
        player.walk_anim_aim_down.clone()
    } else {
        player.idle_anim_aim_down.clone()
    }
}

fn aim_forward_anim(player: &world::PlayerController, speed: f32) -> world::SpriteAnimation {
    if speed > 0.1 {
        player.walk_anim.clone()
    } else {
        player.idle_anim.clone()
    }
}

//
// TODO - replace this with some generic Animation State Machine
//
fn update_animation(player_entity: &world::Entity, components: &mut world::Components, entities: &Vec<world::Entity>) {

    use world::PlayerState;

    let player = components.player_controller.get(player_entity.player_controller.unwrap());
    let body = components.dynamic_body.get_mut(player_entity.dynamic_body.unwrap());
    let animator = components.sprite_animator.get_mut(player_entity.sprite_animator.unwrap());
    let sprite = &mut components.sprite_renderer.get_mut(player_entity.sprite_renderer.unwrap()).sprite;

    let speed_sqr = body.vx * body.vx + body.vy * body.vy;

    match player.state {
        PlayerState::Flying => {

            if player.aim_direction == [0.0, -1.0] {
                animator.animation = aim_up_anim(player, speed_sqr);
            } else if player.aim_direction == [0.0, 1.0] {
                animator.animation = aim_down_anim(player, speed_sqr);
            } else if player.aim_direction[1] < 0.0 {
                animator.animation = aim_up_forward_anim(player, speed_sqr);
            } else if player.aim_direction[1] > 0.0 {
                animator.animation = aim_down_forward_anim(player, speed_sqr);
            } else {
                animator.animation = aim_forward_anim(player, speed_sqr);
            }

            if player.aim_direction[0] > 0.0 {
                sprite.set_flip_x(false);
            } else if player.aim_direction[0] < 0.0 {
                sprite.set_flip_x(true);
            }

        },
        PlayerState::OnLeftWall => {

            if player.aim_direction == [1.0, 0.0] {
                animator.animation = aim_up_anim(player, speed_sqr);
            } else if player.aim_direction == [-1.0, 0.0] {
                animator.animation = aim_down_anim(player, speed_sqr);
            } else if player.aim_direction[0] < 0.0 {
                animator.animation = aim_down_forward_anim(player, speed_sqr);
            } else if player.aim_direction[0] > 0.0 {
                animator.animation = aim_up_forward_anim(player, speed_sqr);
            } else {
                animator.animation = aim_forward_anim(player, speed_sqr);
            }

            if player.aim_direction[1] < 0.0 {
                sprite.set_flip_x(true);
            } else if player.aim_direction[1] > 0.0 {
                sprite.set_flip_x(false);
            }

        },
        PlayerState::OnRightWall => {

            if player.aim_direction == [-1.0, 0.0] {
                animator.animation = aim_up_anim(player, speed_sqr);
            } else if player.aim_direction == [1.0, 0.0] {
                animator.animation = aim_down_anim(player, speed_sqr);
            } else if player.aim_direction[0] > 0.0 {
                animator.animation = aim_down_forward_anim(player, speed_sqr);
            } else if player.aim_direction[0] < 0.0 {
                animator.animation = aim_up_forward_anim(player, speed_sqr);
            } else {
                animator.animation = aim_forward_anim(player, speed_sqr);
            }

            if player.aim_direction[1] > 0.0 {
                sprite.set_flip_x(true);
            } else if player.aim_direction[1] < 0.0 {
                sprite.set_flip_x(false);
            }

        },
        PlayerState::OnCeiling => {

            if player.aim_direction == [0.0, 1.0] {
                animator.animation = aim_up_anim(player, speed_sqr);
            } else if player.aim_direction == [0.0, -1.0] {
                animator.animation = aim_down_anim(player, speed_sqr);
            } else if player.aim_direction[1] > 0.0 {
                animator.animation = aim_up_forward_anim(player, speed_sqr);
            } else if player.aim_direction[1] < 0.0 {
                animator.animation = aim_down_forward_anim(player, speed_sqr);
            } else {
                animator.animation = aim_forward_anim(player, speed_sqr);
            }

            if player.aim_direction[0] > 0.0 {
                sprite.set_flip_x(true);
            } else if player.aim_direction[0] < 0.0 {
                sprite.set_flip_x(false);
            }

        },
        PlayerState::OnFloor => {

            if player.aim_direction == [0.0, -1.0] {
                animator.animation = aim_up_anim(player, speed_sqr);
            } else if player.aim_direction == [0.0, 1.0] {
                animator.animation = aim_down_anim(player, speed_sqr);
            } else if player.aim_direction[1] < 0.0 {
                animator.animation = aim_up_forward_anim(player, speed_sqr);
            } else if player.aim_direction[1] > 0.0 {
                animator.animation = aim_down_forward_anim(player, speed_sqr);
            } else {
                animator.animation = aim_forward_anim(player, speed_sqr);
            }

            if player.aim_direction[0] > 0.0 {
                sprite.set_flip_x(false);
            } else if player.aim_direction[0] < 0.0 {
                sprite.set_flip_x(true);
            }

        },
    }
}

fn update_walk_state(player_entity: &world::Entity, components: &mut world::Components, entities: &Vec<world::Entity>) {

    use world::PlayerState;

    let new_state = get_walk_state(player_entity, components, entities);

    let sprite = &mut components.sprite_renderer.get_mut(player_entity.sprite_renderer.unwrap()).sprite;

    match new_state {
        PlayerState::Flying => { sprite.set_rotation(0.0) },
        PlayerState::OnLeftWall => { sprite.set_rotation(90.0) },
        PlayerState::OnRightWall => { sprite.set_rotation(270.0) },
        PlayerState::OnCeiling => { sprite.set_rotation(180.0) },
        PlayerState::OnFloor => { sprite.set_rotation(0.0) },
    }

    let player = components.player_controller.get_mut(player_entity.player_controller.unwrap());
    player.state = new_state;
}

fn get_walk_state(player_entity: &world::Entity, components: &world::Components, entities: &Vec<world::Entity>) -> world::PlayerState {

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

