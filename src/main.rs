mod framebuffer;
mod line;
mod polygon;
mod bmp;
mod colors;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;
use crate::colors::Color;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::from_hex(0x000000));
    framebuffer.clear();

    let color1 = Color::from_hex(0x00FFFF); // Yellow
    let border = Color::from_hex(0xFFFFFF); // White

    let poly1: Vec<(usize, usize)> = vec![
        (165, 380), 
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330),
        (230, 360), 
        (250, 380), 
        (220, 385), 
        (205, 410), 
        (193, 383)
    ];

    framebuffer.polygon(&poly1, color1, border);


    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
