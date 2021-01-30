extern crate image;
extern crate bspline;

use std::iter;

/// Evaluate the B-spline and plot it to the image buffer passed
fn plot_1d(spline: &bspline::BSpline<f32, f32>, plot: &mut [u8], plot_dim: (usize, usize), scale: (f32, f32),
           offset: (f32, f32)) {
    let step_size = 0.001;
    let t_range = spline.knot_domain();
    let steps = ((t_range.1 - t_range.0) / step_size) as usize;
    for s in 0..steps + 1 {
        let t = step_size * s as f32 + t_range.0;
        let y = spline.point(t);
        let ix = ((t + offset.0) * scale.0) as isize;
        let iy = ((y + offset.1) * scale.1) as isize;
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

/// Plot a simple 1D quadratic B-spline
fn plot_quadratic() {
    let points = vec![0.0, 0.0, 1.0, 0.0, 0.0];
    let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    let degree = 2;

    let plot_w = 720;
    let plot_h = 540;
    let x_scale = plot_w as f32 / knots[knots.len() - 1];
    let y_scale = plot_h as f32 / 2.0;
    let y_offset = 1.0;

    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_w * plot_h * 3).collect();

    println!("Plotting Quadratic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);

    let spline = bspline::BSpline::new(degree, points, knots);

    println!("\tt range = {:?}", spline.knot_domain());

    plot_1d(&spline, &mut plot[..], (plot_w, plot_h), (x_scale, y_scale), (0.0, y_offset));
    match image::save_buffer("quadratic_1d.png", &plot[..], plot_w as u32, plot_h as u32, image::RGB(8)) {
        Ok(_) => println!("1D Quadratic B-spline saved to quadratic_1d.png"),
        Err(e) => println!("Error saving quadratic_1d.png,  {}", e),
    }
}
/// Plot a simple 1D cubic B-spline
fn plot_cubic() {
    let points = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
    let knots = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
    let degree = 3;

    let plot_w = 720;
    let plot_h = 540;
    let x_scale = plot_w as f32 / 4.0;
    let x_offset = 2.0;
    let y_scale = plot_h as f32 / 10.0;
    let y_offset = 5.0;

    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_w * plot_h * 3).collect();

    println!("Plotting Cubic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);

    let spline = bspline::BSpline::new(degree, points, knots);

    println!("\tt range = {:?}", spline.knot_domain());

    plot_1d(&spline, &mut plot[..], (plot_w, plot_h), (x_scale, y_scale), (x_offset, y_offset));
    match image::save_buffer("cubic_1d.png", &plot[..], plot_w as u32, plot_h as u32, image::RGB(8)) {
        Ok(_) => println!("1D Cubic B-spline saved to cubic_1d.png"),
        Err(e) => println!("Error saving cubic_1d.png,  {}", e),
    }
}
/// Plot a simple 1D quartic B-spline
fn plot_quartic() {
    let points = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let knots = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0];
    let degree = 4;

    let plot_w = 720;
    let plot_h = 540;
    let x_scale = plot_w as f32 / 5.0;
    let y_scale = plot_h as f32 / 3.0;
    let y_offset = 1.5;

    let mut plot: Vec<_> = iter::repeat(255u8).take(plot_w * plot_h * 3).collect();

    println!("Plotting Quartic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);

    let spline = bspline::BSpline::new(degree, points, knots);

    println!("\tt range = {:?}", spline.knot_domain());
    println!("spline = {:?}", spline);

    plot_1d(&spline, &mut plot[..], (plot_w, plot_h), (x_scale, y_scale), (0.0, y_offset));
    match image::save_buffer("quartic_1d.png", &plot[..], plot_w as u32, plot_h as u32, image::RGB(8)) {
        Ok(_) => println!("1D Quartic B-spline saved to quartic_1d.png"),
        Err(e) => println!("Error saving quartic_1d.png,  {}", e),
    }
}

fn main() {
    let divider: String = iter::repeat('-').take(80).collect();
    plot_quadratic();
    println!("{}\n\n{}", divider, divider);
    plot_cubic();
    println!("{}\n\n{}", divider, divider);
    plot_quartic();
}

