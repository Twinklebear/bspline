extern crate image;
extern crate bspline;

use std::ops::{Mul, Add, Index, IndexMut};
use std::iter;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

/// Clamp `x` to be between `min` and `max`
pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min { min } else if x > max { max } else { x }
}

#[derive(Debug, Copy, Clone)]
pub struct Colorf {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colorf {
    /// Create an RGB color
    pub fn new(r: f32, g: f32, b: f32) -> Colorf {
        Colorf { r: r, g: g, b: b }
    }
    pub fn broadcast(x: f32) -> Colorf {
        Colorf { r: x, g: x, b: x }
    }
    /// Clamp the color values between [0, 1]
    pub fn clamp(&self) -> Colorf {
        Colorf { r: clamp(self.r, 0.0, 1.0),
                 g: clamp(self.g, 0.0, 1.0),
                 b: clamp(self.b, 0.0, 1.0) }
    }
    /// Convert the linear RGB color to sRGB
    pub fn to_srgb(&self) -> Colorf {
        let a = 0.055f32;
        let b = 1f32 / 2.4;
        let mut srgb = Colorf::broadcast(0.0);
        for i in 0..3 {
            if self[i] <= 0.0031308 {
                srgb[i] = 12.92 * self[i];
            } else {
                srgb[i] = (1.0 + a) * f32::powf(self[i], b) - a;
            }
        }
        srgb
    }
}
impl Add for Colorf {
    type Output = Colorf;
    /// Add two colors together
    fn add(self, rhs: Colorf) -> Colorf {
        Colorf { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}
impl Mul<f32> for Colorf {
    type Output = Colorf;
    /// Scale the color by the float
    fn mul(self, rhs: f32) -> Colorf {
        Colorf { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs }
    }
}
impl Index<usize> for Colorf {
    type Output = f32;
    /// Access the channels by index
    /// 
    /// - 0 = r
    /// - 1 = g
    /// - 2 = b
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Invalid index into color"),
        }
    }
}
impl IndexMut<usize> for Colorf {
    /// Access the channels by index
    /// 
    /// - 0 = r
    /// - 1 = g
    /// - 2 = b
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Invalid index into color"),
        }
    }
}

/// Evaluate the B-spline and plot it to the image buffer passed. The colors and points splines
/// should have the same t range.
fn plot_2d(spline: &bspline::BSpline<Point>, colors: &bspline::BSpline<Colorf>, plot: &mut [u8],
           plot_dim: (usize, usize), scale: (f32, f32), offset: (f32, f32), t_range: (f32, f32)) {
    let step_size = 0.001;
    let steps = ((t_range.1 - t_range.0) / step_size) as usize;
    for s in 0..steps {
        let t = step_size * s as f32 + t_range.0;
        let pt = spline.point(t);
        let color = colors.point(t).to_srgb();
        let ix = ((pt.x + offset.0) * scale.0) as isize;
        let iy = ((pt.y + offset.1) * scale.1) as isize;
        for y in iy - 1..iy + 1 {
            for x in ix - 1..ix + 1 {
                if y >= 0 && y < plot_dim.1 as isize && x >= 0 && x < plot_dim.0 as isize {
                    let px = (plot_dim.1 - 1 - y as usize) * plot_dim.0 * 3 + x as usize * 3;
                    for i in 0..3 {
                        plot[px + i] = (color[i] * 255.0) as u8;
                    }
                }
            }
        }
    }
    // Draw the control points
    for pt in spline.control_points() {
        let ix = ((pt.x + offset.0) * scale.0) as isize;
        let iy = ((pt.y + offset.1) * scale.1) as isize;
        // Plot a 4x4 red marker for each control point
        for y in iy - 2..iy + 2 {
            for x in ix - 2..ix + 2 {
                if y >= 0 && y < plot_dim.1 as isize && x >= 0 && x < plot_dim.0 as isize {
                    let px = (plot_dim.1 - 1 - y as usize) * plot_dim.0 * 3 + x as usize * 3;
                    plot[px] = 0;
                    plot[px + 1] = 0;
                    plot[px + 2] = 0;
                }
            }
        }
    }
}

