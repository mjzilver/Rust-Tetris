use std::io::BufReader;

use rodio::{OutputStreamHandle, OutputStream};

pub struct Audio {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream
}

impl Audio {
    pub fn new() -> Audio {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        Audio {
            stream_handle,
            _stream,
        }
    }

    pub fn play_audio(&self, name: &str) {
        let filename = format!("assets/{}.mp3", name);
        let file = std::fs::File::open(filename).unwrap();
     
        let sink = self.stream_handle.play_once(BufReader::new(file)).unwrap();
        sink.set_volume(1.0);
        sink.detach();
    }
}
