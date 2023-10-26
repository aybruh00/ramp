use std::{fs, path};
use std::io::BufReader;

use anyhow::Result;
use rodio::Source;

mod player;
use crate::player::player::Player;

fn main() -> Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let player = Player::default(&handle)?;

    let file = fs::File::open("/home/ayush/Music/The Weeknd - Is There Someone Else.flac").unwrap();
    player.add_to_queue(rodio::Decoder::new(BufReader::new(file)).unwrap());

    player.wait();
    Ok(())
}
