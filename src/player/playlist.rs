use super::track::Track;

#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub current_index: usize,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: Vec::new(),
            current_index: 0,
        }
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.tracks.get(self.current_index)
    }

    pub fn next(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }
        self.current_index = (self.current_index + 1) % self.tracks.len();
        self.current_track()
    }

    pub fn previous(&mut self) -> Option<&Track> {
        if self.tracks.is_empty() {
            return None;
        }
        if self.current_index == 0 {
            self.current_index = self.tracks.len() - 1;
        } else {
            self.current_index -= 1;
        }
        self.current_track()
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn select(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.current_index = index;
        }
    }
}
