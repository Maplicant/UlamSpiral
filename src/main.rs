extern crate image;
extern crate primal;
extern crate clap;

use std::fs::File;
use clap::{Arg, App};


const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn valid_size(o: String) -> Result<(), String> {
    if let Err(..) = o.parse::<u32>() {
        return Err(format!("'{}' must be a positive integer.", o));
    }
    Ok(())
}

fn main() {
    let matches = App::new("UlamSpiral")
                          .version("0.1")
                          .author("Maplicant <maplicant@gmail.com>")
                          .about("Generates Ulam spirals")
                          .arg(Arg::with_name("size")
                               .short("s")
                               .long("s")
                               .value_name("SIZE")
                               .help("The size of the Ulam Spiral")
                               .validator(valid_size)
                               .takes_value(true))
                          .get_matches();
    let mut size: i32 = matches.value_of("size").unwrap_or("201").parse::<i32>().expect("Couldn't parse input. Try running UlamSpiral --help");
    if size % 2 == 0 {
        size = size + 1;
        println!("Size isn't odd, new dimensions are {0}x{0}", size);
    }
    let offset = size / 2;



    // Initialize grid
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut x = 0;
    let mut y = 0;
    let mut xv = 1;
    let mut yv = 0;
    let mut direction: usize = 3;
    let mut value: usize = 1;
    let sieve = primal::Sieve::new((size*size) as usize);
    let mut imgbuf = image::ImageBuffer::new(size as u32, size as u32);

    // Fill grid
    loop {
        x += xv;
        y += yv;

        if x > max_x {
            max_x = x;
            direction = (direction + 1) % 4;
            xv = DIRECTIONS[direction].0;
            yv = DIRECTIONS[direction].1;
        }
        if y > max_y {
            max_y = y;
            direction = (direction + 1) % 4;
            xv = DIRECTIONS[direction].0;
            yv = DIRECTIONS[direction].1;
        }
        if x < min_x {
            min_x = x;
            direction = (direction + 1) % 4;
            xv = DIRECTIONS[direction].0;
            yv = DIRECTIONS[direction].1;
        }
        if y < min_y {
            min_y = y;
            direction = (direction + 1) % 4;
            xv = DIRECTIONS[direction].0;
            yv = DIRECTIONS[direction].1;
        }

        if x > offset || x < (-offset) || y > offset || y < (-offset) {
            break;
        } else {
            // grid.set((x + offset) as usize, (y + offset) as usize, is_prime(value));
            if !sieve.is_prime(value) {
                imgbuf.put_pixel((x + offset) as u32, (y + offset) as u32, image::Luma([255u8]))
            }
        }
        value += 1;
    }

    let ref mut fout = File::create("output.png").expect("Failed to create output.png.");
    image::ImageLuma8(imgbuf).save(fout, image::PNG).expect("Failed to save image.");
}
