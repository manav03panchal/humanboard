use pdfium_render::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

pub struct PdfDocument {
    pdf_path: PathBuf,
    pub page_count: usize,
    pub current_page: usize,
    pub zoom: f32,
    pub scroll_offset: f32, // Vertical scroll offset for continuous scrolling
    page_cache: HashMap<(usize, i32), PathBuf>, // Cached rendered page images (page, zoom_level)
}

impl PdfDocument {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let pdf_path = path.as_ref().to_path_buf();
        let pdfium = Self::load_pdfium()?;

        let document = pdfium
            .load_pdf_from_file(&pdf_path, None)
            .map_err(|e| format!("Failed to load PDF: {:?}", e))?;

        let page_count = document.pages().len() as usize;

        Ok(Self {
            pdf_path,
            page_count,
            current_page: 0,
            zoom: 1.0,
            scroll_offset: 0.0,
            page_cache: HashMap::new(),
        })
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

    /// Render a single page and cache it. Returns path to the cached PNG.
    fn render_page(&mut self, page_num: usize) -> Option<PathBuf> {
        // Use zoom level as cache key (rounded to avoid too many cache entries)
        let zoom_key = (self.zoom * 10.0) as i32;
        let cache_key = (page_num, zoom_key);

        // Check cache first
        if let Some(cached_path) = self.page_cache.get(&cache_key) {
            if cached_path.exists() {
                return Some(cached_path.clone());
            }
        }

        // Render the page
        let pdfium = Self::load_pdfium().ok()?;
        let document = pdfium.load_pdf_from_file(&self.pdf_path, None).ok()?;
        let page = document.pages().get(page_num as u16).ok()?;

        // High resolution rendering for crisp quality at all zoom levels
        let base_width = 2400; // Increased from 1400 for better quality
        let target_width = (base_width as f32 * self.zoom) as i32;

        let render_config = PdfRenderConfig::new()
            .set_target_width(target_width)
            .set_maximum_height((3200.0 * self.zoom) as i32);

        let bitmap = page.render_with_config(&render_config).ok()?;
        let image = bitmap.as_image();

        // Encode image to PNG data first
        let mut png_data = Vec::new();
        image
            .write_to(
                &mut std::io::Cursor::new(&mut png_data),
                image::ImageFormat::Png,
            )
            .ok()?;

        // Save to secure temp file
        let temp_dir = std::env::temp_dir().join("humanboard").join("pdf_pages");
        std::fs::create_dir_all(&temp_dir).ok()?;

        // Generate cryptographic hash of the full canonical path for unique filename
        let canonical_path = self
            .pdf_path
            .canonicalize()
            .unwrap_or_else(|_| self.pdf_path.clone());
        let mut hasher = Sha256::new();
        hasher.update(canonical_path.to_string_lossy().as_bytes());
        let path_hash = format!("{:x}", hasher.finalize());
        let image_path = temp_dir.join(format!("{}_{}_{}.png", &path_hash[..16], page_num, zoom_key));

        // Use atomic write: create temp file, write, then persist
        // This prevents TOCTOU race conditions and symlink attacks
        let mut temp_file = NamedTempFile::new_in(&temp_dir).ok()?;
        temp_file.write_all(&png_data).ok()?;
        temp_file.persist(&image_path).ok()?;

        self.page_cache.insert(cache_key, image_path.clone());
        Some(image_path)
    }

    /// Get the current page image, rendering if needed
    pub fn get_current_page_image(&mut self) -> Option<PathBuf> {
        self.render_page(self.current_page)
    }

    pub fn next_page(&mut self) -> bool {
        if self.current_page < self.page_count.saturating_sub(1) {
            self.current_page += 1;
            true
        } else {
            false
        }
    }

    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }

    pub fn go_to_page(&mut self, page: usize) -> bool {
        if page < self.page_count {
            self.current_page = page;
            true
        } else {
            false
        }
    }

    pub fn zoom_in(&mut self) -> bool {
        if self.zoom < 5.0 {
            self.zoom = (self.zoom * 1.25).min(5.0);
            true
        } else {
            false
        }
    }

    pub fn zoom_out(&mut self) -> bool {
        if self.zoom > 0.25 {
            self.zoom = (self.zoom / 1.25).max(0.25);
            true
        } else {
            false
        }
    }

    pub fn zoom_reset(&mut self) {
        self.zoom = 1.0;
    }

    /// Smooth zoom for scroll wheel - allows any value between limits
    pub fn set_zoom(&mut self, new_zoom: f32) {
        self.zoom = new_zoom.clamp(0.25, 5.0);
    }

    /// Handle scroll in PDF viewer - returns true if we should update
    pub fn handle_scroll(&mut self, delta_y: f32) -> bool {
        // Smooth scrolling through content
        self.scroll_offset -= delta_y;

        // Keep scroll offset in reasonable bounds (0 to some max based on page count)
        let max_scroll = (self.page_count as f32 - 1.0) * 100.0; // Rough estimate
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);

        // Update current page based on scroll position
        let new_page = (self.scroll_offset / 100.0).floor() as usize;
        let changed_page = new_page != self.current_page;

        if new_page < self.page_count {
            self.current_page = new_page;
        }

        changed_page
    }
}
