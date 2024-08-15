use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use image::{DynamicImage, Rgba};
use qrcode::render::{svg, unicode, Renderer};
use qrcode::QrCode;
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg,
};
use rqrr::PreparedImage;

use crate::cli::args::{Arguments, OutputFormat};
use crate::errors::BoxResult;

pub fn make_code(data: &str) -> BoxResult<QrCode> {
    Ok(QrCode::new(data.as_bytes())?)
}

pub fn is_svg_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(OsStr::to_str),
        Some("svg" | "svgz")
    )
}

fn convert_svg_to_image_data(data: &[u8]) -> BoxResult<Vec<u8>> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(data, &options)?;

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    let png_data = pixmap.encode_png()?;

    Ok(png_data)
}

/// Converts SVG data to an image.
fn convert_svg_to_image(input: &[u8]) -> BoxResult<DynamicImage> {
    let svg_img = convert_svg_to_image_data(input)?;
    image::load_from_memory_with_format(&svg_img, image::ImageFormat::Png).map_err(Into::into)
}

/// Renders the QR code into an SVG image.
pub fn to_svg(code: &QrCode, view_arguments: QrCodeViewArguments) -> String {
    let (dark_color, light_color) = match view_arguments.invert_colors {
        true => (svg::Color("white"), svg::Color("black")),
        false => (svg::Color("black"), svg::Color("white")),
    };

    Renderer::<svg::Color<'_>>::new(&code.to_colors(), code.width(), view_arguments.margin)
        .dark_color(dark_color)
        .light_color(light_color)
        .build()
}

/// Renders the QR code into the unicode string that can be used in terminal
pub fn to_unicode(code: &QrCode, view_arguments: QrCodeViewArguments) -> String {
    let (dark_color, light_color) = match view_arguments.invert_colors {
        true => (unicode::Dense1x2::Dark, unicode::Dense1x2::Light),
        false => (unicode::Dense1x2::Light, unicode::Dense1x2::Dark),
    };

    Renderer::<unicode::Dense1x2>::new(&code.to_colors(), code.width(), view_arguments.margin)
        .dark_color(dark_color)
        .light_color(light_color)
        .build()
}

/// Renders the QR code into an image.
pub fn to_image(code: &QrCode, view_arguments: QrCodeViewArguments) -> DynamicImage {
    let mut image = DynamicImage::ImageRgba8(
        Renderer::<Rgba<u8>>::new(&code.to_colors(), code.width(), view_arguments.margin).build(),
    );

    if view_arguments.invert_colors {
        image.invert()
    }

    image
}

pub fn print_code_to_term(code: &QrCode, view_arguments: QrCodeViewArguments) {
    println!("\n{}", to_unicode(code, view_arguments));
}

/// Extracts qrcode data from image
pub fn extract_contents_from_image(img: DynamicImage) -> Vec<String> {
    let mut prepared_img = PreparedImage::prepare(img.to_luma8());

    prepared_img
        .detect_grids()
        .into_iter()
        .map(|grid| {
            grid.decode()
                .map(|(_, content)| content)
                .unwrap_or_else(|err| {
                    eprintln!("\nERROR reading data from qr code: {}", err);
                    panic!();
                })
        })
        .collect()
}

pub fn read_data_from_image(file: &Path) -> BoxResult<Vec<String>> {
    let img = if is_svg_path(file) {
        let input = fs::read(file)?;
        convert_svg_to_image(&input)?
    } else {
        image::open(file)?
    };

    Ok(extract_contents_from_image(img))
}

pub fn save(file: &Path, code: &QrCode, image_save_args: ImageSaveArguments) -> BoxResult<()> {
    let save_fn: fn(&Path, &QrCode, QrCodeViewArguments) -> BoxResult<()> =
        match image_save_args.output_format {
            OutputFormat::Image => save_image,
            OutputFormat::Svg => save_svg,
            OutputFormat::Unicode => save_unicode,
        };
    save_fn(file, code, image_save_args.view_arguments)
}

fn save_unicode(file: &Path, code: &QrCode, view_arguments: QrCodeViewArguments) -> BoxResult<()> {
    let unicode_image = to_unicode(code, view_arguments);
    fs::write(file, unicode_image).map_err(|err| handle_save_error(file, err))
}

fn save_image(file: &Path, code: &QrCode, view_arguments: QrCodeViewArguments) -> BoxResult<()> {
    let image = to_image(code, view_arguments);

    let image = match file.extension().and_then(OsStr::to_str) {
        Some("jpg") | Some("jpeg") | Some("pbm") | Some("pgm") | Some("ppm") => {
            DynamicImage::ImageRgb8(image.into_rgb8())
        }
        _ => image,
    };

    image.save(file).map_err(|err| handle_save_error(file, err))
}

fn save_svg(file: &Path, code: &QrCode, view_arguments: QrCodeViewArguments) -> BoxResult<()> {
    let svg_image = to_svg(code, view_arguments);
    fs::write(file, svg_image).map_err(|err| handle_save_error(file, err))
}

fn handle_save_error<E: Error + 'static>(file: &Path, error: E) -> Box<dyn Error> {
    eprintln!("\nERROR saving the file: {}", error);
    fs::remove_file(file).unwrap();
    Box::new(error)
}

pub struct ImageSaveArguments<'a> {
    pub output_format: &'a OutputFormat,
    pub view_arguments: QrCodeViewArguments,
}

pub struct QrCodeViewArguments {
    pub margin: u32,
    pub invert_colors: bool,
}

impl<'a> From<&'a Arguments> for ImageSaveArguments<'a> {
    fn from(args: &'a Arguments) -> Self {
        Self {
            output_format: &args.output_format,
            view_arguments: args.into(),
        }
    }
}

impl From<&Arguments> for QrCodeViewArguments {
    fn from(args: &Arguments) -> Self {
        Self {
            margin: args.margin,
            invert_colors: args.invert_colors,
        }
    }
}
