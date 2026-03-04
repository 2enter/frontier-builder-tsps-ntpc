use bytes::Bytes;
use image::{GenericImageView, ImageBuffer, ImageFormat, Rgb};

const BG_COLOR: Rgb<u8> = Rgb([255, 222, 193]);

pub fn generate_texture(id: i32, bytes: &Bytes, path: &str) {
    let mut img = image::load_from_memory(bytes).unwrap();

    img.save(format!("{path}/paint/{id}.png")).unwrap();

    let (w, h) = img.dimensions();
    img = img.crop(0, (h - w) / 2, w, w);

    let mut bg = ImageBuffer::from_pixel(w, w, BG_COLOR);

    let img = img.to_rgba8();

    for y in 0..w {
        for x in 0..w {
            let pixel = img.get_pixel(x, y);
            let bg_pixel = bg.get_pixel_mut(x, y);

            let alpha = pixel[3] as f32 / 255.0;
            for i in 0..3 {
                bg_pixel[i] = ((1.0 - alpha) * bg_pixel[i] as f32 + alpha * pixel[i] as f32) as u8;
            }
        }
    }

    bg.save_with_format(format!("{path}/texture/{id}.jpg"), ImageFormat::Jpeg)
        .expect("Failed to save texture image");
}
