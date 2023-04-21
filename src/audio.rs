use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use rodio::source::{Buffered, SamplesConverter};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, Sink};

/// Keeps all the possible sound effects as an enum
#[derive(Eq, Hash, PartialEq)]
pub enum SoundEffect {
    Lose,
    Move,
    Rotate,
    Menu,
    RowCompleted,
}

/// turns the enum into a filename
impl SoundEffect {
    fn to_filename(&self) -> String {
        "assets/".to_string() +
        match self {
            SoundEffect::Lose => "lose",
            SoundEffect::Move => "move",
            SoundEffect::Rotate => "rotate",
            SoundEffect::Menu => "menu",
            SoundEffect::RowCompleted => "row_completed",
        }
        + ".mp3"
    }
}

/// audio system that saves and loads sounds
pub struct Audio {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    audio_cache: HashMap<SoundEffect, Buffered<Decoder<BufReader<File>>>>,
}

impl Audio {
    /// creating a new audio device
    pub fn new() -> Audio {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        let mut audio = Audio {
            stream_handle,
            _stream,
            audio_cache: HashMap::new(),
        };

        audio.load_file(SoundEffect::Lose);
        audio.load_file(SoundEffect::Move);
        audio.load_file(SoundEffect::Rotate);
        audio.load_file(SoundEffect::Menu);
        audio.load_file(SoundEffect::RowCompleted);

        audio
    }

    /// loading in a new sound file and saving it to memory
    fn load_file(&mut self, sound_effect: SoundEffect) {
        let value =  Decoder::new(BufReader::new(File::open(sound_effect.to_filename()).expect("file not found")))
        .expect("could not load source")
        .buffered();

        self.audio_cache.insert(sound_effect, value);
    }

    /// plays the sound associated with the enum
    pub fn play_audio(&mut self, sound_effect: SoundEffect) {
        let buffer: SamplesConverter<Buffered<Decoder<BufReader<File>>>, i16> = self.audio_cache.get(&sound_effect).unwrap().clone().convert_samples();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(0.8);
        sink.append(buffer);
        sink.play();
        sink.detach()
    }
}