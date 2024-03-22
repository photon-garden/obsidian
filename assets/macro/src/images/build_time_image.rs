use super::build_time_resized_image::*;
use super::dynamic_image_extension::DynamicImageExtension;
use assets_runtime::Placeholder;
use image::{DynamicImage, GenericImageView};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct BuildTimeImage {
    pub absolute_path_to_original_image: PathBuf,
    pub path_starting_from_images_dir: PathBuf,
    pub resized_copies: Vec<BuildTimeResizedImage>,
    pub placeholder: Placeholder,
    pub width: u32,
    pub height: u32,
    pub name_in_source_code: String,
    pub alt: String,
    pub mime_type: mime::Mime,
}

impl BuildTimeImage {
    pub fn new(
        path_to_images_dir: &Path,
        absolute_path_to_original_image: PathBuf,
        original_image: DynamicImage,
        placeholder_to_generate: PlaceholderToGenerate,
        alt: String,
    ) -> Self {
        let path_starting_from_images_dir = absolute_path_to_original_image
            .strip_prefix(path_to_images_dir)
            .expect("Error stripping prefix from absolute path to original image.")
            .to_path_buf();
        let (width, height) = original_image.dimensions();
        let resized_copies = Self::resized_copies(
            path_to_images_dir,
            &absolute_path_to_original_image,
            &original_image,
        );
        let placeholder = placeholder_to_generate.to_placeholder(&original_image);

        let file_stem = absolute_path_to_original_image
            .file_stem()
            .expect("Error parsing file stem.")
            .to_string_lossy()
            .to_string();

        let name_in_source_code = file_stem.replace(
            |character: char| {
                let should_keep = character.is_alphanumeric() || character == '_';
                !should_keep
            },
            "_",
        );

        BuildTimeImage {
            path_starting_from_images_dir,
            absolute_path_to_original_image,
            resized_copies,
            placeholder,
            width,
            height,
            name_in_source_code,
            alt,
            mime_type: assets_runtime::RESIZED_IMAGE_MIME_TYPE,
        }
    }

    fn resized_copies(
        path_to_images_dir: &Path,
        absolute_path_to_original_image: &Path,
        original_image: &DynamicImage,
    ) -> Vec<BuildTimeResizedImage> {
        let original_width = original_image.width();

        Self::available_widths(original_width)
            .into_iter()
            .map(|target_width| {
                BuildTimeResizedImage::new(
                    target_width,
                    path_to_images_dir,
                    absolute_path_to_original_image,
                    original_image,
                )
            })
            .collect()
    }

    fn available_widths(image_width: u32) -> Vec<u32> {
        Self::possible_widths()
            .into_iter()
            .filter(|possible_width| possible_width <= &image_width)
            .collect()
    }

    fn possible_widths() -> Vec<u32> {
        (100..=4000).step_by(100).collect()
    }
}

pub enum PlaceholderToGenerate {
    Lqip,
    AutomaticallyDetectedColor,
}

impl PlaceholderToGenerate {
    pub fn to_placeholder(&self, original_image: &DynamicImage) -> Placeholder {
        match self {
            // Returns a string like "data:image/jpeg;base64,/9j/4AAQSk...".
            PlaceholderToGenerate::Lqip => {
                let data_uri = original_image.resize_to_width(40).to_data_uri();
                Placeholder::Lqip { data_uri }
            }

            // Returns a string like "rgba(255, 255, 255, 1.0)".
            PlaceholderToGenerate::AutomaticallyDetectedColor => {
                let [red, green, blue, alpha] = original_image
                    .resize_exact(1, 1, image::imageops::Lanczos3)
                    .get_pixel(0, 0)
                    .0;

                let css_string = format!(
                    "rgba({red}, {green}, {blue}, {alpha})",
                    red = red,
                    green = green,
                    blue = blue,
                    alpha = alpha
                );

                Placeholder::Color { css_string }
            }
        }
    }
}
