use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use rodio::source::{Buffered, SamplesConverter};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, Sink};

pub struct Audio {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    audio_cache: HashMap<String, Buffered<Decoder<BufReader<File>>>>,
}

impl Audio {
    pub fn new() -> Audio {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();


        let mut audio = Audio {
            stream_handle,
            _stream,
            audio_cache: HashMap::new(),
        };

        audio.load_file("lose");
        audio.load_file("blip");
        audio.load_file("woosh");

        audio
    }

    fn load_file(&mut self, name: &str) {
        let filename = format!("assets/{}.mp3", name);
        let value =  Decoder::new(BufReader::new(File::open(filename).expect("file not found")))
        .expect("could not load source")
        .buffered();

        self.audio_cache.insert(name.to_owned(), value);
    }

    pub fn play_audio(&mut self, name: &str) {
        let buffer: SamplesConverter<Buffered<Decoder<BufReader<File>>>, i16> = self.audio_cache.get(name).unwrap().clone().convert_samples();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.set_volume(0.8);
        sink.append(buffer);
        sink.play();
        sink.detach()
    }
}