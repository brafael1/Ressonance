use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::Mutex;

use super::command::AudioCommand;
use super::player::PlayerProcess;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

pub fn audio_thread(
    mut command_rx: mpsc::UnboundedReceiver<AudioCommand>,
    is_playing: Arc<AtomicBool>,
    elapsed: Arc<Mutex<Duration>>,
) {
    let mut player: Option<PlayerProcess> = None;

    loop {
        if let Some(ref mut p) = player {
            if !p.paused {
                let current = p.start_time.elapsed();
                *elapsed.blocking_lock() = current;
            }
        }

        match command_rx.try_recv() {
            Ok(cmd) => match cmd {
                AudioCommand::Play(path) => {
                    if let Some(ref mut p) = player {
                        p.kill();
                    }

                    match Command::new("ffplay")
                        .arg("-nodisp")
                        .arg("-autoexit")
                        .arg("-loglevel")
                        .arg("quiet")
                        .arg(&path)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                    {
                        Ok(child) => {
                            player = Some(PlayerProcess::new(child));
                            is_playing.store(true, Ordering::SeqCst);
                        }
                        Err(e) => {
                            eprintln!("Failed to start ffplay: {}", e);
                        }
                    }
                }
                AudioCommand::Pause => {
                    if let Some(ref mut p) = player {
                        if !p.paused {
                            let _ = kill(Pid::from_raw(p.child.id() as i32), Signal::SIGSTOP);
                            p.paused = true;
                            is_playing.store(false, Ordering::SeqCst);
                        }
                    }
                }
                AudioCommand::Resume => {
                    if let Some(ref mut p) = player {
                        if p.paused {
                            let _ = kill(Pid::from_raw(p.child.id() as i32), Signal::SIGCONT);
                            p.paused = false;
                            is_playing.store(true, Ordering::SeqCst);
                        }
                    }
                }
                AudioCommand::Stop => {
                    if let Some(ref mut p) = player {
                        p.kill();
                    }
                    player = None;
                    is_playing.store(false, Ordering::SeqCst);
                    *elapsed.blocking_lock() = Duration::ZERO;
                }
                AudioCommand::SetVolume(_) => {}
            },
            Err(_) => {
                if let Some(ref mut p) = player {
                    match p.child.try_wait() {
                        Ok(Some(_)) => {
                            is_playing.store(false, Ordering::SeqCst);
                            player = None;
                            *elapsed.blocking_lock() = Duration::ZERO;
                        }
                        Ok(None) => {}
                        Err(_) => {
                            player = None;
                            is_playing.store(false, Ordering::SeqCst);
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
}
