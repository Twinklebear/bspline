extern crate bspline;

use std::iter;

/// Plot a simple 1D quadratic B-spline
fn plot_quadratic() {
    let points = vec![0.0, 0.0, 1.0, 0.0, 0.0];
    let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    let t_start = knots[0];
    let t_end = knots[knots.len() - 1];

    let plot_w = 80;
    let plot_h = 30;
    let x_scale = plot_w as f32 / knots[knots.len() - 1];
    let y_scale = plot_h as f32 / 1.0;

    let mut plot: Vec<_> = iter::repeat(' ').take(plot_w * plot_h).collect();

    println!("Plotting Quadratic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);
    println!("\tStarting at {}, ending at {}", t_start, t_end);
    let spline = bspline::BSpline::new(2, points, knots);

    let step_size = 0.001;
    let steps = ((t_end - t_start) / step_size) as usize;
    for s in 0..steps {
        let t = step_size * s as f32 + t_start;
        let y = spline.point(t);
        let iy = (y * y_scale) as isize;
        let ix = (t * x_scale) as isize;
        if iy >= 0 && iy < plot_h as isize {
            plot[(plot_h - 1 - iy as usize) * plot_w + ix as usize] = 'O';
        }
    }
    for y in 0..plot_h {
        for x in 0..plot_w {
            print!("{}", plot[y * plot_w + x]);
        }
        println!("");
    }
}
/// Plot a simple 1D cubic B-spline
fn plot_cubic() {
    let points = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
    let knots = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
    let t_start = knots[0];
    let t_end = knots[knots.len() - 1];

    let plot_w = 80;
    let plot_h = 30;
    let x_scale = plot_w as f32 / 4.0;
    let x_offset = 2.0;
    let y_scale = plot_h as f32 / 6.0;

    let mut plot: Vec<_> = iter::repeat(' ').take(plot_w * plot_h).collect();

    println!("Plotting Cubic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);
    println!("\tStarting at {}, ending at {}", t_start, t_end);
    let spline = bspline::BSpline::new(3, points, knots);

    let step_size = 0.001;
    let steps = ((t_end - t_start) / step_size) as usize;
    for s in 0..steps {
        let t = step_size * s as f32 + t_start;
        let y = spline.point(t);
        let iy = (y * y_scale) as isize;
        let ix = ((t + x_offset) * x_scale) as isize;
        if iy >= 0 && iy < plot_h as isize {
            plot[(plot_h - 1 - iy as usize) * plot_w + ix as usize] = 'O';
        }
    }
    for y in 0..plot_h {
        for x in 0..plot_w {
            print!("{}", plot[y * plot_w + x]);
        }
        println!("");
    }
}
/// Plot a simple 1D quartic B-spline
fn plot_quartic() {
        let points = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0];
    let t_start = knots[0];
    let t_end = knots[knots.len() - 1];

    let plot_w = 80;
    let plot_h = 30;
    let x_scale = plot_w as f32 / 5.0;
    let y_scale = plot_h as f32 / 1.0;

    let mut plot: Vec<_> = iter::repeat(' ').take(plot_w * plot_h).collect();

    println!("Plotting Quartic B-spline with:\n\tpoints = {:?}\n\tknots = {:?}",
             points, knots);
    println!("\tStarting at {}, ending at {}", t_start, t_end);
    let spline = bspline::BSpline::new(4, points, knots);

    let step_size = 0.001;
    let steps = ((t_end - t_start) / step_size) as usize;
    for s in 0..steps {
        let t = step_size * s as f32 + t_start;
        let y = spline.point(t);
        let iy = (y * y_scale) as isize;
        let ix = (t * x_scale) as isize;
        if iy >= 0 && iy < plot_h as isize {
            plot[(plot_h - 1 - iy as usize) * plot_w + ix as usize] = 'O';
        }
    }
    for y in 0..plot_h {
        for x in 0..plot_w {
            print!("{}", plot[y * plot_w + x]);
        }
        println!("");
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