/// Plot the text 'bspline' to create the logo for the library
fn main() {
    let points = vec![// Draw the b
                      Point::new(-4.0, 4.0), Point::new(-4.0, -1.0), Point::new(-4.0, -1.0),
                      Point::new(-2.0, 0.0), Point::new(-4.0, 1.35), Point::new(-4.0, 1.35),
                      // Draw the s
                      Point::new(-1.0, 1.5), Point::new(-1.0, 1.5), Point::new(-2.8, 1.0),
                      Point::new(-0.5, 0.0), Point::new(-2.5, -0.5), Point::new(-2.5, -0.5),
                      // Draw the p
                      Point::new(0.0, -1.0), Point::new(0.0, 1.8), Point::new(0.0, 1.8),
                      Point::new(0.0, 1.8), Point::new(-0.2, -1.8), Point::new(-0.2, -1.8),
                      Point::new(-0.2, -1.8), Point::new(0.25, 2.5), Point::new(1.5, 1.2),
                      Point::new(0.0, -0.5), Point::new(0.0, -0.5), Point::new(1.0, -0.3),
                      Point::new(1.0, -0.3), Point::new(2.5, 1.5),
                      // Draw the l
                      Point::new(2.0, 3.0), Point::new(2.0, 3.0), Point::new(1.5, 1.5),
                      Point::new(2.5, -0.5), Point::new(2.5, -0.5),
                      // Draw the i
                      Point::new(2.8, 1.3), Point::new(2.8, 1.3), Point::new(2.8, 1.3),
                      // Draw the n
                      Point::new(3.1, -0.5), Point::new(3.1, -0.5), Point::new(3.4, 1.2),
                      Point::new(3.4, 1.2), Point::new(3.4, 1.2), Point::new(3.4, -0.5),
                      Point::new(3.4, -0.5), Point::new(3.4, -0.5), Point::new(4.0, 2.3),
                      Point::new(4.2, -0.5), Point::new(4.2, -0.5),
                      // Draw the e
                      Point::new(5.5, 0.8), Point::new(5.5, 0.8), Point::new(4.5, 1.3), Point::new(4.5, -0.3),
                      Point::new(6.5, -0.3)];
    let knots = vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 6.0, 7.0, 7.0,
                     8.0, 9.0, 9.0, 9.0, 10.0, 10.0, 10.0, 11.0, 12.0, 13.0, 13.0,
                     14.0, 14.0, 15.0, 16.0, 16.0, 17.0, 18.0, 18.0, 19.0, 19.0, 19.0,
                     20.0, 20.0, 21.0, 21.0, 21.0, 22.0, 22.0, 22.0, 23.0, 24.0, 24.0,
                     25.0, 25.0, 26.0, 27.0, 28.0, 28.0, 28.0, 28.0];

    let colors = vec![Colorf::new(1.0, 0.0, 0.0), Colorf::new(1.0, 0.0, 0.0),
                      Colorf::new(0.0, 0.0, 1.0), Colorf::new(0.0, 1.0, 0.0),
                      Colorf::new(0.0, 1.0, 0.0)];
    let color_knots = vec![0.0, 0.0, 0.0, 14.0, 28.0, 28.0, 28.0];

    let t_start = knots[0];
    let t_end = knots[knots.len() - 1];

    let plot_dim = (720, 540);
    let scale = (plot_dim.0 as f32 / 14.0, plot_dim.1 as f32 / 10.0);
    let offset = (6.0, 5.0);

    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_dim.0 * plot_dim.1 * 3).collect();

    println!("Plotting Cubic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);
    println!("\tStarting at {}, ending at {}", t_start, t_end);
    let spline = bspline::BSpline::new(3, points, knots);
    let color_spline = bspline::BSpline::new(2, colors, color_knots);

    plot_2d(&spline, &color_spline, &mut plot[..], plot_dim, scale, offset, (t_start, t_end));
    match image::save_buffer("logo.png", &plot[..], plot_dim.0 as u32, plot_dim.1 as u32, image::RGB(8)) {
        Ok(_) => println!("B-spline logo saved to logo.png"),
        Err(e) => println!("Error saving logo.png,  {}", e),
    }
}

