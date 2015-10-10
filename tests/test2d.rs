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
           offset: (f32, f32)) {
    let step_size = 0.001;
    let t_range = spline.knot_domain();
    let steps = ((t_range.1 - t_range.0) / step_size) as usize;
    for s in 0..steps + 1 {
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
    // Draw the control points
    for pt in spline.control_points() {
        let ix = ((pt.x + offset.0) * scale.0) as isize;
        let iy = ((pt.y + offset.1) * scale.1) as isize;
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
}

#[test]
fn plot_quadratic2d() {
    let points = vec![Point::new(-1.5, 0.0), Point::new(0.0, 1.5), Point::new(1.5, 0.0)];
    let knots = vec![0.0, 0.0, 0.0, 3.0, 3.0, 3.0];
    let degree = 2;
    let spline = bspline::BSpline::new(degree, points, knots);

    let plot_dim = (200, 200);
    let scale = (plot_dim.0 as f32 / 4.0, plot_dim.1 as f32 / 4.0);
    let offset = (2.0, 2.0);
    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_dim.0 * plot_dim.1 * 3).collect();
    plot_2d(&spline, &mut plot[..], plot_dim, scale, offset);

    let expect_plot = match image::open("tests/quadratic_2d_expect.png") {
        Ok(image::ImageRgb8(img)) => img.into_vec(),
        Ok(_) => panic!("Invalid image found for expected quadratic 2d plot"),
        Err(e) => panic!(e),
    };
    assert!(plot == expect_plot);
}
#[test]
fn plot_cubic2d() {
    let points = vec![Point::new(-1.5, -1.5), Point::new(-0.5, 1.5),
                      Point::new(0.5, -1.5), Point::new(1.5, 1.5)];
    let knots = vec![0.0, 1.0, 2.0, 2.0, 5.0, 5.0, 6.0, 7.0];
    let degree = 3;
    let spline = bspline::BSpline::new(degree, points, knots);

    let plot_dim = (200, 200);
    let scale = (plot_dim.0 as f32 / 4.0, plot_dim.1 as f32 / 4.0);
    let offset = (2.0, 2.0);
    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_dim.0 * plot_dim.1 * 3).collect();

    plot_2d(&spline, &mut plot[..], plot_dim, scale, offset);
    let expect_plot = match image::open("tests/cubic_2d_expect.png") {
        Ok(image::ImageRgb8(img)) => img.into_vec(),
        Ok(_) => panic!("Invalid image found for expected cubic 2d plot"),
        Err(e) => panic!(e),
    };
    assert!(plot == expect_plot);
}
#[test]
fn plot_quartic2d() {
    let points = vec![Point::new(-1.8, -1.4), Point::new(-1.2, 0.5), Point::new(-0.2, -0.8),
                      Point::new(-0.6, 0.7), Point::new(0.0, 1.6), Point::new(1.0, 0.0),
                      Point::new(0.6, -0.3), Point::new(0.0, -1.0)];
    let knots = vec![0.0, 0.0, 0.0, 0.0, 0.2, 1.0, 2.0, 3.0, 5.0, 5.0, 5.0, 5.0, 5.0];
    let degree = 4;
    let spline = bspline::BSpline::new(degree, points, knots);

    let plot_dim = (200, 200);
    let scale = (plot_dim.0 as f32 / 4.0, plot_dim.1 as f32 / 4.0);
    let offset = (2.0, 2.0);
    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_dim.0 * plot_dim.1 * 3).collect();

    plot_2d(&spline, &mut plot[..], plot_dim, scale, offset);
    let expect_plot = match image::open("tests/quartic_2d_expect.png") {
        Ok(image::ImageRgb8(img)) => img.into_vec(),
        Ok(_) => panic!("Invalid image found for expected quartic 2d plot"),
        Err(e) => panic!(e),
    };
    assert!(plot == expect_plot);
}

