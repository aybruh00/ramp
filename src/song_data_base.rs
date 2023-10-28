use anyhow::Result;
use audiotags::Tag;

use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{self, BufReader};
use std::collections::HashMap;

pub struct Song {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    path: PathBuf,
    duration: Option<f64>,
}

impl Song {
    fn new(path: &Path) -> Song {
        Song{
            title: String::new(),
            artist: None,
            album: None,
            path: PathBuf::from(path),
            duration: None,
        }
    }

    fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    fn set_artist(&mut self, artist: String) {
        self.artist = Some(String::from(artist));
    }

    fn set_album(&mut self, album: &str) {
        self.album = Some(String::from(album));
    }
    
    fn set_duration(&mut self, duration: f64) {
        self.duration = Some(duration);
    }
}

pub struct SongDB{
    db: HashMap<String, Song>,
}

impl SongDB {
    fn new(root: PathBuf) {
        let _res = SongDB::discover_files(&root, &mut HashMap::new());
    }

    fn discover_files(root: &Path, map: &mut HashMap<String, Song>) -> io::Result<()> {
        if root.is_dir() {
            for entry in fs::read_dir(root)? {
                let entry = entry?;
                let path = entry.path();
                if !path.is_dir() {
                    if let Ok(tag) = Tag::new().read_from_path(&path) {
                        let title = path.file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap();
                        let mut newsong = Song::new(&path);
                        let anytag = tag.to_anytag();

                        if let Some(title) = anytag.title() {
                            newsong.set_title(title);
                        }

                        if let Some(artist) = anytag.artists_as_string() {
                            newsong.set_artist(artist);
                        }

                        if let Some(album) = anytag.album_title() {
                            newsong.set_album(album);
                        }
                        
                        if let Some(len) = anytag.duration() {
                            newsong.set_duration(len);
                        }
                        
                        map.insert(String::from(title), newsong);
                    }
                } else {
                    SongDB::discover_files(&path, map)?;
                }
            }
        }
        Ok(())
    }
}