//! PDF thumbnail generation module.
//!
//! Generates thumbnail images for PDF documents.

use pdfium_render::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// Thumbnail width in pixels
const THUMBNAIL_WIDTH: i32 = 400;

/// Maximum thumbnail height in pixels
const THUMBNAIL_MAX_HEIGHT: i32 = 600;

/// Generate a thumbnail image for a PDF's first page
pub fn generate_pdf_thumbnail<P: AsRef<Path>>(pdf_path: P) -> Option<PathBuf> {
    let pdf_path = pdf_path.as_ref();

    // Load pdfium
    let pdfium = load_pdfium().ok()?;

    // Load PDF document
    let document = pdfium.load_pdf_from_file(pdf_path, None).ok()?;

    // Get first page
    let page = document.pages().get(0).ok()?;

    // Render at thumbnail size
    let render_config = PdfRenderConfig::new()
        .set_target_width(THUMBNAIL_WIDTH)
        .set_maximum_height(THUMBNAIL_MAX_HEIGHT);

    let bitmap = page.render_with_config(&render_config).ok()?;
    let image = bitmap.as_image();

    // Create temp directory for thumbnails
    let temp_dir = std::env::temp_dir()
        .join("humanboard")
        .join("pdf_thumbnails");

    if std::fs::create_dir_all(&temp_dir).is_err() {
        eprintln!("Failed to create thumbnail directory");
        return None;
    }

    // Generate unique filename using proper hash
    let path_hash = compute_path_hash(pdf_path);
    let thumbnail_path = temp_dir.join(format!("{:016x}_thumb.png", path_hash));

    // Save as PNG
    let mut png_data = Vec::new();
    if image
        .write_to(
            &mut std::io::Cursor::new(&mut png_data),
            image::ImageFormat::Png,
        )
        .is_err()
    {
        eprintln!("Failed to encode thumbnail as PNG");
        return None;
    }

    if std::fs::write(&thumbnail_path, &png_data).is_err() {
        eprintln!("Failed to write thumbnail to disk");
        return None;
    }

    Some(thumbnail_path)
}

/// Compute a hash of the path for unique filenames
fn compute_path_hash(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    hasher.finish()
}

/// Load the pdfium library
fn load_pdfium() -> Result<Pdfium, String> {
    // Try to load from lib/ directory first
    let lib_path = std::env::current_dir()
        .ok()
        .map(|p| p.join("lib/libpdfium.dylib"));

    if let Some(path) = lib_path {
        if path.exists() {
            if let Ok(bindings) = Pdfium::bind_to_library(&path) {
                return Ok(Pdfium::new(bindings));
            }
        }
    }

    // Fallback to system library
    Pdfium::bind_to_system_library()
        .map(Pdfium::new)
        .map_err(|e| format!("Failed to load pdfium: {:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_path_hash_consistent() {
        let path = PathBuf::from("/test/document.pdf");
        let hash1 = compute_path_hash(&path);
        let hash2 = compute_path_hash(&path);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_compute_path_hash_unique() {
        let path1 = PathBuf::from("/test/doc1.pdf");
        let path2 = PathBuf::from("/test/doc2.pdf");
        let path3 = PathBuf::from("/other/doc1.pdf");

        let hash1 = compute_path_hash(&path1);
        let hash2 = compute_path_hash(&path2);
        let hash3 = compute_path_hash(&path3);

        // All should be different
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);
    }
}
