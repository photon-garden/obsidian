use assets_runtime::ImageAsset;
use build_time_image::*;
use image::DynamicImage;
use image_asset_extension::*;
use include_image_input::*;
use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
// use include_images_in_folder_input::*;
// use quote::format_ident;
// use rayon::prelude::*;
// use walkdir::WalkDir;

mod build_time_image;
mod build_time_resized_image;
mod dynamic_image_extension;
mod image_asset_extension;
mod include_image_input;
mod include_images_in_folder_input;

pub fn include_image(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as IncludeImageInput);
    crate::logger::init_logger(input.debug);

    let absolute_path_to_image = &input.absolute_path_to_image;

    log::info!(
        "Including image: {}",
        input.absolute_path_to_image.display()
    );
    let image_file = try_get_image_file_from_path(absolute_path_to_image)
        .expect("Error getting image file from path.");

    let absolute_path_to_image_dir = absolute_path_to_image
        .parent()
        .expect("Error getting parent directory of image.")
        .to_path_buf();

    let alt = match input.alt {
        Alt::Literal(alt) => alt,
        Alt::Automatic => "fix me".to_string(), // image_captioner::get_caption(absolute_path_to_image).expect("Error getting image caption."),
    };

    let build_time_image = BuildTimeImage::new(
        &absolute_path_to_image_dir,
        absolute_path_to_image.clone(),
        image_file.image,
        input.placeholder_to_generate,
        alt,
    );

    let image_asset = ImageAsset::from_build_time_image(&build_time_image);

    let code = quote! {
        #image_asset
    };

    // print_code_for_debugging(&code);

    code.into()
}

fn try_get_image_file_from_path(path: &Path) -> Option<ImageFile> {
    match image::open(path) {
        Ok(dynamic_image) => {
            let image_file = ImageFile {
                image: dynamic_image,
            };
            Some(image_file)
        }
        Err(error) => {
            log::info!("Error opening file as image {:?}: {:?}", path, error);
            None
        }
    }
}

struct ImageFile {
    image: DynamicImage,
}

#[allow(dead_code)]
fn print_code_for_debugging(token_stream: &proc_macro2::TokenStream) {
    let wrapped_in_main_function = quote! {
        fn main() {
            #token_stream
        }
    };

    let item = syn::parse2::<syn::Item>(wrapped_in_main_function).unwrap();

    let syn_file = syn::File {
        attrs: vec![],
        items: vec![item],
        shebang: None,
    };

    let formatted_code = prettyplease::unparse(&syn_file);

    log::info!("{}", formatted_code);
}

// pub fn include_images_in_folder(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as IncludeImagesInFolderInput);
//     crate::logger::init_logger(input.debug);

//     log::info!("Includeing images.");
//     log::info!(
//         "Path to images directory: {}",
//         input.absolute_path_to_image_dir.display()
//     );
//     let images_to_include = get_images_from_disk(input);
//     let code = generate_code(&images_to_include);

//     // print_code_for_debugging(&code);

//     code.into()
// }

// We wrap our built images in Arcs because we need owned copies
// to use Rayon, and we presume that cloning BuiltImages is expensive
// because they hold giant piles of image bytes.
// fn get_images_from_disk(input: IncludeImagesInFolderInput) -> Vec<BuildTimeImage> {
//     log::info!("Getting original images from disk.");
//     let original_images = get_image_files(&input.absolute_path_to_images_dir);
//     log::info!("Found {} original images.", original_images.len());

//     log::info!("Generating placeholders and saving images to disk if necessary.");
//     original_images
//         .into_par_iter()
//         .map(|image_file| {
//             BuildTimeImage::new(
//                 &input.absolute_path_to_images_dir,
//                 image_file.absolute_path_to_image,
//                 image_file.image,
//                 PlaceholderToGenerate::Lqip,
//                 "fix me".to_string(),
//             )
//         })
//         .collect()
// }

// fn get_image_files(path_to_images_dir: &Path) -> Vec<ImageFile> {
//     WalkDir::new(path_to_images_dir)
//         .into_iter()
//         .filter_map(try_get_image_file_from_dir_entry)
//         .collect()
// }

// fn try_get_image_file_from_dir_entry(
//     maybe_dir_entry: walkdir::Result<walkdir::DirEntry>,
// ) -> Option<ImageFile> {
//     match maybe_dir_entry {
//         Ok(entry) if entry.file_type().is_dir() => None,
//         Ok(entry) => {
//             let path = entry.path();
//             try_get_image_file_from_path(path)
//         }
//         Err(error) => {
//             log::info!("Error reading image file: {:?}", error);
//             None
//         }
//     }
// }

// fn generate_code(images_to_include: &[BuildTimeImage]) -> proc_macro2::TokenStream {
//     log::info!("Generating code for built images.");

//     log::info!("Building property names.");
//     let image_property_names: Vec<_> = images_to_include
//         .iter()
//         .map(|image| format_ident!("{}", image.name_in_source_code))
//         .collect();

//     log::info!("Instantiating ImageAssets.");
//     let image_property_declarations = images_to_include.iter().map(|image| {
//         let name_in_source_code = format_ident!("{}", image.name_in_source_code);
//         let image_asset = ImageAsset::from_build_time_image(image);
//         quote! {
//             #name_in_source_code: #image_asset,
//         }
//     });

//     quote! {
//         {
//             use std::path::PathBuf;
//             use std::str::FromStr;

//             pub struct ImageAssets {
//                 #(
//                     pub #image_property_names: ImageAsset,
//                 )*
//             }

//             ImageAssets {
//                 #( #image_property_declarations )*
//             }
//         }
//     }
// }
