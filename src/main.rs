extern crate image;
extern crate rand;
extern crate crossbeam;

use std::io::Write;
use std::fs::File;
use std::path::Path;

mod render;
pub use render::*;

fn main()
{
    let args: Vec<String> = std::env::args().skip(1).collect();
    let size = match render::get_size(&args)
    {
        Ok(sz) => sz,
        Err(er) =>
        {
            writeln!(std::io::stderr(), "{}", er).unwrap();
            std::process::exit(1);
        }
    };
    prepare_rand_map_and_save(size);
}
