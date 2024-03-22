use assets_runtime::LQIP_MIME_TYPE;
use base64::Engine;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;

pub trait DynamicImageExtension {
    fn resize_to_width(&self, new_width: u32) -> Self;
    fn to_data_uri(&self) -> String;
    fn to_bytes_with_format(&self, format: ImageFormat) -> Vec<u8>;
    fn height_if_resized_to_width(&self, new_width: u32) -> u32;
}

impl DynamicImageExtension for DynamicImage {
    fn resize_to_width(&self, new_width: u32) -> Self {
        let new_height = self.height_if_resized_to_width(new_width);
        self.resize(new_width, new_height, FilterType::Lanczos3)
    }

    fn height_if_resized_to_width(&self, new_width: u32) -> u32 {
        let (width, height) = self.dimensions();

        let new_width_f32 = new_width as f32;
        let height_f32 = height as f32;
        let width_f32 = width as f32;

        let scale = new_width_f32 / width_f32;

        let new_height_f32 = height_f32 * scale;
        new_height_f32.ceil() as u32
    }

    fn to_data_uri(&self) -> String {
        let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        self.write_to(&mut bytes, ImageFormat::Jpeg)
            .expect("Error encoding low quality image placeholder.");
        let base64_encoded = base64::engine::general_purpose::STANDARD.encode(bytes.into_inner());

        format!(
            "data:{mime_type};base64,{base64}",
            mime_type = LQIP_MIME_TYPE,
            base64 = base64_encoded
        )
    }

    fn to_bytes_with_format(&self, format: ImageFormat) -> Vec<u8> {
        let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        self.write_to(&mut bytes, format)
            .expect("Error encoding image.");
        bytes.into_inner()
    }
}
