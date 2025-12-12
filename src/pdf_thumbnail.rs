use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

/// Generate a thumbnail image for a PDF's first page
pub fn generate_pdf_thumbnail<P: AsRef<Path>>(pdf_path: P) -> Option<PathBuf> {
    let pdf_path = pdf_path.as_ref();

    // Load pdfium
    let pdfium = load_pdfium().ok()?;

    // Load PDF document
    let document = pdfium.load_pdf_from_file(pdf_path, None).ok()?;

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
    image
        .write_to(
            &mut std::io::Cursor::new(&mut png_data),
            image::ImageFormat::Png,
        )
        .ok()?;
    std::fs::write(&thumbnail_path, &png_data).ok()?;

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
