use graphics::Context;
use opengl_graphics::GlGraphics;

use world;

impl world::EventReceiver {

    pub fn new() -> world::EventReceiver {
        world::EventReceiver {
            event_queue: Vec::new(),
        }
    }

    pub fn post_event(&mut self, event: world::Event) {
        // put event in queue?
        // dispatch event to any listeners?
        // iterate over components, look for ones that implement handle_event?

        // Send events to systems??
        self.event_queue.push(event);
    }

    pub fn clear_events(&mut self) {
        self.event_queue.clear();
    }

}

pub struct EventSystem;

impl world::System for EventSystem {

    ///
    /// Should be handled after all other systems...?
    ///
    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) {

        for entity in entities.iter() {
            if let Some(receiver_id) = entity.event_receiver {
                let receiver = components.event_receiver.get_mut(receiver_id);
                receiver.clear_events();
            }
        }

    }

    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) {
        // not implemented
    }
}
