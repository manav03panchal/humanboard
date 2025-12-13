use crate::pdf_thumbnail::generate_pdf_thumbnail;
use crate::spotify_webview::SpotifyContentType;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasItem {
    pub id: u64,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub content: ItemContent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemContent {
    Image(PathBuf),
    Text(String),
    Video(PathBuf),
    Audio(PathBuf),
    Pdf {
        path: PathBuf,
        thumbnail: Option<PathBuf>,
    },
    Link(String),
    YouTube(String), // Video ID
    Spotify {
        content_type: SpotifyContentType,
        content_id: String,
    },
    SpotifyApp, // Full Spotify web player
    Markdown {
        path: PathBuf,
        title: String,
        content: String, // Store content for preview
    },
}

/// Extract YouTube video ID from various URL formats
pub fn extract_youtube_id(url: &str) -> Option<String> {
    // Handle youtu.be/VIDEO_ID
    if url.contains("youtu.be/") {
        return url
            .split("youtu.be/")
            .nth(1)
            .and_then(|s| s.split(['?', '&', '#']).next())
            .map(|s| s.to_string());
    }

    // Handle youtube.com/watch?v=VIDEO_ID
    if url.contains("youtube.com/watch") {
        return url
            .split("v=")
            .nth(1)
            .and_then(|s| s.split(['&', '#']).next())
            .map(|s| s.to_string());
    }

    // Handle youtube.com/embed/VIDEO_ID
    if url.contains("youtube.com/embed/") {
        return url
            .split("youtube.com/embed/")
            .nth(1)
            .and_then(|s| s.split(['?', '&', '#']).next())
            .map(|s| s.to_string());
    }

    None
}

impl ItemContent {
    pub fn default_size(&self) -> (f32, f32) {
        match self {
            ItemContent::Image(path) => {
                // Try to load the image and get its actual dimensions, scaled to max 800px
                if let Ok(img) = image::open(path) {
                    let (width, height) = img.dimensions();
                    let max_dimension = 800.0;

                    let aspect_ratio = width as f32 / height as f32;

                    if width > height {
                        if width as f32 > max_dimension {
                            (max_dimension, max_dimension / aspect_ratio)
                        } else {
                            (width as f32, height as f32)
                        }
                    } else {
                        if height as f32 > max_dimension {
                            (max_dimension * aspect_ratio, max_dimension)
                        } else {
                            (width as f32, height as f32)
                        }
                    }
                } else {
                    (800.0, 600.0)
                }
            }
            ItemContent::Text(_) => (300.0, 100.0),
            ItemContent::Video(_) => (400.0, 300.0),
            ItemContent::Audio(_) => (320.0, 160.0), // Compact audio player
            ItemContent::Pdf { .. } => (250.0, 350.0),
            ItemContent::Link(_) => (300.0, 150.0),
            ItemContent::YouTube(_) => (560.0, 315.0), // 16:9 aspect ratio
            ItemContent::Spotify { content_type, .. } => {
                match content_type {
                    SpotifyContentType::Track => (400.0, 100.0), // Single track - very compact
                    SpotifyContentType::Album | SpotifyContentType::Playlist => (352.0, 380.0), // List view
                    SpotifyContentType::Artist => (352.0, 380.0),
                    SpotifyContentType::Episode | SpotifyContentType::Show => (352.0, 160.0),
                }
            }
            ItemContent::SpotifyApp => (900.0, 600.0), // Full Spotify web player
            ItemContent::Markdown { .. } => (200.0, 36.0), // Simple filename button
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            ItemContent::Image(path) | ItemContent::Video(path) | ItemContent::Audio(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ItemContent::Pdf { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ItemContent::Text(text) => text.clone(),
            ItemContent::Link(url) => url.clone(),
            ItemContent::YouTube(id) => format!("YouTube: {}", id),
            ItemContent::Spotify { content_type, content_id } => {
                format!("Spotify {}: {}", content_type.as_str(), content_id)
            }
            ItemContent::SpotifyApp => "Spotify".to_string(),
            ItemContent::Markdown { title, .. } => title.clone(),
        }
    }

    pub fn type_label(&self) -> &str {
        match self {
            ItemContent::Image(_) => "IMAGE",
            ItemContent::Video(_) => "VIDEO",
            ItemContent::Audio(_) => "AUDIO",
            ItemContent::Pdf { .. } => "PDF",
            ItemContent::Text(_) => "TEXT",
            ItemContent::Link(_) => "LINK",
            ItemContent::YouTube(_) => "YOUTUBE",
            ItemContent::Spotify { .. } => "SPOTIFY",
            ItemContent::SpotifyApp => "SPOTIFY",
            ItemContent::Markdown { .. } => "MARKDOWN",
        }
    }

    pub fn from_path(path: &PathBuf) -> Self {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" => {
                    ItemContent::Image(path.clone())
                }
                "mp4" | "mov" | "avi" | "webm" | "mkv" => ItemContent::Video(path.clone()),
                "mp3" | "wav" | "ogg" | "m4a" | "aac" | "flac" => ItemContent::Audio(path.clone()),
                "pdf" => {
                    let thumbnail = generate_pdf_thumbnail(path);
                    ItemContent::Pdf {
                        path: path.clone(),
                        thumbnail,
                    }
                }
                "md" => {
                    let title = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Untitled")
                        .to_string();
                    let content = std::fs::read_to_string(path).unwrap_or_default();
                    ItemContent::Markdown {
                        path: path.clone(),
                        title,
                        content,
                    }
                }
                "txt" | "rs" | "js" | "json" | "html" | "css" => {
                    ItemContent::Text(format!("{}", path.file_name().unwrap().to_string_lossy()))
                }
                _ => ItemContent::Text(format!("{}", path.file_name().unwrap().to_string_lossy())),
            }
        } else {
            ItemContent::Text(format!("{}", path.file_name().unwrap().to_string_lossy()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_content_from_path_image() {
        let path = PathBuf::from("/test/image.png");
        let content = ItemContent::from_path(&path);
        assert!(matches!(content, ItemContent::Image(_)));
    }

    #[test]
    fn test_item_content_from_path_video() {
        let path = PathBuf::from("/test/video.mp4");
        let content = ItemContent::from_path(&path);
        assert!(matches!(content, ItemContent::Video(_)));
    }

    #[test]
    fn test_item_content_from_path_pdf() {
        let path = PathBuf::from("/test/document.pdf");
        let content = ItemContent::from_path(&path);
        assert!(matches!(content, ItemContent::Pdf { .. }));
    }

    #[test]
    fn test_item_content_from_path_text() {
        let path = PathBuf::from("/test/code.rs");
        let content = ItemContent::from_path(&path);
        assert!(matches!(content, ItemContent::Text(_)));
    }

    #[test]
    fn test_item_content_from_path_unknown() {
        let path = PathBuf::from("/test/file.xyz");
        let content = ItemContent::from_path(&path);
        assert!(matches!(content, ItemContent::Text(_)));
    }

    #[test]
    fn test_type_labels() {
        assert_eq!(ItemContent::Image(PathBuf::new()).type_label(), "IMAGE");
        assert_eq!(ItemContent::Video(PathBuf::new()).type_label(), "VIDEO");
        assert_eq!(
            ItemContent::Pdf {
                path: PathBuf::new(),
                thumbnail: None
            }
            .type_label(),
            "PDF"
        );
        assert_eq!(ItemContent::Text(String::new()).type_label(), "TEXT");
        assert_eq!(ItemContent::Link(String::new()).type_label(), "LINK");
    }

    #[test]
    fn test_display_name_file() {
        let content = ItemContent::Image(PathBuf::from("/path/to/photo.jpg"));
        assert_eq!(content.display_name(), "photo.jpg");
    }

    #[test]
    fn test_display_name_text() {
        let content = ItemContent::Text("Hello World".to_string());
        assert_eq!(content.display_name(), "Hello World");
    }

    #[test]
    fn test_display_name_link() {
        let content = ItemContent::Link("https://example.com".to_string());
        assert_eq!(content.display_name(), "https://example.com");
    }

    #[test]
    fn test_default_size_text() {
        let content = ItemContent::Text("test".to_string());
        assert_eq!(content.default_size(), (300.0, 100.0));
    }

    #[test]
    fn test_default_size_video() {
        let content = ItemContent::Video(PathBuf::new());
        assert_eq!(content.default_size(), (400.0, 300.0));
    }

    #[test]
    fn test_default_size_pdf() {
        let content = ItemContent::Pdf {
            path: PathBuf::new(),
            thumbnail: None,
        };
        assert_eq!(content.default_size(), (250.0, 350.0));
    }

    #[test]
    fn test_default_size_link() {
        let content = ItemContent::Link(String::new());
        assert_eq!(content.default_size(), (300.0, 150.0));
    }

    #[test]
    fn test_canvas_item_creation() {
        let item = CanvasItem {
            id: 1,
            position: (100.0, 200.0),
            size: (300.0, 400.0),
            content: ItemContent::Text("Test".to_string()),
        };
        assert_eq!(item.id, 1);
        assert_eq!(item.position, (100.0, 200.0));
        assert_eq!(item.size, (300.0, 400.0));
    }

    #[test]
    fn test_image_extensions() {
        let extensions = ["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg"];
        for ext in extensions {
            let path = PathBuf::from(format!("/test/file.{}", ext));
            let content = ItemContent::from_path(&path);
            assert!(
                matches!(content, ItemContent::Image(_)),
                "Failed for {}",
                ext
            );
        }
    }

    #[test]
    fn test_video_extensions() {
        let extensions = ["mp4", "mov", "avi", "webm", "mkv"];
        for ext in extensions {
            let path = PathBuf::from(format!("/test/file.{}", ext));
            let content = ItemContent::from_path(&path);
            assert!(
                matches!(content, ItemContent::Video(_)),
                "Failed for {}",
                ext
            );
        }
    }

    #[test]
    fn test_audio_extensions() {
        let extensions = ["mp3", "wav", "ogg", "m4a", "aac", "flac"];
        for ext in extensions {
            let path = PathBuf::from(format!("/test/file.{}", ext));
            let content = ItemContent::from_path(&path);
            assert!(
                matches!(content, ItemContent::Audio(_)),
                "Failed for {}",
                ext
            );
        }
    }

    #[test]
    fn test_item_content_from_path_audio() {
        let path = PathBuf::from("/test/audio.mp3");
        let content = ItemContent::from_path(&path);
        assert!(matches!(content, ItemContent::Audio(_)));
    }

    #[test]
    fn test_default_size_audio() {
        let content = ItemContent::Audio(PathBuf::new());
        assert_eq!(content.default_size(), (320.0, 160.0));
    }

    #[test]
    fn test_type_label_audio() {
        assert_eq!(ItemContent::Audio(PathBuf::new()).type_label(), "AUDIO");
    }
}
