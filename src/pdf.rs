use pdfium_render::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct PdfDocument {
    pdf_path: PathBuf,
    pub page_count: usize,
    pub current_page: usize,
    page_cache: HashMap<usize, PathBuf>, // Cached rendered page images
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
            page_cache: HashMap::new(),
        })
    }

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

    /// Render a single page and cache it. Returns path to the cached PNG.
    fn render_page(&mut self, page_num: usize) -> Option<PathBuf> {
        // Check cache first
        if let Some(cached_path) = self.page_cache.get(&page_num) {
            if cached_path.exists() {
                return Some(cached_path.clone());
            }
        }

        // Render the page
        let pdfium = Self::load_pdfium().ok()?;
        let document = pdfium.load_pdf_from_file(&self.pdf_path, None).ok()?;
        let page = document.pages().get(page_num as u16).ok()?;

        let render_config = PdfRenderConfig::new()
            .set_target_width(1000)
            .set_maximum_height(1400);

        let bitmap = page.render_with_config(&render_config).ok()?;
        let image = bitmap.as_image();

        // Save to temp file
        let temp_dir = std::env::temp_dir().join("humanboard").join("pdf_pages");
        std::fs::create_dir_all(&temp_dir).ok()?;

        // Use hash of pdf path + page number for unique filename
        let pdf_hash = self.pdf_path.to_string_lossy().len(); // Simple hash
        let image_path = temp_dir.join(format!("{}_{}.png", pdf_hash, page_num));

        let mut png_data = Vec::new();
        image
            .write_to(
                &mut std::io::Cursor::new(&mut png_data),
                image::ImageFormat::Png,
            )
            .ok()?;
        std::fs::write(&image_path, &png_data).ok()?;

        self.page_cache.insert(page_num, image_path.clone());
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
}
