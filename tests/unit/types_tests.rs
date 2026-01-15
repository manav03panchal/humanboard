//! Unit tests for types module.

use humanboard::types::{CanvasItem, ItemContent};
use std::path::PathBuf;

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
fn test_item_content_from_path_code() {
    // .rs files are recognized as code, not text
    let path = PathBuf::from("/test/code.rs");
    let content = ItemContent::from_path(&path);
    assert!(matches!(content, ItemContent::Code { .. }));
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
    assert_eq!(content.default_size(), (180.0, 240.0));
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
