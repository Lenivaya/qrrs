use crate::cli::OutputFormat;
use crate::errors::BoxResult;

use image::Luma;
use qrencode::render::{svg, unicode};
use qrencode::QrCode;
use rqrr::PreparedImage;

use std::error::Error;
use std::fs;
use std::path::Path;

pub fn make_code(data: &str) -> BoxResult<QrCode> {
    Ok(QrCode::new(data.as_bytes())?)
}

/// Renders the QR code into an SVG image.
pub fn to_svg(code: &QrCode) -> String {
    code.render::<svg::Color<'_>>().build()
}

/// Renders the QR code into the unicode string that can be used in terminal
pub fn to_terminal(code: &QrCode) -> String {
    code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}

/// Renders the QR code into an image.
pub fn to_image(code: &QrCode) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    code.render::<Luma<u8>>().build()
}

pub fn print_code_to_term(code: &QrCode) {
    println!("\n{}", to_terminal(code));
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

pub fn save(file: &Path, code: &QrCode, output_format: &OutputFormat) -> BoxResult<()> {
    match output_format {
        OutputFormat::Image => save_image(file, code),
        OutputFormat::Svg => save_svg(file, code),
    }
}

fn save_image(file: &Path, code: &QrCode) -> BoxResult<()> {
    let image = to_image(code);
    image.save(file).map_err(|err| handle_save_error(file, err))
}

fn save_svg(file: &Path, code: &QrCode) -> BoxResult<()> {
    let svg_image = to_svg(code);
    fs::write(file, svg_image).map_err(|err| handle_save_error(file, err))
}

fn handle_save_error<E: Error + 'static>(file: &Path, error: E) -> Box<dyn Error> {
    eprintln!("\nERROR saving the file: {}", error);
    fs::remove_file(file).unwrap();
    Box::new(error)
}
