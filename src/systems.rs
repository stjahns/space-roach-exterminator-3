use components::*;

use ecs::{
    System,
    EntityIter,
    DataHelper
};

use ecs::system::{
    EntityProcess,
    EntitySystem,
};

use time::*;

systems! {
    SRSystems<SRComponents, ()> {
        sprites: EntitySystem<SpriteProcess> = EntitySystem::new(
             SpriteProcess,
             aspect!(<SRComponents> all: [update_context, position, sprite_renderer, sprite_animator])
        )
    }
}

pub struct SpriteProcess;

impl System for SpriteProcess { type Components = SRComponents; type Services = (); }

impl EntityProcess for SpriteProcess {
    fn process(
        &mut self,
        entities: EntityIter<SRComponents>,
        data: &mut DataHelper<SRComponents, ()>)
    {
        for e in entities {

            // TODO move elsewhere ...

            if data.update_context[e].context.borrow().input.left {
                data.position[e].x -= 1.0;
            }

            if data.update_context[e].context.borrow().input.right {
                data.position[e].x += 1.0;
            }

            if data.update_context[e].context.borrow().input.up {
                data.position[e].y -= 1.0;
            }

            if data.update_context[e].context.borrow().input.down {
                data.position[e].y += 1.0;
            }

            // TODO - convert from world-space to screen-space ...

            let Position {x, y} = data.position[e];
            data.sprite_renderer[e].sprite.set_position(x as f64, y as f64);
            // TODO not all sprites will necessarily be animated ...
            let frame = data.sprite_animator[e].get_frame(precise_time_s());
            data.sprite_renderer[e].sprite.set_src_rect(frame);
        }
    }
}
