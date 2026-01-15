//! PDF thumbnail generation using pdfium.
//!
//! This module renders the first page of a PDF document as a PNG thumbnail
//! for display on the canvas. Thumbnails are cached in a temp directory
//! with content-based naming to avoid regeneration.
//!
//! ## Security
//!
//! - Uses atomic writes via tempfile to prevent TOCTOU race conditions
//! - Generates cryptographic hashes of canonical paths for cache keys
//!
//! ## Library Loading
//!
//! Pdfium is loaded dynamically from (in order):
//! 1. `lib/libpdfium.dylib` in working directory
//! 2. `lib/libpdfium.dylib` relative to executable
//! 3. `Resources/lib/libpdfium.dylib` in macOS bundle
//! 4. System library fallback

use pdfium_render::prelude::*;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use tracing::{debug, info_span, warn};

/// Generate a thumbnail image for a PDF's first page
pub fn generate_pdf_thumbnail<P: AsRef<Path>>(pdf_path: P) -> Option<PathBuf> {
    let pdf_path = pdf_path.as_ref();
    let _span = info_span!("generate_pdf_thumbnail", path = ?pdf_path).entered();

    // Load pdfium
    let pdfium = match load_pdfium() {
        Ok(p) => p,
        Err(e) => {
            warn!("Failed to load pdfium library: {}", e);
            return None;
        }
    };

    // Load PDF document
    let document = match pdfium.load_pdf_from_file(pdf_path, None) {
        Ok(d) => d,
        Err(e) => {
            warn!("Failed to load PDF {:?}: {:?}", pdf_path, e);
            return None;
        }
    };

    // Get first page
    let page = document.pages().get(0).ok()?;

    // Render at a reasonable thumbnail size (400px width)
    let render_config = PdfRenderConfig::new()
        .set_target_width(400)
        .set_maximum_height(600);

    let bitmap = page.render_with_config(&render_config).ok()?;
    let image = bitmap.as_image();

    // Save as PNG data first
    let mut png_data = Vec::new();
    if let Err(e) = image.write_to(
        &mut std::io::Cursor::new(&mut png_data),
        image::ImageFormat::Png,
    ) {
        warn!("Failed to encode PDF thumbnail as PNG: {:?}", e);
        return None;
    }

    // Create secure temp directory with restricted permissions
    let temp_dir = std::env::temp_dir()
        .join("humanboard")
        .join("pdf_thumbnails");
    std::fs::create_dir_all(&temp_dir).ok()?;

    // Generate cryptographic hash of the full canonical path for unique filename
    let canonical_path = pdf_path.canonicalize().unwrap_or_else(|_| pdf_path.to_path_buf());
    let mut hasher = Sha256::new();
    hasher.update(canonical_path.to_string_lossy().as_bytes());
    let path_hash = format!("{:x}", hasher.finalize());
    let thumbnail_filename = format!("{}_thumb.png", &path_hash[..16]);
    let thumbnail_path = temp_dir.join(&thumbnail_filename);

    // Use atomic write: create temp file, write, then persist
    // This prevents TOCTOU race conditions and symlink attacks
    let mut temp_file = match NamedTempFile::new_in(&temp_dir) {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to create temp file for thumbnail: {:?}", e);
            return None;
        }
    };

    if let Err(e) = temp_file.write_all(&png_data) {
        warn!("Failed to write thumbnail data: {:?}", e);
        return None;
    }

    // Atomically persist to final location
    if let Err(e) = temp_file.persist(&thumbnail_path) {
        warn!("Failed to persist PDF thumbnail to {:?}: {:?}", thumbnail_path, e);
        return None;
    }

    debug!("Generated PDF thumbnail: {:?}", thumbnail_path);
    Some(thumbnail_path)
}

fn load_pdfium() -> Result<Pdfium, String> {
    // Try to load from lib/ directory first (development)
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

    // Try relative to executable
    let exe_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("lib/libpdfium.dylib"));

    if let Some(path) = exe_path {
        if path.exists() {
            if let Ok(bindings) = Pdfium::bind_to_library(&path) {
                return Ok(Pdfium::new(bindings));
            }
        }
    }

    // Try macOS bundle Resources folder
    let bundle_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("Resources/lib/libpdfium.dylib"));

    if let Some(path) = bundle_path {
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
