use super::build_time_image::BuildTimeImage;
use super::build_time_resized_image::BuildTimeResizedImage;
use assets_runtime::ImageAsset;

pub trait ImageAssetExtension {
    fn from_build_time_image(build_time_image: &BuildTimeImage) -> Self;
}

impl ImageAssetExtension for ImageAsset {
    fn from_build_time_image(build_time_image: &BuildTimeImage) -> Self {
        ImageAsset {
            alt: build_time_image.alt.clone(),
            placeholder: build_time_image.placeholder.clone(),
            width: build_time_image.width,
            height: build_time_image.height,
            srcset: generate_srcset(&build_time_image.resized_copies),
            src: generate_src(build_time_image),
            mime_type: build_time_image.mime_type.clone(),
        }
    }
}

fn generate_src(built_image: &BuildTimeImage) -> String {
    // If their browser doesn't have support for the srcset attribute,
    // it's probably an old mobile browser. If that's the case, they
    // also probably don't have a lot of bandwidth so go with the smallest
    // image possible.
    let narrowest = built_image
        .resized_copies
        .iter()
        .min_by_key(|resized_copy| resized_copy.width)
        .expect("Received a built image with no resized copies.");

    assets_runtime::asset_url_path(&narrowest.path_starting_from_images_dir)
        .to_string_lossy()
        .to_string()
}

fn generate_srcset(resized_copies: &[BuildTimeResizedImage]) -> String {
    resized_copies
        .iter()
        .map(|resized_copy| {
            let width = resized_copy.width;
            let path = assets_runtime::asset_url_path(&resized_copy.path_starting_from_images_dir);
            let path_str = path.to_str().unwrap();
            format!("{path_str} {width}w")
        })
        .collect::<Vec<String>>()
        .join(", ")
}
