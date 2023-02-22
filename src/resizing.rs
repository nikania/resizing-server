use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::num::NonZeroU32;
use std::path::Path;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use fast_image_resize as fr;

use crate::common::AppError;
use crate::routes::ResizeData;

pub fn run(data: ResizeData) -> Result<String, AppError> {
    let target = data.dimensions;
    // let current = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    // println!("current path: {current}");
    // Read source image from file
    let filepath = "./data/".to_owned();
    let mut filename = filepath.clone();
    filename.push_str(&data.filename);
    filename.push_str(".");
    filename.push_str(&data.file_extension);
    let img = ImageReader::open(filename)
        .map_err(|_| AppError::NotFoundError {
            error: "File not found".into(),
        })?
        .decode()
        .unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();

    // Multiple RGB channels of source image by alpha channel
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(target.0).unwrap();
    let dst_height = NonZeroU32::new(target.1).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        )
        .unwrap();

    let mut resized_filename = filepath.clone();
    resized_filename.push_str(&data.filename);
    resized_filename.push_str("_res.");
    resized_filename.push_str(&data.file_extension);
    // save result to file
    image::save_buffer(
        resized_filename,
        dst_image.buffer(),
        target.0,
        target.1,
        image::ColorType::Rgba8,
    )
    .unwrap();

    Ok("ok".into())
}
