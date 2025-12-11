//! PDF document handling module.
//!
//! Provides PDF rendering with proper caching using content-based hashing.

use pdfium_render::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// Base width for high-quality PDF rendering
const BASE_RENDER_WIDTH: i32 = 2400;

/// Maximum render height
const MAX_RENDER_HEIGHT: i32 = 3200;

/// Zoom range limits
const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 5.0;

/// PDF document with page caching
pub struct PdfDocument {
    pdf_path: PathBuf,
    pub page_count: usize,
    pub current_page: usize,
    pub zoom: f32,
    /// Vertical scroll offset for continuous scrolling
    pub scroll_offset: f32,
    /// Cached rendered page images: (page_num, zoom_key) -> path
    page_cache: HashMap<(usize, i32), PathBuf>,
    /// Hash of the PDF path for unique cache filenames
    path_hash: u64,
}

impl PdfDocument {
    /// Open a PDF document
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, PdfError> {
        let pdf_path = path.as_ref().to_path_buf();
        let pdfium = load_pdfium()?;

        let document = pdfium
            .load_pdf_from_file(&pdf_path, None)
            .map_err(|e| PdfError::LoadFailed(format!("{:?}", e)))?;

        let page_count = document.pages().len() as usize;
        let path_hash = compute_path_hash(&pdf_path);

        Ok(Self {
            pdf_path,
            page_count,
            current_page: 0,
            zoom: 1.0,
            scroll_offset: 0.0,
            page_cache: HashMap::new(),
            path_hash,
        })
    }

    /// Render a page and cache it. Returns path to the cached PNG.
    pub fn render_page(&mut self, page_num: usize) -> Option<PathBuf> {
        // Quantize zoom level for cache key
        let zoom_key = (self.zoom * 10.0) as i32;
        let cache_key = (page_num, zoom_key);

        // Check cache first
        if let Some(cached_path) = self.page_cache.get(&cache_key) {
            if cached_path.exists() {
                return Some(cached_path.clone());
            }
        }

        // Render the page
        self.render_page_to_cache(page_num, zoom_key, cache_key)
    }

    /// Actually render a page to disk
    fn render_page_to_cache(
        &mut self,
        page_num: usize,
        zoom_key: i32,
        cache_key: (usize, i32),
    ) -> Option<PathBuf> {
        let pdfium = load_pdfium().ok()?;
        let document = pdfium.load_pdf_from_file(&self.pdf_path, None).ok()?;
        let page = document.pages().get(page_num as u16).ok()?;

        let target_width = (BASE_RENDER_WIDTH as f32 * self.zoom) as i32;
        let max_height = (MAX_RENDER_HEIGHT as f32 * self.zoom) as i32;

        let render_config = PdfRenderConfig::new()
            .set_target_width(target_width)
            .set_maximum_height(max_height);

        let bitmap = page.render_with_config(&render_config).ok()?;
        let image = bitmap.as_image();

        // Create cache directory
        let temp_dir = std::env::temp_dir().join("humanboard").join("pdf_pages");
        if std::fs::create_dir_all(&temp_dir).is_err() {
            return None;
        }

        // Use proper hash for unique filename
        let image_path = temp_dir.join(format!(
            "{:016x}_{}_{}.png",
            self.path_hash, page_num, zoom_key
        ));

        // Save as PNG
        let mut png_data = Vec::new();
        if image
            .write_to(
                &mut std::io::Cursor::new(&mut png_data),
                image::ImageFormat::Png,
            )
            .is_err()
        {
            return None;
        }

        if std::fs::write(&image_path, &png_data).is_err() {
            return None;
        }

        self.page_cache.insert(cache_key, image_path.clone());
        Some(image_path)
    }

    /// Get the current page image, rendering if needed
    pub fn get_current_page_image(&mut self) -> Option<PathBuf> {
        self.render_page(self.current_page)
    }

    /// Navigate to the next page
    pub fn next_page(&mut self) -> bool {
        if self.current_page < self.page_count.saturating_sub(1) {
            self.current_page += 1;
            true
        } else {
            false
        }
    }

