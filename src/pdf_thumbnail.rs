use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};
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

    // Save to temp directory
    let temp_dir = std::env::temp_dir()
        .join("humanboard")
        .join("pdf_thumbnails");
    std::fs::create_dir_all(&temp_dir).ok()?;

    // Create unique filename from PDF path
    let pdf_name = pdf_path.file_stem()?.to_string_lossy();
    let thumbnail_path = temp_dir.join(format!("{}_thumb.png", pdf_name));

    // Save as PNG
    let mut png_data = Vec::new();
    if let Err(e) = image.write_to(
        &mut std::io::Cursor::new(&mut png_data),
        image::ImageFormat::Png,
    ) {
        warn!("Failed to encode PDF thumbnail as PNG: {:?}", e);
        return None;
    }

    if let Err(e) = std::fs::write(&thumbnail_path, &png_data) {
        warn!("Failed to save PDF thumbnail to {:?}: {:?}", thumbnail_path, e);
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
