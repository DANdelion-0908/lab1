use std::ops::{Add, Mul};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Self::from_rgb(r, g, b)
    }

    pub fn to_hex(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn from_array(arr: [f32; 3]) -> Self {
        Color { r: arr[0], g: arr[1], b: arr[2] }
    }

    fn clamp(value: f32) -> f32 {
        value.max(0.0).min(1.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color {
            r: Self::clamp(self.r + other.r),
            g: Self::clamp(self.g + other.g),
            b: Self::clamp(self.b + other.b),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Color {
            r: Self::clamp(self.r * scalar),
            g: Self::clamp(self.g * scalar),
            b: Self::clamp(self.b * scalar),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(R: {:.2}, G: {:.2}, B: {:.2})", self.r, self.g, self.b)
    }
}