//! [![logo](http://i.imgur.com/dnpEXyh.jpg)](http://i.imgur.com/RUEw8EW.png)
//!
//! bspline
//! ===
//! A library for computing B-spline interpolating curves on generic control points. bspline can
//! be used to evaluate B-splines of varying orders on any type that can be linearly interpolated,
//! ranging from floats, positions, RGB colors to transformation matrices and so on.
//!
//! The bspline logo was generated using this library with a cubic B-spline in 2D for the positioning
//! of the curve and a quadratic B-spline in RGB space to color it (check out the
//! [logo](https://github.com/Twinklebear/bspline/blob/master/examples/logo.rs) example!). Other
//! much simpler examples of 1D and 2D quadratic, cubic and quartic B-splines can also be found in
//! the [examples](https://github.com/Twinklebear/bspline/tree/master/examples).
//!
//! # Readings on B-splines
//! The library assumes you are familiar at some level with how B-splines work, e.g. how
//! control points and knots and effect the curve produced. No interactive
//! editor is provided (at least currently). Some good places to start reading about B-splines to
//! effectively use this library can be found below.
//!
//! - [Wikipedia page on B-splines](https://en.wikipedia.org/wiki/B-spline)
//! - [Fundamentals of Computer Graphics](http://www.amazon.com/Fundamentals-Computer-Graphics-Peter-Shirley/dp/1568814690)
//! (has a good chapter on curves)
//! - [Splines and B-splines: An Introduction](http://www.uio.no/studier/emner/matnat/ifi/INF-MAT5340/v07/undervisningsmateriale/kap1.pdf)
//! - [Geometric Modeling](http://atrey.karlin.mff.cuni.cz/projekty/vrr/doc/grafika/geometric%20modelling.pdf)
//! - [A nice set of interactive examples](https://www.ibiblio.org/e-notes/Splines/Intro.htm)
//!

use std::ops::{Mul, Add};
use std::fmt::Debug;
use std::slice::Iter;

/// The interpolate trait is used to linearly interpolate between two types (or in the
/// case of Quaternions, spherically linearly interpolate). The B-spline curve uses this
/// trait to compute points on the curve for the given parameter value.
///
/// A default implementation of this trait is provided for all `T` that are `Mul<f32, Output = T>
/// + Add<Output = T> + Copy` since the interpolation provided by the trait is expected to be
/// a simple linear interpolation.
pub trait Interpolate {
    /// Linearly interpolate between `self` and `other` using `t`, for example with floats:
    ///
    /// ```text
    /// self * (1.0 - t) + other * t
    /// ```
    ///
    /// If the result returned is not a correct linear interpolation of the values the
    /// curve produced using the value may not be correct.
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

impl<T: Mul<f32, Output = T> + Add<Output = T> + Copy> Interpolate for T {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        *self * (1.0 - t) + *other * t
    }
}

/// Represents a B-spline that will use polynomials of the specified degree to interpolate
/// between the control points given the knots.
pub struct BSpline<T: Interpolate + Copy + Debug> {
    /// Degree of the polynomial that we use to make the curve segments
    degree: usize,
    /// Control points for the curve
    control_points: Vec<T>,
    /// The knot vector
    knots: Vec<f32>,
}

