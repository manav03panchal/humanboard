use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

/// Native audio player using system audio playback
pub struct AudioPlayer {
    pub audio_path: PathBuf,
    pub file_name: String,
    pub is_playing: Arc<Mutex<bool>>,
    pub process: Arc<Mutex<Option<Child>>>,
}

impl AudioPlayer {
    pub fn new(audio_path: PathBuf) -> Self {
        let file_name = audio_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Audio")
            .to_string();

        Self {
            audio_path,
            file_name,
            is_playing: Arc::new(Mutex::new(false)),
            process: Arc::new(Mutex::new(None)),
        }
    }

    pub fn toggle_playback(&self) {
        println!("toggle_playback called for: {:?}", self.audio_path);
        let mut is_playing = self.is_playing.lock().unwrap();
        let mut process = self.process.lock().unwrap();

        println!("Current state: is_playing={}", *is_playing);

        if *is_playing {
            // Stop playback
            println!("Stopping playback...");
            if let Some(ref mut child) = *process {
                let _ = child.kill();
            }
            *process = None;
            *is_playing = false;
        } else {
            // Start playback using afplay (macOS)
            println!("Starting playback with afplay for: {:?}", self.audio_path);
            #[cfg(target_os = "macos")]
            {
                match Command::new("afplay").arg(&self.audio_path).spawn() {
                    Ok(child) => {
                        println!("afplay started successfully, pid: {:?}", child.id());
                        *process = Some(child);
                        *is_playing = true;
                    }
                    Err(e) => {
                        eprintln!("Failed to start audio playback: {}", e);
                    }
                }
            }

            #[cfg(not(target_os = "macos"))]
            {
                eprintln!("Native audio playback not implemented for this platform");
            }
        }
    }

    pub fn stop(&self) {
        let mut is_playing = self.is_playing.lock().unwrap();
        let mut process = self.process.lock().unwrap();

        if let Some(ref mut child) = *process {
            let _ = child.kill();
        }
        *process = None;
        *is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        // Check if process is still running
        let mut process = self.process.lock().unwrap();
        if let Some(ref mut child) = *process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    // Process finished
                    *process = None;
                    let mut is_playing = self.is_playing.lock().unwrap();
                    *is_playing = false;
                    false
                }
                Ok(None) => true, // Still running
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
