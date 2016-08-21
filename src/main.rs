extern crate image;
use std::env;
use std::fs::File;

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn main() {
    // Get user input
    let mut size: i32 = match env::args().nth(1) {
        None => panic!("Didn't get size argument."),
        Some(input) => match input.parse::<i32>() {
            Err(_) => panic!("Unable to parse input."),
            Ok(parsed) => parsed
        }
    };
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
    let mut value: u32 = 1;
    // let mut grid: Grid = Grid::new(size as usize);
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
            if is_prime(value) {
                imgbuf.put_pixel((x + offset) as u32, (y + offset) as u32, image::Luma([0u8]))
            }
            else {
                imgbuf.put_pixel((x + offset) as u32, (y + offset) as u32, image::Luma([255u8]))
            }
        }
        value += 1;
    }

    // grid.print();
    // grid.save_to_image();
    let ref mut fout = File::create("output.png").expect("Failed to create output.png.");
    image::ImageLuma8(imgbuf).save(fout, image::PNG).expect("Failed to save image.");
}

// struct Grid {
//     values: Box<Vec<bool>>,
//     size: usize
// }
//
// impl Grid {
//     fn new(size: usize) -> Grid {
//         Grid {
//             values: Box::new(vec![false; size * size + 1]),
//             size: size
//         }
//     }
//     fn get(&self, x: usize, y: usize) -> bool {
//         self.values[x * (self.size - 1) + y]
//     }
//     fn set(&mut self, x: usize, y: usize, value: bool) {
//         self.values[x * (self.size - 1) + y] = value;
//     }
//     fn print(&self) {
//         let mut counter = 0;
//         for value in self.values.iter() {
//             print!("{}", match *value {
//                 true => "#",
//                 false => " "
//             });
//             if counter > self.size {
//                 println!("");
//                 counter = 0;
//             }
//             counter += 1;
//         }
//     }
//     fn save_to_image(&self) {
//         let mut imgbuf = image::ImageBuffer::new(self.size as u32, self.size as u32);
//         for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//             if self.get(x as usize, y as usize) {
//                 *pixel = image::Luma([0u8]);
//             } else {
//                 *pixel = image::Luma([255u8]);
//             }
//         }
//         let ref mut fout = File::create("output.png").unwrap();
//         let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
//     }
// }

fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5u32;
    let mut w = 2u32;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    true
}
