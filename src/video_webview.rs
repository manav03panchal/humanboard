use std::path::PathBuf;
use std::process::Command;

/// Native video player - opens in system player
pub struct VideoPlayer {
    pub video_path: PathBuf,
    pub file_name: String,
}

impl VideoPlayer {
    pub fn new(video_path: PathBuf) -> Self {
        let file_name = video_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Video")
            .to_string();

        Self {
            video_path,
            file_name,
        }
    }

    /// Open the video in the system's default player
    pub fn open_in_system_player(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open").arg(&self.video_path).spawn();
        }

        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("cmd")
                .args(["/C", "start", "", self.video_path.to_str().unwrap_or("")])
                .spawn();
        }

        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("xdg-open").arg(&self.video_path).spawn();
        }
    }
}
