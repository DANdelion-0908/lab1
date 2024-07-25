use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    // Constructor con valores RGB normalizados (0.0 a 1.0)
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: Self::clamp(r),
            g: Self::clamp(g),
            b: Self::clamp(b),
        }
    }

    // Constructor con valores RGB en el rango [0, 255]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        )
    }

    // Constructor con valor hexadecimal
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color::from_rgb(r, g, b)
    }

    // Convertir el color a hexadecimal
    pub fn to_hex(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    // Convertir el color a un array de f32
    pub fn to_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    // Convertir un array de f32 a Color
    pub fn from_array(arr: [f32; 3]) -> Self {
        Color::new(arr[0], arr[1], arr[2])
    }

    // Asegurar que los valores estén en el rango [0.0, 1.0]
    fn clamp(value: f32) -> f32 {
        value.max(0.0).min(1.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Color::new(
            self.r * scalar,
            self.g * scalar,
            self.b * scalar,
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color(R: {:.2}, G: {:.2}, B: {:.2})",
            self.r, self.g, self.b
        )
    }
}
