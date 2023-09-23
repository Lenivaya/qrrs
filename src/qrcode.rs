use crate::cli::{Arguments, OutputFormat};
use crate::errors::BoxResult;

use image::{DynamicImage, Rgba};
use qrencode::render::{svg, unicode, Renderer};
use qrencode::QrCode;
use rqrr::PreparedImage;

use std::error::Error;
use std::fs;
use std::path::Path;

pub fn make_code(data: &str) -> BoxResult<QrCode> {
    Ok(QrCode::new(data.as_bytes())?)
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
pub fn to_terminal(code: &QrCode, view_arguments: QrCodeViewArguments) -> String {
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
    println!("\n{}", to_terminal(code, view_arguments));
}

pub fn read_data_image(file: &Path) -> BoxResult<Vec<String>> {
    let img = image::open(file)?.to_luma8();
    let mut prepared_img = PreparedImage::prepare(img);

    let contents = prepared_img
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
        .collect();

    Ok(contents)
}

pub fn save(file: &Path, code: &QrCode, image_save_args: ImageSaveArguments) -> BoxResult<()> {
    match image_save_args.output_format {
        OutputFormat::Image => save_image(file, code, image_save_args),
        OutputFormat::Svg => save_svg(file, code, image_save_args),
    }
}

fn save_image(file: &Path, code: &QrCode, args: ImageSaveArguments) -> BoxResult<()> {
    let image = to_image(code, args.view_arguments);
    image.save(file).map_err(|err| handle_save_error(file, err))
}

fn save_svg(file: &Path, code: &QrCode, args: ImageSaveArguments) -> BoxResult<()> {
    let svg_image = to_svg(code, args.view_arguments);
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
