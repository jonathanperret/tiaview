extern crate minifb;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

const WIDTH: usize = 162;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let opts =
        WindowOptions {
            borderless: false,
            title: true,
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
        };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        opts
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window.set_position(200, 200);

    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let flen : usize = file.metadata().unwrap().len() as usize;
    println!("Length is {}", flen);
    let mut fbuf = Vec::new();
    let readcount = file.read_to_end(&mut fbuf).unwrap();
    println!("Read {} bytes", readcount);

    let mut fpos : usize = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) && fpos < flen {
        let mut x : i32 = 0;
        let mut y : i32 = -64;
        let mut syncs = 0;
        let mut gotvsync = false;
        let mut clock = false;
        loop {
            let b = fbuf[fpos];
            fpos = fpos + 1;
            if fpos >= flen {
                break;
            }
            if b & 0x01 == 0 {
                clock = false;
                continue;
            } else {
                if clock {
                    continue;
                }
                clock = true;
            }
            if b & 0x10 == 0 {
                syncs = syncs + 1;
                continue;
            } else {
                if syncs > 0 {
                    x = -33;
                    y = y + 1;
                    if syncs > 100 {
                        gotvsync = true;
                    } else {
                        if gotvsync {
                            break;
                        }
                    }
                    syncs = 0;
                }
            }
            x = x + 1;
            if x >= 0 && (x as usize) < WIDTH && y >= 0 && (y as usize) < HEIGHT {
                buffer[(y as usize) * WIDTH + (x as usize)] =
                    if b & 0x04 != 0 { 0xff0000 } else { 0 } |
                    if b & 0x08 != 0 { 0xff00 } else { 0 } |
                    if b & 0x02 != 0 { 0xff } else { 0 }
                ;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
