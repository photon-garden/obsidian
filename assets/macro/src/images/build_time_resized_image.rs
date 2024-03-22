use super::dynamic_image_extension::DynamicImageExtension;
use image::DynamicImage;
use mime::Mime;
use std::fs;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone)]
pub struct BuildTimeResizedImage {
    pub absolute_path: PathBuf,
    pub path_starting_from_images_dir: PathBuf,
    pub width: u32,
    pub height: u32,
    pub mime_type: Mime,
    pub bytes: Vec<u8>,
}

impl BuildTimeResizedImage {
    pub fn new(
        width: u32,
        path_to_images_dir: &Path,
        absolute_path_to_original_image: &Path,
        original_image: &DynamicImage,
    ) -> Self {
        let path_starting_from_images_dir_without_width = absolute_path_to_original_image
            .strip_prefix(path_to_images_dir)
            .expect("Error stripping prefix from absolute path to original image.")
            .to_path_buf();

        let mime_type = mime::IMAGE_JPEG;

        let path_starting_from_images_dir_with_width =
            Self::path_starting_from_images_dir_with_width(
                &path_starting_from_images_dir_without_width,
                width,
                &mime_type,
            );

        let absolute_path_with_width =
            assets_runtime::paths::built_image_path(&path_starting_from_images_dir_with_width);

        let height = original_image.height_if_resized_to_width(width);

        let bytes = Self::generate_bytes(width, &absolute_path_with_width, original_image);

        Self {
            absolute_path: absolute_path_with_width,
            path_starting_from_images_dir: path_starting_from_images_dir_with_width,
            width,
            height,
            mime_type,
            bytes,
        }
    }

    fn path_starting_from_images_dir_with_width(
        path_starting_from_images_dir_without_width: &Path,
        width: u32,
        mime_type: &Mime,
    ) -> PathBuf {
        let file_name = path_starting_from_images_dir_without_width
            .file_name()
            .expect("Error parsing file name.");
        let file_name = PathBuf::from(file_name);
        let file_name_with_width = Self::file_name_with_width(&file_name, width, mime_type);

        path_starting_from_images_dir_without_width
            .parent()
            .expect("Error getting parent of path starting from images dir.")
            .join(file_name_with_width)
    }

    fn file_name_with_width(file_name: &Path, width: u32, mime_type: &Mime) -> PathBuf {
        let old_file_stem = file_name.file_stem().unwrap().to_str().unwrap();
        let new_file_extension = mime_type.subtype().as_str();
        let new_file_name_string = format!("{}_{}w.{}", old_file_stem, width, new_file_extension);
        PathBuf::from_str(&new_file_name_string).unwrap()
    }

    fn generate_bytes(width: u32, path: &Path, original_image: &DynamicImage) -> Vec<u8> {
        fs::read(path).unwrap_or_else(|error| {
            println!(
                "Couldn't read resized image file {:?} so regenerating the resized image. Original error message: {}",
                &path, error
            );

            let format = image::ImageFormat::from_mime_type(assets_runtime::RESIZED_IMAGE_MIME_TYPE).expect("Error getting image format from mime type.");

            let bytes = original_image
                .resize_to_width(width)
                .to_bytes_with_format(format);

            fs::create_dir_all(path.parent().unwrap()).expect("Error creating built images dir.");
            fs::write(path, &bytes).expect("Error writing resized image to disk.");

            bytes
        })
    }
}
