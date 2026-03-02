use captcha::Captcha;
use captcha::filters::{Noise, Wave};
use image::ImageOutputFormat;
use std::io::Cursor;

pub fn generate_captcha(length: u8, difficulty: u8) -> Result<(String, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
    let mut captcha = Captcha::new();
    
    captcha
        .add_chars(length as u32)
        .apply_filter(Noise::new(0.1 * difficulty as f32))
        .apply_filter(Wave::new(2.0, 10.0).horizontal())
        .view(220, 120);

    let code = captcha.chars_as_string();
    
    let image = captcha.as_image().ok_or("Failed to generate captcha image")?;
    
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, ImageOutputFormat::Png)?;
    
    Ok((code, buffer.into_inner()))
}
