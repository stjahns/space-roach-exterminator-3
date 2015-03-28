use graphics::Context;
use opengl_graphics::GlGraphics;

use world;

pub struct PhysicsSystem;

impl world::System for PhysicsSystem {

    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {

        // For each dynamic entity with a collider...
        for entity in entities.iter() {
            if let (Some(body_id), Some(collider_id), Some(position_id)) = (entity.dynamic_body, entity.collider, entity.position) {

                let body = components.dynamic_body.get(body_id);
                let collider = components.collider.get(collider_id);

                let mut new_position = components.position.get(position_id).clone();
                new_position.x += body.vx;
                new_position.y += body.vy;

                if !check_collision(collider, &new_position, entity, components, entities) {
                    let position = components.position.get_mut(position_id);
                    position.x = new_position.x;
                    position.y = new_position.y;

                }
            }
        }
    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }
}

fn check_collision(
    collider_1: &world::AABBCollider,
    position_1: &world::Position,
    entity: &world::Entity,
    components: &world::Components,
    entities: &Vec<world::Entity>
) -> bool {

    let mut colliding = false;

    for entity_2 in entities.iter() {

        if let (Some(collider_2_id), Some(position_2_id)) = (entity_2.collider, entity_2.position) {

            // TODO - could boost efficiency by keeping static collider AABBs sorted...

            // Don't check for collisions with self!
            if entity.collider.unwrap() == collider_2_id {
                continue;
            }

            let collider_2 = components.collider.get(collider_2_id);
            let position_2 = components.position.get(position_2_id);

            if aabb_intersect(collider_1, position_1, collider_2, position_2) {
                colliding = true;
            }
        }
    }

    colliding
}


pub fn aabb_intersect(
    a_collider: &world::AABBCollider,
    a_position: &world::Position,
    b_collider: &world::AABBCollider,
    b_position: &world::Position
) -> bool {

    let x_overlap = a_position.x - a_collider.width / 2.0 < b_position.x + b_collider.width / 2.0
        && a_position.x + a_collider.width / 2.0 > b_position.x - b_collider.width / 2.0;

    let y_overlap = a_position.y - a_collider.height / 2.0 < b_position.y + b_collider.height / 2.0
        && a_position.y + a_collider.height / 2.0 > b_position.y - b_collider.height / 2.0;

    x_overlap && y_overlap
}
