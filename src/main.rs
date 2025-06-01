use image::GrayImage;
use image::ImageReader;
use image::Pixel;
use std::env;
use std::error::Error;
use std::result::Result;

const HEIGHT: u32 = 128;
const WIDTH: u32 = 128;

fn ascii_conversion() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ascii-conv <path-to-image> <con/full>");
        return Ok(());
    }

    let img_path = &args[1];
    let mut conv = true;

    if args.len() > 2 {
        conv = match args[2].as_str() {
            "con" => true,
            "full" => false,
            _ => {
                println!("Cannot parse second argument");
                return Ok(());
            }
        }
    }

    let img = ImageReader::open(img_path)?.decode()?;
    let mut luma = img.into_luma8();
    let width = luma.width();
    let height = luma.height();

    if conv {
        let step_x = width / WIDTH + 1;
        let step_y = height / HEIGHT + 1;
        let radius = std::cmp::min(step_x, step_y);

        luma = convolve(luma, radius, step_x, step_y)?;
    }

    let mut ascii_string = String::from("");
    for (x, _, pix) in luma.enumerate_pixels() {
        if x == 0 {
            ascii_string = String::from("");
        }
        // println!("{:?}", pix.channels()[0]);

        match pix.channels()[0] {
            u8::MIN..=42 => ascii_string.push('.'),
            43..=85 => ascii_string.push(','),
            86..=128 => ascii_string.push('<'),
            129..=170 => ascii_string.push('/'),
            171..=212 => ascii_string.push('#'),
            213..=255 => ascii_string.push('@'),
        }

        if x == luma.width() - 1 {
            println!("{}", ascii_string);
        }

        //       println!("{}, {}, {:?}", x, y, pix.channels()[0]);
    }

    Ok(())
}

fn convolve(
    img: GrayImage,
    radius: u32,
    step_x: u32,
    step_y: u32,
) -> Result<GrayImage, Box<dyn Error>> {
    let out_img = GrayImage::from_fn(
        (img.width() - 2 * radius) / step_x,
        (img.height() - 2 * radius) / step_y,
        |x, y| {
            let min_x = x * step_x;
            let max_x = x * step_x + 2 * radius;
            let min_y = y * step_y;
            let max_y = y * step_y + 2 * radius;
            let mut tot: u32 = 0;
            let mut cnt: u32 = 0;
            for i in min_x..=max_x {
                for j in min_y..=max_y {
                    tot += img.get_pixel(i, j).channels()[0] as u32;
                    cnt += 1;
                }
            }

            image::Luma([(tot / cnt) as u8])
        },
    );

    Ok(out_img)
}

fn main() {
    match ascii_conversion() {
        Ok(_) => (),
        Err(e) => println!("Encountered an error {e}"),
    }
}
