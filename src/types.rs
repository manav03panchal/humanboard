//! Type definitions for canvas items and content.
//!
//! This module defines the core data structures for items on the canvas.

use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::pdf_thumbnail::generate_pdf_thumbnail;

/// Default fallback size for images that fail to load
const DEFAULT_IMAGE_SIZE: (f32, f32) = (800.0, 600.0);

/// Maximum dimension for scaled images
const MAX_IMAGE_DIMENSION: f32 = 800.0;

/// A canvas item with position, size, and content
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasItem {
    pub id: u64,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub content: ItemContent,
}

impl CanvasItem {
    /// Create a new canvas item
    pub fn new(id: u64, position: (f32, f32), content: ItemContent) -> Self {
        let size = content.default_size();
        Self {
            id,
            position,
            size,
            content,
        }
    }

    /// Get the bounds of this item in canvas coordinates
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (
            self.position.0,
            self.position.1,
            self.position.0 + self.size.0,
            self.position.1 + self.size.1,
        )
    }

    /// Check if a point is inside this item's bounds
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.position.0
            && x <= self.position.0 + self.size.0
            && y >= self.position.1
            && y <= self.position.1 + self.size.1
    }
}

/// Content types that can be displayed on the canvas
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemContent {
    /// An image file
    Image(PathBuf),
    /// Plain text
    Text(String),
    /// A video file
    Video(PathBuf),
    /// A PDF document with optional thumbnail
    Pdf {
        path: PathBuf,
        thumbnail: Option<PathBuf>,
    },
    /// A URL link
    Link(String),
}

impl ItemContent {
    /// Get the default size for this content type
    pub fn default_size(&self) -> (f32, f32) {
        match self {
            ItemContent::Image(path) => compute_image_size(path),
            ItemContent::Text(_) => (300.0, 100.0),
            ItemContent::Video(_) => (400.0, 300.0),
            ItemContent::Pdf { .. } => (250.0, 350.0),
            ItemContent::Link(_) => (300.0, 150.0),
        }
    }

    /// Get the display name for this content
    pub fn display_name(&self) -> String {
        match self {
            ItemContent::Image(path) | ItemContent::Video(path) => {
                get_filename(path).unwrap_or_else(|| "Unknown".to_string())
            }
            ItemContent::Pdf { path, .. } => {
                get_filename(path).unwrap_or_else(|| "Unknown".to_string())
            }
            ItemContent::Text(text) => text.clone(),
            ItemContent::Link(url) => url.clone(),
        }
    }

    /// Get a short type label for this content
    pub fn type_label(&self) -> &'static str {
        match self {
            ItemContent::Image(_) => "IMAGE",
            ItemContent::Video(_) => "VIDEO",
            ItemContent::Pdf { .. } => "PDF",
            ItemContent::Text(_) => "TEXT",
            ItemContent::Link(_) => "LINK",
        }
    }

    /// Create content from a file path, detecting the type by extension
    pub fn from_path(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());

        match ext.as_deref() {
            Some("jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg") => {
                ItemContent::Image(path.to_path_buf())
            }
            Some("mp4" | "mov" | "avi" | "webm" | "mkv") => ItemContent::Video(path.to_path_buf()),
            Some("pdf") => {
                let thumbnail = generate_pdf_thumbnail(path);
                ItemContent::Pdf {
                    path: path.to_path_buf(),
                    thumbnail,
                }
            }
            _ => {
                // For text files or unknown types, use filename as text
                let filename = get_filename(path).unwrap_or_else(|| "Unknown file".to_string());
                ItemContent::Text(filename)
            }
        }
    }

    /// Check if this is an image content
    pub fn is_image(&self) -> bool {
        matches!(self, ItemContent::Image(_))
    }

    /// Check if this is a PDF content
    pub fn is_pdf(&self) -> bool {
        matches!(self, ItemContent::Pdf { .. })
    }

    /// Get the file path if this content has one
    pub fn path(&self) -> Option<&Path> {
        match self {
            ItemContent::Image(p) | ItemContent::Video(p) => Some(p),
            ItemContent::Pdf { path, .. } => Some(path),
            ItemContent::Text(_) | ItemContent::Link(_) => None,
        }
    }
}

/// Compute the display size for an image, scaling to fit within max dimensions
fn compute_image_size(path: &Path) -> (f32, f32) {
    match image::open(path) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            scale_dimensions(width as f32, height as f32, MAX_IMAGE_DIMENSION)
        }
        Err(_) => DEFAULT_IMAGE_SIZE,
    }
}

/// Scale dimensions to fit within a maximum size while preserving aspect ratio
fn scale_dimensions(width: f32, height: f32, max_size: f32) -> (f32, f32) {
    let aspect_ratio = width / height;

    if width > height {
        if width > max_size {
            (max_size, max_size / aspect_ratio)
        } else {
            (width, height)
        }
    } else {
        if height > max_size {
            (max_size * aspect_ratio, max_size)
        } else {
            (width, height)
        }
    }
}

/// Get the filename from a path as a String
fn get_filename(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
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
    fn test_canvas_item_contains_point() {
        let item = CanvasItem {
            id: 1,
            position: (100.0, 100.0),
            size: (200.0, 150.0),
            content: ItemContent::Text("Test".to_string()),
        };

        assert!(item.contains_point(150.0, 150.0)); // Center
        assert!(item.contains_point(100.0, 100.0)); // Top-left
        assert!(item.contains_point(300.0, 250.0)); // Bottom-right
        assert!(!item.contains_point(50.0, 50.0)); // Outside
        assert!(!item.contains_point(350.0, 150.0)); // Outside right
    }

    #[test]
    fn test_canvas_item_bounds() {
        let item = CanvasItem {
            id: 1,
            position: (100.0, 200.0),
            size: (300.0, 400.0),
            content: ItemContent::Text("Test".to_string()),
        };

        let (x1, y1, x2, y2) = item.bounds();
        assert_eq!(x1, 100.0);
        assert_eq!(y1, 200.0);
        assert_eq!(x2, 400.0);
        assert_eq!(y2, 600.0);
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
    fn test_scale_dimensions() {
        // Wide image that needs scaling
        let (w, h) = scale_dimensions(1600.0, 900.0, 800.0);
        assert_eq!(w, 800.0);
        assert!((h - 450.0).abs() < 0.01);

        // Tall image that needs scaling
        let (w, h) = scale_dimensions(600.0, 1200.0, 800.0);
        assert!((w - 400.0).abs() < 0.01);
        assert_eq!(h, 800.0);

        // Small image - no scaling needed
        let (w, h) = scale_dimensions(400.0, 300.0, 800.0);
        assert_eq!(w, 400.0);
        assert_eq!(h, 300.0);
    }

    #[test]
    fn test_content_is_image() {
        assert!(ItemContent::Image(PathBuf::new()).is_image());
        assert!(!ItemContent::Text("test".to_string()).is_image());
    }

    #[test]
    fn test_content_is_pdf() {
        assert!(
            ItemContent::Pdf {
                path: PathBuf::new(),
                thumbnail: None
            }
            .is_pdf()
        );
        assert!(!ItemContent::Image(PathBuf::new()).is_pdf());
    }

    #[test]
    fn test_content_path() {
        let img = ItemContent::Image(PathBuf::from("/test/img.png"));
        assert_eq!(img.path(), Some(Path::new("/test/img.png")));

        let text = ItemContent::Text("hello".to_string());
        assert_eq!(text.path(), None);
    }
}
