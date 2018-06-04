use ::image::png::PNGEncoder;
use ::image::ColorType;
use ::rand::prelude::*;

use std::io::Write;

pub struct Size
{
    width: u32,
    height: u32
}

pub fn get_size(args: &Vec<String>) -> Result<Size, Box<::std::error::Error>>
{
    let mut iter = args.iter();

    //Coded by vitalyd at users.rust-lang.org/ .... Thank you so much :)
    match (iter.next(), iter.next())
    {
        (Some(x), Some(y)) => Ok(Size
            {
            width: x
                .parse()
                .map_err(|e| format!("Failed to parse x due to {}: {}", e, x))?,
            height: y
                .parse()
                .map_err(|e| format!("Failed to parse y due to {}: {}", e, y))?,
        }),
        _ => Err("Invalid arguments".into()),
    }
    //...
}

pub fn prepare_rand_map_and_save(size: Size) -> bool
{
    let mut imbf: ::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>> = ::image::ImageBuffer::new(size.width, size.height);
    let threads = 8;
    let amount = (size.height  * size.width) * 4;

    let mut pixels: Vec<u8> = imbf.into_vec();
    println!("Rendering random map...");

    ::crossbeam::scope(|spawner|
        {
            for mut line in pixels.chunks_mut((amount  / threads) as usize)
            {
                spawner.spawn(move ||
                {
                    for i in 0..line.len()
                    {
                        line[i] = random::<u8>();
                    }
                });
            }
         });
    imbf = ::image::ImageBuffer::from_vec(size.width, size.height, pixels).unwrap();


    let output = ::File::create("black_hawk.png");
    match output
    {
        Ok(x) =>
        {
            let encoder = PNGEncoder::new(x);
            println!("Encoding file");
            encoder.encode(&imbf, size.width, size.height, ColorType::RGBA(8)).unwrap();
            true
        },
        Err(err) =>
        {
            writeln!(::std::io::stderr(), "{}", err).unwrap();
            false
        }
    }
}

pub fn image_to_file(buff: ::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>) -> bool
{
    let path = ::Path::new("black_hawk.png");
    match ::image::ImageRgba8(buff).save(path)
    {
        Ok(_) => true,
        Err(_) => false
    }
}
