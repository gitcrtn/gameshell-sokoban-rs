use gameshell_base::{ImageResource, Position};
use crate::constants::{MAP_OFFSET_X, MAP_OFFSET_Y, TILE_WIDTH};

#[derive(Eq, Hash, PartialEq, Clone, Default)]
pub enum TextureId {
    // images
    #[default]
    BoxRed1,
    BoxRed2,
    BoxBlue1,
    BoxBlue2,
    BoxSpotRed,
    BoxSpotBlue,
    Player1,
    Player2,
    Player3,
    Floor,
    Wall,

    // font images
    FontDefault,
    FontScaleA,

    // texts
    TextReset,
}

mod image_context {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use texture::TextureId;
    use crate::texture;

    lazy_static! {
        pub static ref IMAGES: HashMap<TextureId, Vec<u8>> = {
            let mut m = HashMap::new();
            m.insert(TextureId::BoxRed1, include_bytes!("../resources/images/box_red_1.png").to_vec());
            m.insert(TextureId::BoxRed2, include_bytes!("../resources/images/box_red_2.png").to_vec());
            m.insert(TextureId::BoxBlue1, include_bytes!("../resources/images/box_blue_1.png").to_vec());
            m.insert(TextureId::BoxBlue2, include_bytes!("../resources/images/box_blue_2.png").to_vec());
            m.insert(TextureId::BoxSpotRed, include_bytes!("../resources/images/box_spot_red.png").to_vec());
            m.insert(TextureId::BoxSpotBlue, include_bytes!("../resources/images/box_spot_blue.png").to_vec());
            m.insert(TextureId::Player1, include_bytes!("../resources/images/player_1.png").to_vec());
            m.insert(TextureId::Player2, include_bytes!("../resources/images/player_2.png").to_vec());
            m.insert(TextureId::Player3, include_bytes!("../resources/images/player_3.png").to_vec());
            m.insert(TextureId::Floor, include_bytes!("../resources/images/floor.png").to_vec());
            m.insert(TextureId::Wall, include_bytes!("../resources/images/wall.png").to_vec());
            m.insert(TextureId::FontDefault, include_bytes!("../resources/images/font_ponderosa_8px.png").to_vec());
            m.insert(TextureId::FontScaleA, include_bytes!("../resources/images/font_ponderosa_12px.png").to_vec());
            m
        };

        pub static ref FONT_HEIGHTS: HashMap<TextureId, u32> = {
            let mut m = HashMap::new();
            m.insert(TextureId::FontDefault, 8);
            m.insert(TextureId::FontScaleA, 12);
            m
        };

        pub static ref TEXTS: HashMap<TextureId, String> = {
            let mut m = HashMap::new();
            m.insert(TextureId::TextReset, "A: Reset".to_string());
            m
        };
    }
}

#[derive(Default)]
pub struct Image;

impl ImageResource for Image {
    type TextureId = TextureId;

    fn get_image_ids(&self) -> Vec<Self::TextureId> {
        let ref images_raw = image_context::IMAGES;
        images_raw.keys().cloned().collect::<Vec<TextureId>>()
    }

    fn get_image(&self, texture_id: &Self::TextureId) -> &Vec<u8> {
        let ref images_raw = image_context::IMAGES;
        images_raw.get(texture_id).unwrap()
    }

    fn get_text_ids(&self) -> Vec<Self::TextureId> {
        let ref texts_raw = image_context::TEXTS;
        texts_raw.keys().cloned().collect::<Vec<TextureId>>()
    }

    fn get_text(&self, texture_id: &Self::TextureId) -> &String {
        let ref texts_raw = image_context::TEXTS;
        texts_raw.get(texture_id).unwrap()
    }

    fn get_font_height(&self, texture_id: &Self::TextureId) -> &u32 {
        let ref font_heights = image_context::FONT_HEIGHTS;
        font_heights.get(texture_id).unwrap()
    }

    fn get_default_font_id(&self) -> Self::TextureId {
        TextureId::FontDefault
    }

    fn get_tile_position(&self, position: &Position) -> (i32, i32) {
        (
            position.x as i32 * TILE_WIDTH + MAP_OFFSET_X,
            position.y as i32 * TILE_WIDTH + MAP_OFFSET_Y,
        )
    }
}