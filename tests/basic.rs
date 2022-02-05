extern crate bspline;
use bspline::BSpline;
use std::ops::{Add, Mul};
extern crate trait_set;
use trait_set::trait_set;
extern crate num_traits;
trait_set! {
    pub trait Float = num_traits::Float;
}

#[cfg(feature = "nalgebra-support")]
extern crate nalgebra;
#[cfg(feature = "nalgebra-support")]
trait_set! {
    pub trait Float = nalgebra::RealField;
}

/// Check that the bspline returns the values we expect it to at various t values
fn check_bspline<T: Mul<F, Output = T> + Add<Output = T> + Copy + PartialOrd, F: Float>(
    spline: &BSpline<T, F>,
    expect: &Vec<(F, T)>,
) -> bool {
    expect
        .iter()
        .fold(true, |ac, &(t, x)| ac && spline.point(t) == x)
}

#[test]
fn linear_bspline() {
    let expect: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (0.2, 0.2),
        (0.4, 0.4),
        (0.6, 0.6),
        (0.8, 0.8),
        (1.0, 1.0),
    ];
    let points: Vec<f32> = vec![0.0, 1.0];
    let knots: Vec<f32> = vec![0.0, 0.0, 1.0, 1.0];
    let degree = 1;
    let spline = BSpline::new(degree, points, knots);
    assert!(check_bspline(&spline, &expect));
}
#[test]
fn quadratic_bspline() {
    let expect: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (0.5, 0.125),
        (1.0, 0.5),
        (1.4, 0.74),
        (1.5, 0.75),
        (1.6, 0.74),
        (2.0, 0.5),
        (2.5, 0.125),
        (3.0, 0.0),
    ];
    let points: Vec<f32> = vec![0.0, 0.0, 1.0, 0.0, 0.0];
    let knots: Vec<f32> = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
    let degree = 2;
    let spline = BSpline::new(degree, points, knots);
    assert!(check_bspline(&spline, &expect));
}
#[test]
fn cubic_bspline() {
    let expect: Vec<(f32, f32)> = vec![
        (-2.0, 0.0),
        (-1.5, 0.125),
        (-1.0, 1.0),
        (-0.6, 2.488),
        (0.0, 4.0),
        (0.5, 2.875),
        (1.5, 0.12500001),
        (2.0, 0.0),
    ];
    let points: Vec<f32> = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
    let knots: Vec<f32> = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
    let degree = 3;
    let spline = BSpline::new(degree, points, knots);
    assert!(check_bspline(&spline, &expect));
}
#[test]
fn quartic_bspline() {
    let expect: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (0.4, 0.0010666668),
        (1.0, 0.041666668),
        (1.5, 0.19791667),
        (2.0, 0.4583333),
        (2.5, 0.5989583),
        (3.0, 0.4583333),
        (3.2, 0.35206667),
        (4.1, 0.02733751),
        (4.5, 0.002604167),
        (5.0, 0.0),
    ];
    let points: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let knots: Vec<f32> = vec![
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0,
    ];
    let degree = 4;
    let spline = BSpline::new(degree, points, knots);
    assert!(check_bspline(&spline, &expect));
}
#[test]
fn quartic_bspline_f64() {
    let expect: Vec<(f64, f64)> = vec![
        (0.0, 0.0),
        (0.4, 0.001066666666666667),
        (1.0, 0.041666666666666664),
        (1.5, 0.19791666666666666),
        (2.0, 0.45833333333333337),
        (2.5, 0.5989583333333334),
        (3.0, 0.4583333333333333),
        (3.2, 0.3520666666666666),
        (4.1, 0.027337500000000046),
        (4.5, 0.002604166666666666),
        (5.0, 0.0),
    ];
    let points: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let knots: Vec<f64> = vec![
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0,
    ];
    let degree = 4;
    let spline = BSpline::new(degree, points, knots);
    assert!(check_bspline(&spline, &expect));
}
