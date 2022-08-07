use specs::{RunNow, World, WorldExt};
use gameshell_base::{AudioContext, Core, MainLoop, RenderContext, RenderingHelper};
use crate::audio::Sound;
use crate::components::register_components;
use crate::constants::BG_COLOR;
use crate::map::initialize_level;
use crate::resources::{Gameplay, register_resources};
use crate::texture::Image;
use crate::systems::{EventSystem, GameplayStateSystem, InputSystem, RenderingSystem};

pub struct Loop;

impl<'a> MainLoop<'a, Image, Sound> for Loop {
    fn create_world(&self) -> World {
        let mut world = World::new();
        register_components(&mut world);
        register_resources(&mut world);
        world
    }

    fn post_create_world(&self, world: &mut World) {
        initialize_level(world);
    }

    fn setup(&self, render_context: &mut RenderContext<Image>, _audio_context: &mut AudioContext<'a, Sound>) {
        render_context.clear(BG_COLOR);
        render_context.present();
    }

    fn update(&self, world: &mut World, audio_context: &mut AudioContext<'a, Sound>) {
        {
            let mut is = InputSystem {};
            is.run_now(world);
        }

        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(world);
        }

        {
            let mut es = EventSystem {
                audio_context,
            };
            es.run_now(world);
        }
    }

    fn reset_game(&self, world: &mut World, _render_context: &mut RenderContext<Image>, _audio_context: &mut AudioContext<'a, Sound>) {
        world.delete_all();
        initialize_level(world);

        let mut core = world.write_resource::<Core>();
        core.pause_time = false;

        let mut gameplay = world.write_resource::<Gameplay>();
        gameplay.moves_count = 0;
    }

    fn reset_frame(&self, _world: &mut World, _render_context: &mut RenderContext<Image>, _audio_context: &mut AudioContext<'a, Sound>) {
        // empty
    }

    fn draw(&self, world: &mut World, render_context: &mut RenderContext<Image>) {
        {
            let mut rs = RenderingSystem {
                helper: RenderingHelper { context: render_context },
            };
            rs.run_now(world);
        }
    }
}