impl<T: Interpolate + Copy + Debug> BSpline<T> {
    /// Create a new B-spline curve of the desired `degree` that will interpolate
    /// the `control_points` using the `knots`. The knots should be sorted in non-decreasing
    /// order, otherwise they will be sorted for you which may lead to undesired knots
    /// for control points. Note that this is in terms of the interpolating polynomial degree,
    /// if you are familiar with the convention of "B-spline curve order" the degree is `curve_order - 1`.
    ///
    /// Your curve must have a valid number of control points and knots or the function will panic. A B-spline
    /// curve requires at least as many control points as the degree (`control_points.len() >=
    /// degree`) and the number of knots should be equal to `control_points.len() + degree + 1`.
    pub fn new(degree: usize, control_points: Vec<T>, mut knots: Vec<f32>) -> BSpline<T> {
        if control_points.len() < degree {
            panic!("Too few control points for curve");
        }
        if knots.len() != control_points.len() + degree + 1 {
            panic!(format!("Invalid number of knots, got {}, expected {}", knots.len(),
                control_points.len() + degree + 1));
        }
        knots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        BSpline { degree: degree, control_points: control_points, knots: knots }
    }
    /// Compute a point on the curve at `t`, the parameter **must** be in the inclusive range
    /// of values returned by `knot_domain`. If `t` is out of bounds this function will assert
    /// on debug builds and on release builds you'll likely get an out of bounds crash.
    pub fn point(&self, t: f32) -> T {
        debug_assert!(t >= self.knot_domain().0 && t <= self.knot_domain().1);
        // Find the first index with a knot value greater than the t we're searching for. We want
        // to find i such that: knot[i] <= t < knot[i + 1]
        let i = match upper_bounds(&self.knots[..], t) {
            Some(x) if x == 0 => self.degree,
            Some(x) => x,
            None => self.knots.len() - self.degree - 1,
        };
        self.de_boor_iterative(t, i)
    }
    /// Get an iterator over the control points.
    pub fn control_points(&self) -> Iter<T> {
        self.control_points.iter()
    }
    /// Get an iterator over the knots.
    pub fn knots(&self) -> Iter<f32> {
        self.knots.iter()
    }
    /// Get the min and max knot domain values for finding the `t` range to compute
    /// the curve over. The curve is only defined over this (inclusive) range, passing
    /// a `t` value out of this range will assert on debug builds and likely result in
    /// a crash on release builds.
    pub fn knot_domain(&self) -> (f32, f32) {
        (self.knots[self.degree], self.knots[self.knots.len() - 1 - self.degree])
    }
    /// Iteratively compute de Boor's B-spline algorithm, this computes the recursive
    /// de Boor algorithm tree from the bottom up. At each level we use the results
    /// from the previous one to compute this level and store the results in the
    /// array indices we no longer need to compute the current level (the left one
    /// used computing node j).
    fn de_boor_iterative(&self, t: f32, i_start: usize) -> T {
        let mut tmp = Vec::with_capacity(self.degree + 1);
        for j in 0..self.degree + 1 {
            let p = j + i_start - self.degree - 1;
            tmp.push(self.control_points[p]);
        }
        for lvl in 0..self.degree {
            let k = lvl + 1;
            for j in 0..self.degree - lvl {
                let i = j + k + i_start - self.degree;
                let alpha = (t - self.knots[i - 1]) / (self.knots[i + self.degree - k] - self.knots[i - 1]);
                tmp[j] = tmp[j].interpolate(&tmp[j + 1], alpha);
            }
        }
        tmp[0]
    }
}

/// Return the index of the first element greater than the value passed.
/// The data **must** be sorted.
/// If no element greater than the value passed is found the function returns None.
fn upper_bounds(data: &[f32], value: f32) -> Option<usize> {
    let mut first = 0usize;
    let mut step;
    let mut count = data.len() as isize;
    while count > 0 {
        step = count / 2;
        let it = first + step as usize;
        if !value.lt(&data[it]) {
            first = it + 1;
            count -= step + 1;
        } else {
            count = step;
        }
    }
    // If we didn't find an element greater than value
    if first == data.len() {
        None
    } else {
        Some(first)
    }
}

#[cfg(test)]
mod test {
    use super::BSpline;

    /// Check that the bspline returns the values we expect it to at various t values
    fn check_bspline(spline: &BSpline<f32>, expect: &Vec<(f32, f32)>) -> bool {
        expect.iter().fold(true, |ac, &(t, x)| ac && spline.point(t) == x)
    }

    #[test]
    fn linear_bspline() {
        let expect = vec![(0.0, 0.0), (0.2, 0.2), (0.4, 0.4), (0.6, 0.6),
                          (0.8, 0.8), (1.0, 1.0)];
        let points = vec![0.0, 1.0];
        let knots = vec![0.0, 0.0, 1.0, 1.0];
        let degree = 1;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn quadratic_bspline() {
        let expect = vec![(0.0, 0.0), (0.5, 0.125), (1.0, 0.5), (1.4, 0.74), (1.5, 0.75),
                          (1.6, 0.74), (2.0, 0.5), (2.5, 0.125), (3.0, 0.0)];
        let points = vec![0.0, 0.0, 1.0, 0.0, 0.0];
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let degree = 2;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn cubic_bspline() {
        let expect = vec![(-2.0, 0.0), (-1.5, 0.125), (-1.0, 1.0), (-0.6, 2.488),
                           (0.0, 4.0), (0.5, 2.875), (1.5, 0.12500001), (2.0, 0.0)];
        let points = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
        let knots = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
        let degree = 3;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn quartic_bspline() {
        let expect = vec![(0.0, 0.0), (0.4, 0.0010666668), (1.0, 0.041666668),
                          (1.5, 0.19791667), (2.0, 0.4583333), (2.5, 0.5989583),
                          (3.0, 0.4583333), (3.2, 0.35206667), (4.1, 0.02733751),
                          (4.5, 0.002604167), (5.0, 0.0)];
        let points = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0];
        let degree = 4;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
}

