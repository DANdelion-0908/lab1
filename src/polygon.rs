use crate::colors::Color;
use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, points: &[(usize, usize)], fill_color: Color, border_color: Color);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[(usize, usize)], fill_color: Color, border_color: Color) {
        if points.len() < 3 {
            return;
        }

        let (min_y, max_y) = points.iter().fold((usize::MAX, 0), |(min_y, max_y), &(_, y)| {
            (min_y.min(y), max_y.max(y))
        });

        for y in min_y..=max_y {
            let mut intersections = vec![];

            for i in 0..points.len() {
                let (x0, y0) = points[i];
                let (x1, y1) = points[(i + 1) % points.len()];

                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    let x = x0 as f32 + (y as f32 - y0 as f32) * (x1 as f32 - x0 as f32) / (y1 as f32 - y0 as f32);
                    intersections.push(x as usize);
                }
            }

            intersections.sort();

            for pair in intersections.chunks(2) {
                if pair.len() == 2 {
                    let (start, end) = (pair[0], pair[1]);
                    for x in start..=end {
                        self.buffer[y * self.width + x] = fill_color.to_hex();
                    }
                }
            }
        }

        self.set_current_color(border_color);
        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];
            self.line(x0, y0, x1, y1);
        }
    }
}
