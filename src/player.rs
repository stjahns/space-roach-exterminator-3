use graphics::Context;
use opengl_graphics::GlGraphics;
use vecmath::*;
use openal::al;
use std::num::Float;
use std::f64;

use world;
use physics;
use audio;

use world::Weapon;

use std::rc::Rc;
use std::path::Path;
use opengl_graphics::Texture;

pub struct PlayerSystem;

impl Weapon {

    pub fn new() -> Weapon {

        let mut fire_sound = al::Buffer::gen();
        audio::load_buffer("assets/Shoot.wav", &mut fire_sound);

        world::Weapon {
            fire_delay: 0.05,
            bullet_speed: 3.0,
            fire_timer: 0.0,
            fire_sound: fire_sound,
            firing: false,
            fire_direction: [0.0, 0.0],
        }
    }

}

impl world::System for PlayerSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {

        // Update player...
        for entity in entities.iter() {
            if let (Some(player_id), Some(body_id), Some(animator_id)) = (entity.player_controller, entity.dynamic_body, entity.sprite_animator) {

                // Update orientation
                update_walk_state(entity, components, entities);

                // Update aim direction
                if let Some(aim_direction) = get_aim_direction(control_state) {
                    let player = components.player_controller.get_mut(player_id);
                    player.aim_direction = aim_direction;

                    if let Some(weapon_id) = entity.weapon {
                        let weapon = components.weapon.get_mut(weapon_id);
                        weapon.firing = true;
                        weapon.fire_direction = aim_direction;
                    }
                } else {
                    if let Some(weapon_id) = entity.weapon {
                        let weapon = components.weapon.get_mut(weapon_id);
                        weapon.firing = false;
                    }
                }

                // Update animation given orientation, aim direction, and speed
                update_animation(entity, components, entities);

                update_control(entity, control_state, components, entities);

            }
        }

        let mut bullets = Vec::new();

        // Update weapons...
        for entity in entities.iter() {
            if let (Some(weapon_id), Some(position_id), Some(source_id)) = (entity.weapon, entity.position, entity.audio_source) {

                let weapon = components.weapon.get_mut(weapon_id);
                let position = components.position.get(position_id).clone();
                let audio_source = components.audio_source.get_mut(source_id);

                weapon.fire_timer -= 0.001; // TODO need delta time..

                if weapon.firing && !(weapon.fire_timer > 0.0) {
                    audio_source.play_buffer(weapon.fire_sound.clone());
                    weapon.fire_timer = weapon.fire_delay;

                    let bullet_texture = Rc::new(Texture::from_path(&Path::new("./assets/Bullet.png")).unwrap());
                    let mut sprite_renderer = world::SpriteRenderer::from_texture_region(
                        bullet_texture.clone(),
                        [0, 0, 8, 8],
                    );

                    let angle = weapon.fire_direction[1].atan2(weapon.fire_direction[0]) as f64 * 180.0 / f64::consts::PI;
                    sprite_renderer.sprite.set_rotation(angle + 90.0);

                    let velocity = vec2_scale(weapon.fire_direction, weapon.bullet_speed);

                    let bullet_origin = vec2_add(vec2_scale(weapon.fire_direction, 32.0), [position.x, position.y]);

                    let bullet_entity = world::Entity {
                        position: Some(components.position.add(world::Position { x: bullet_origin[0], y: bullet_origin[1] })),
                        sprite_renderer: Some(components.sprite_renderer.add(sprite_renderer)),
                        sprite_animator: None,
                        player_controller: None,
                        camera_target: None,
                        collider: Some(components.collider.add(world::AABBCollider { width: 8.0, height: 8.0 })),
                        dynamic_body: Some(components.dynamic_body.add(world::DynamicBody { vx: velocity[0], vy: velocity[1] })),
                        audio_source: None,
                        weapon: None,
                        bullet: Some(components.bullet.add(world::Bullet)),
                        event_receiver: Some(components.event_receiver.add(world::EventReceiver::new())),
                    };

                    bullets.push(bullet_entity);

                }

            }
        }

        // TODO use some kind of bullet entity pool...

        let (new_entities, reserve): (Vec<_>, _) = entities.drain().partition(|entity| {

            if let (Some(bullet_id), Some(receiver_id)) = (entity.bullet, entity.event_receiver) {
                let event_receiver = components.event_receiver.get(receiver_id);

                let mut colliding = false;
                for event in event_receiver.event_queue.iter() {
                    match *event {
                        world::Event::Collision(collider_1, collider_2) => {
                            colliding = true
                        },
                    }
                }

                // filter out colliding bullets...
                !colliding

            } else {
                true
            }
        });


        *entities = new_entities;

        while (bullets.len() > 0 ) {
            entities.push(bullets.pop().unwrap());
        }
    }
    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }

}

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

