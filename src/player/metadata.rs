use std::path::PathBuf;

use id3::TagLike;

use super::track::Track;

pub fn find_audio_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "mp3" | "flac" | "wav" | "ogg" | "m4a") {
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    files
}

pub fn read_metadata(path: &PathBuf) -> Track {
    let mut track = Track::from_path(path.clone());

    if let Ok(tag) = id3::Tag::read_from_path(path) {
        if let Some(title) = tag.title() {
            track.title = title.to_string();
        }
        if let Some(artist) = tag.artist() {
            track.artist = artist.to_string();
        }
        if let Some(album) = tag.album() {
            track.album = album.to_string();
        }
    }

    track
}
