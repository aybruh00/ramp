use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{self, BufReader};

use anyhow::Result;
use rodio::Source;
use audiotags::{Tag};

mod player;
mod song_data_base;

use crate::player::player::Player;

fn mmain() -> Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let player = Player::default(&handle)?;

    let file = fs::File::open("/home/ayush/Music/The Weeknd - Is There Someone Else.flac").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    // let source = source.skip_duration(std::time::Duration::from_secs(192));
    // player.add_to_queue(source);

    let file = fs::File::open("/home/ayush/Music/Ruth B. - Dandelions.flac").unwrap();
    let file = fs::File::open(Path::new("/home/ayush/Music/Kenya Grace - Strangers.mp3")).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    // let source = source.skip_duration(std::time::Duration::from_secs(150));
    let source = source.fade_in(std::time::Duration::from_secs(4));
    player.add_to_queue(source);

    player.wait();
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}

fn main(){
    // let src = Path::new("/home/ayush/Music/Kenya Grace - Strangers.mp3");
    let src = Path::new("/home/ayush/Music/Poets of the Fall - My Dark Disquiet.mp3");
    // let src = Path::new("/home/ayush/Music/test.rs");
    let mut tag = Tag::new().read_from_path(src).unwrap();

    println!("{:?}",tag.duration());
    let any = tag.to_anytag();
    println!("{:#?} {:#?} {:#?} {:#?} {:#?} {:#?} ", any.title, any.artists, any.year, any.duration, any.album_title, any.album_artists);
}
