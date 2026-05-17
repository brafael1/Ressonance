use std::process::{Child, Command, Stdio};

pub struct PlayerProcess {
    pub child: Child,
    pub start_time: std::time::Instant,
    pub paused: bool,
}

impl PlayerProcess {
    pub fn new(child: Child) -> Self {
        Self {
            child,
            start_time: std::time::Instant::now(),
            paused: false,
        }
    }

    pub fn kill(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn kill_ffplay_processes() {
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("ffplay.*-nodisp")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
