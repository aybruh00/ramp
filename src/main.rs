use std::io::BufReader;
use std::thread;
use std::time::Duration;

use rodio::Source;

// Plays a tone alternating between right and left ears, with right being first.
fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("Spatialized.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
