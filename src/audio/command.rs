use std::path::PathBuf;

pub enum AudioCommand {
    Play(PathBuf),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}
