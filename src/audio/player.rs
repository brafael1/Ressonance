use std::process::{Child, Command, Stdio};

pub struct PlayerProcess {
    pub child: Child,
    pub elapsed: std::time::Duration,
    pub paused: bool,
    pub pause_start: Option<std::time::Instant>,
}

impl PlayerProcess {
    pub fn new(child: Child) -> Self {
        Self {
            child,
            elapsed: std::time::Duration::ZERO,
            paused: false,
            pause_start: None,
        }
    }

    pub fn kill(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }

    pub fn pause(&mut self) {
        if !self.paused {
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;
                let _ = kill(Pid::from_raw(self.child.id() as i32), Signal::SIGSTOP);
            }
            self.paused = true;
            self.pause_start = Some(std::time::Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if self.paused {
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;
                let _ = kill(Pid::from_raw(self.child.id() as i32), Signal::SIGCONT);
            }
            if let Some(pause_start) = self.pause_start.take() {
                self.elapsed += pause_start.elapsed();
            }
            self.paused = false;
        }
    }

    pub fn current_elapsed(&self) -> std::time::Duration {
        if self.paused {
            self.elapsed
        } else {
            self.elapsed + self.pause_start.unwrap_or(std::time::Instant::now()).elapsed()
        }
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
