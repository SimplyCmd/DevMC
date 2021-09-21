use eframe::{egui, epi};
use eframe::egui::TextureId;

pub fn load_image(frame: &mut epi::Frame<'_>) -> TextureId {
    // Load the image:
    let image_data = include_bytes!("add.png");
    use image::GenericImageView;
    let image = image::load_from_memory(image_data).expect("Failed to load image");
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.into_vec();
    let pixels: Vec<_> = pixels
        .chunks_exact(4)
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    // Allocate a texture:
    let texture_id = frame
        .tex_allocator()
        .alloc_srgba_premultiplied((image.width() as usize, image.height() as usize), &pixels);
    return texture_id;
}