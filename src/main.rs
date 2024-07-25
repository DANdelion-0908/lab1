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
    let color2 = Color::from_hex(0xFF0000); // Red
    let color3 = Color::from_hex(0x0000FF); // Blue
    let color4 = Color::from_hex(0x00FF00); // Green
    let color5 = Color::from_hex(0x000000); // Black
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

    let poly2: Vec<(usize, usize)> = vec![
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302)
    ];

    let poly3: Vec<(usize, usize)> = vec![
        (377, 249),
        (411, 197),
        (436, 249)
    ];

    let poly4: Vec<(usize, usize)> = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180)
    ];

    let poly5: Vec<(usize, usize)> = vec![
        (682, 175),
        (708, 120),
        (735, 148),
        (739, 170)
    ];

    framebuffer.polygon(&poly1, color1, border);
    framebuffer.polygon(&poly2, color2, border);
    framebuffer.polygon(&poly3, color3, border);
    framebuffer.polygon(&poly4, color4, border);
    framebuffer.polygon(&poly5, color5, border);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