fn update_control(player_entity: &world::Entity, control_state: &world::ControlState, components: &mut world::Components, entities: &Vec<world::Entity>) {

    use world::PlayerState;

    let player = components.player_controller.get(player_entity.player_controller.unwrap());
    let body = components.dynamic_body.get_mut(player_entity.dynamic_body.unwrap());

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

//
// TODO - probably replace this with some generic Animation State Machine
//
fn update_animation(player_entity: &world::Entity, components: &mut world::Components, entities: &Vec<world::Entity>) {

    use world::PlayerState;

    let player = components.player_controller.get(player_entity.player_controller.unwrap());
    let body = components.dynamic_body.get_mut(player_entity.dynamic_body.unwrap());
    let animator = components.sprite_animator.get_mut(player_entity.sprite_animator.unwrap());
    let sprite = &mut components.sprite_renderer.get_mut(player_entity.sprite_renderer.unwrap()).sprite;

    let speed2 = body.vx * body.vx + body.vy * body.vy;

    match player.state {
        PlayerState::Flying => {

            if player.aim_direction == [0.0, -1.0] {
                animator.animation = aim_up_anim(player, speed2);
            } else if player.aim_direction == [0.0, 1.0] {
                animator.animation = aim_down_anim(player, speed2);
            } else if player.aim_direction[1] < 0.0 {
                animator.animation = aim_up_forward_anim(player, speed2);
            } else if player.aim_direction[1] > 0.0 {
                animator.animation = aim_down_forward_anim(player, speed2);
            } else {
                animator.animation = aim_forward_anim(player, speed2);
            }

            if player.aim_direction[0] > 0.0 {
                sprite.set_flip_x(false);
            } else if player.aim_direction[0] < 0.0 {
                sprite.set_flip_x(true);
            }

        },
        PlayerState::OnLeftWall => {

            if player.aim_direction == [1.0, 0.0] {
                animator.animation = aim_up_anim(player, speed2);
            } else if player.aim_direction == [-1.0, 0.0] {
                animator.animation = aim_down_anim(player, speed2);
            } else if player.aim_direction[0] < 0.0 {
                animator.animation = aim_down_forward_anim(player, speed2);
            } else if player.aim_direction[0] > 0.0 {
                animator.animation = aim_up_forward_anim(player, speed2);
            } else {
                animator.animation = aim_forward_anim(player, speed2);
            }

            if player.aim_direction[1] < 0.0 {
                sprite.set_flip_x(true);
            } else if player.aim_direction[1] > 0.0 {
                sprite.set_flip_x(false);
            }

        },
        PlayerState::OnRightWall => {

            if player.aim_direction == [-1.0, 0.0] {
                animator.animation = aim_up_anim(player, speed2);
            } else if player.aim_direction == [1.0, 0.0] {
                animator.animation = aim_down_anim(player, speed2);
            } else if player.aim_direction[0] > 0.0 {
                animator.animation = aim_down_forward_anim(player, speed2);
            } else if player.aim_direction[0] < 0.0 {
                animator.animation = aim_up_forward_anim(player, speed2);
            } else {
                animator.animation = aim_forward_anim(player, speed2);
            }

            if player.aim_direction[1] > 0.0 {
                sprite.set_flip_x(true);
            } else if player.aim_direction[1] < 0.0 {
                sprite.set_flip_x(false);
            }

        },
        PlayerState::OnCeiling => {

            if player.aim_direction == [0.0, 1.0] {
                animator.animation = aim_up_anim(player, speed2);
            } else if player.aim_direction == [0.0, -1.0] {
                animator.animation = aim_down_anim(player, speed2);
            } else if player.aim_direction[1] > 0.0 {
                animator.animation = aim_up_forward_anim(player, speed2);
            } else if player.aim_direction[1] < 0.0 {
                animator.animation = aim_down_forward_anim(player, speed2);
            } else {
                animator.animation = aim_forward_anim(player, speed2);
            }

            if player.aim_direction[0] > 0.0 {
                sprite.set_flip_x(true);
            } else if player.aim_direction[0] < 0.0 {
                sprite.set_flip_x(false);
            }

        },
        PlayerState::OnFloor => {

            if player.aim_direction == [0.0, -1.0] {
                animator.animation = aim_up_anim(player, speed2);
            } else if player.aim_direction == [0.0, 1.0] {
                animator.animation = aim_down_anim(player, speed2);
            } else if player.aim_direction[1] < 0.0 {
                animator.animation = aim_up_forward_anim(player, speed2);
            } else if player.aim_direction[1] > 0.0 {
                animator.animation = aim_down_forward_anim(player, speed2);
            } else {
                animator.animation = aim_forward_anim(player, speed2);
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

    let player = components.player_controller.get_mut(player_entity.player_controller.unwrap());
    let sprite = &mut components.sprite_renderer.get_mut(player_entity.sprite_renderer.unwrap()).sprite;
    let audio_source = components.audio_source.get_mut(player_entity.audio_source.unwrap());

    match new_state {
        PlayerState::Flying => {

            if player.state != PlayerState::Flying {
                audio_source.play_buffer(player.jump_sound.clone());
            }

            sprite.set_rotation(0.0)
        },
        PlayerState::OnLeftWall => { sprite.set_rotation(90.0) },
        PlayerState::OnRightWall => { sprite.set_rotation(270.0) },
        PlayerState::OnCeiling => { sprite.set_rotation(180.0) },
        PlayerState::OnFloor => { sprite.set_rotation(0.0) },
    }

    if player.state == PlayerState::Flying && new_state != PlayerState::Flying {
        audio_source.play_buffer(player.land_sound.clone());
    }

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

