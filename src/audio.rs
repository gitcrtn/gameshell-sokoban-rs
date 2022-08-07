use gameshell_base::SoundResource;

#[derive(Eq, Hash, PartialEq, Clone, Default)]
pub enum AudioId {
    #[default]
    Correct,
    Incorrect,
    Wall,
}

mod sound_context {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use audio::AudioId;
    use crate::audio;

    lazy_static! {
        pub static ref SOUNDS: HashMap<AudioId, Vec<u8>> = {
            let mut m = HashMap::new();
            m.insert(AudioId::Correct, include_bytes!("../resources/sounds/correct.mp3").to_vec());
            m.insert(AudioId::Incorrect, include_bytes!("../resources/sounds/incorrect.mp3").to_vec());
            m.insert(AudioId::Wall, include_bytes!("../resources/sounds/wall.mp3").to_vec());
            m
        };
    }
}

#[derive(Default, Clone, Copy)]
pub struct Sound;

impl<'a> SoundResource<'a> for Sound {
    type AudioId = AudioId;

    fn get_audio_ids(&self) -> Vec<Self::AudioId> {
        let ref sounds_raw = sound_context::SOUNDS;
        sounds_raw.keys().cloned().collect::<Vec<AudioId>>()
    }

    fn get_audio(&self, audio_id: &Self::AudioId) -> &'static Vec<u8> {
        let ref sounds_raw = sound_context::SOUNDS;
        sounds_raw.get(audio_id).unwrap()
    }
}