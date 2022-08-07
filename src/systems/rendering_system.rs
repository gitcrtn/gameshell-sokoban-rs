use gameshell_base::{Position, Renderable, RenderingHelper, Time};
use specs::{Read, ReadStorage, System};
use crate::constants::{BG_COLOR, TEXT_FOOTER_Y, TEXT_OFFSET_X};
use crate::resources::{Gameplay, GameplayState};
use crate::texture::{Image, TextureId};

pub struct RenderingSystem<'a> {
    pub helper: RenderingHelper<'a, Image>,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable<Image>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

        self.helper.context.clear(BG_COLOR);
        self.helper.draw_renderables(&positions, &renderables, &time);

        // if gameplay.keys_pressed_text.len() > 0 {
        //     self.helper.context.draw_text(gameplay.keys_pressed_text.clone(), 0, 0);
        // }

        self.helper.context.draw_text(format!("FPS:  {:.2}", time.fps_avg), TEXT_OFFSET_X, 16);
        self.helper.context.draw_text(format!("TIME: {:>02}:{:>02}", time.minutes, time.seconds), TEXT_OFFSET_X, 24);
        self.helper.context.draw_text(format!("STEP: {}", gameplay.moves_count), TEXT_OFFSET_X, 32);

        if gameplay.state == GameplayState::Won {
            self.helper.context.draw_text_font("You Win".to_string(), TEXT_OFFSET_X, 56, TextureId::FontScaleA);
        }

        self.helper.context.draw(TextureId::TextReset, TEXT_OFFSET_X, TEXT_FOOTER_Y);
        self.helper.context.present();
    }
}