    /// Navigate to the previous page
    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }

    /// Jump to a specific page
    pub fn go_to_page(&mut self, page: usize) -> bool {
        if page < self.page_count {
            self.current_page = page;
            true
        } else {
            false
        }
    }

    /// Zoom in by a step
    pub fn zoom_in(&mut self) -> bool {
        if self.zoom < MAX_ZOOM {
            self.zoom = (self.zoom * 1.25).min(MAX_ZOOM);
            true
        } else {
            false
        }
    }

    /// Zoom out by a step
    pub fn zoom_out(&mut self) -> bool {
        if self.zoom > MIN_ZOOM {
            self.zoom = (self.zoom / 1.25).max(MIN_ZOOM);
            true
        } else {
            false
        }
    }

    /// Reset zoom to 100%
    pub fn zoom_reset(&mut self) {
        self.zoom = 1.0;
    }

    /// Set zoom to a specific value (clamped to valid range)
    pub fn set_zoom(&mut self, new_zoom: f32) {
        self.zoom = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    }

    /// Handle scroll in PDF viewer
    pub fn handle_scroll(&mut self, delta_y: f32) -> bool {
        self.scroll_offset -= delta_y;

        // Keep scroll offset in reasonable bounds
        let max_scroll = (self.page_count as f32 - 1.0) * 100.0;
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);

        // Update current page based on scroll position
        let new_page = (self.scroll_offset / 100.0).floor() as usize;
        let changed_page = new_page != self.current_page;

        if new_page < self.page_count {
            self.current_page = new_page;
        }

        changed_page
    }

    /// Clear the page cache
    pub fn clear_cache(&mut self) {
        self.page_cache.clear();
    }

    /// Get the PDF file path
    pub fn path(&self) -> &Path {
        &self.pdf_path
    }
}

/// Errors that can occur when working with PDFs
#[derive(Debug)]
pub enum PdfError {
    /// Failed to load pdfium library
    LibraryNotFound(String),
    /// Failed to load the PDF file
    LoadFailed(String),
    /// Failed to render a page
    RenderFailed(String),
}

impl std::fmt::Display for PdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfError::LibraryNotFound(msg) => write!(f, "Pdfium library not found: {}", msg),
            PdfError::LoadFailed(msg) => write!(f, "Failed to load PDF: {}", msg),
            PdfError::RenderFailed(msg) => write!(f, "Failed to render PDF: {}", msg),
        }
    }
}

impl std::error::Error for PdfError {}

/// Compute a hash of the PDF path for unique cache filenames
fn compute_path_hash(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    hasher.finish()
}

/// Load the pdfium library
fn load_pdfium() -> Result<Pdfium, PdfError> {
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
        .map_err(|e| PdfError::LibraryNotFound(format!("{:?}", e)))
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
    fn test_compute_path_hash_different_paths() {
        let path1 = PathBuf::from("/test/doc1.pdf");
        let path2 = PathBuf::from("/test/doc2.pdf");
        assert_ne!(compute_path_hash(&path1), compute_path_hash(&path2));
    }

    #[test]
    fn test_zoom_limits() {
        let mut doc = PdfDocument {
            pdf_path: PathBuf::new(),
            page_count: 10,
            current_page: 0,
            zoom: 1.0,
            scroll_offset: 0.0,
            page_cache: HashMap::new(),
            path_hash: 0,
        };

        // Test zoom in limit
        doc.zoom = MAX_ZOOM;
        assert!(!doc.zoom_in());

        // Test zoom out limit
        doc.zoom = MIN_ZOOM;
        assert!(!doc.zoom_out());
    }

    #[test]
    fn test_page_navigation() {
        let mut doc = PdfDocument {
            pdf_path: PathBuf::new(),
            page_count: 10,
            current_page: 0,
            zoom: 1.0,
            scroll_offset: 0.0,
            page_cache: HashMap::new(),
            path_hash: 0,
        };

        // Test next page
        assert!(doc.next_page());
        assert_eq!(doc.current_page, 1);

        // Test prev page
        assert!(doc.prev_page());
        assert_eq!(doc.current_page, 0);

        // Test prev at first page
        assert!(!doc.prev_page());

        // Test go to last page
        assert!(doc.go_to_page(9));
        assert_eq!(doc.current_page, 9);

        // Test next at last page
        assert!(!doc.next_page());

        // Test invalid page
        assert!(!doc.go_to_page(100));
    }
}
