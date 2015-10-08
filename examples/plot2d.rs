extern crate image;
extern crate bspline;

use std::ops::{Mul, Add};
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

/// Evaluate the B-spline and plot it to the image buffer passed
fn plot_2d(spline: &bspline::BSpline<Point>, plot: &mut [u8], plot_dim: (usize, usize), scale: (f32, f32),
           offset: (f32, f32), t_range: (f32, f32)) {
    let step_size = 0.001;
    let steps = ((t_range.1 - t_range.0) / step_size) as usize;
    for s in 0..steps {
        let t = step_size * s as f32 + t_range.0;
        let pt = spline.point(t);
        let ix = ((pt.x + offset.0) * scale.0) as isize;
        let iy = ((pt.y + offset.1) * scale.1) as isize;
        for y in iy - 1..iy + 1 {
            for x in ix - 1..ix + 1 {
                if y >= 0 && y < plot_dim.1 as isize && x >= 0 && x < plot_dim.0 as isize {
                    let px = (plot_dim.1 - 1 - y as usize) * plot_dim.0 * 3 + x as usize * 3;
                    for i in 0..3 {
                        plot[px + i] = 0;
                    }
                }
            }
        }
    }
}

/// Plot a simple 2D quadratic B-spline
fn plot_quadratic() {
    let points = vec![Point::new(-1.0, 0.0), Point::new(-1.0, 0.0), Point::new(0.0, 1.0),
                      Point::new(1.0, 0.0), Point::new(1.0, 0.0)];
    let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    let t_start = knots[0];
    let t_end = knots[knots.len() - 1];

    let plot_dim = (720, 540);
    let scale = (plot_dim.0 as f32 / 4.0, plot_dim.1 as f32 / 4.0);
    let offset = (2.0, 2.0);

    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_dim.0 * plot_dim.1 * 3).collect();

    println!("Plotting Quadratic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);
    println!("\tStarting at {}, ending at {}", t_start, t_end);
    let spline = bspline::BSpline::new(2, points.clone(), knots);

    plot_2d(&spline, &mut plot[..], plot_dim, scale, offset, (t_start, t_end));

    // Draw the control points
    for pt in points.iter() {
        let ix = ((pt.x + offset.0) * scale.0) as isize;
        let iy = ((pt.y + offset.1) * scale.1) as isize;
        // Plot a 4x4 red marker for each control point
        for y in iy - 3..iy + 3 {
            for x in ix - 3..ix + 3 {
                if y >= 0 && y < plot_dim.1 as isize && x >= 0 && x < plot_dim.0 as isize {
                    let px = (plot_dim.1 - 1 - y as usize) * plot_dim.0 * 3 + x as usize * 3;
                    plot[px] = 255;
                    plot[px + 1] = 0;
                    plot[px + 2] = 0;
                }
            }
        }
    }
    match image::save_buffer("quadratic_2d.png", &plot[..], plot_dim.0 as u32, plot_dim.1 as u32, image::RGB(8)) {
        Ok(_) => println!("2D Quadratic B-spline saved to quadratic_2d.png"),
        Err(e) => println!("Error saving quadratic_2d.png,  {}", e),
    }
}

fn main() {
    let divider: String = iter::repeat('-').take(80).collect();
    plot_quadratic();
}

