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
//! # 1D Example
//!
//! This example shows how to create the 1D cardinal cubic B-spline example shown on [Wikipedia's
//! B-splines page](https://en.wikipedia.org/wiki/B-spline). For examples of evaluating the spline
//! to an image and saving the output see the [examples](https://github.com/Twinklebear/bspline/tree/master/examples).
//!
//! ```rust
//! let points = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
//! let knots = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
//! let degree = 3;
//! let spline = bspline::BSpline::new(degree, points, knots);
//! ```
//!
//! # Readings on B-splines
//!
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
use std::slice::Iter;
extern crate nalgebra;
use nalgebra as na;

/// The interpolate trait is used to linearly interpolate between two types (or in the
/// case of Quaternions, spherically linearly interpolate). The B-spline curve uses this
/// trait to compute points on the curve for the given parameter value.
///
/// A default implementation of this trait is provided for all `T` that are `Mul<f32, Output = T>
/// + Add<Output = T> + Copy` as these are the only operations needed to linearly interpolate the
/// values. Any type implementing this trait should perform whatever the appropriate linear
/// interpolaton is for the type.
pub trait Interpolate<F> {
    /// Linearly interpolate between `self` and `other` using `t`, for example with floats:
    ///
    /// ```text
    /// self * (1.0 - t) + other * t
    /// ```
    ///
    /// If the result returned is not a correct linear interpolation of the values the
    /// curve produced using the value may not be correct.
    fn interpolate(&self, other: &Self, t: F) -> Self;
}

impl<T: Mul<F, Output = T> + Add<Output = T> + Copy, F: na::RealField> Interpolate<F> for T {
    fn interpolate(&self, other: &Self, t: F) -> Self {
        *self * (F::one() - t) + *other * t
    }
}

/// Represents a B-spline curve that will use polynomials of the specified degree
/// to interpolate between the control points given the knots.
#[derive(Clone, Debug)]
pub struct BSpline<T: Interpolate<F> + Copy, F: na::RealField> {
    /// Degree of the polynomial that we use to make the curve segments
    degree: usize,
    /// Control points for the curve
    control_points: Vec<T>,
    /// The knot vector
    knots: Vec<F>,
}

impl<T: Interpolate<F> + Copy, F: na::RealField> BSpline<T, F> {
    /// Create a new B-spline curve of the desired `degree` that will interpolate
    /// the `control_points` using the `knots`. The knots should be sorted in non-decreasing
    /// order otherwise they will be sorted for you, which may lead to undesired knots
    /// for control points. Note that here we use the interpolating polynomial degree,
    /// if you're familiar with the convention of "B-spline curve order" the degree is `curve_order - 1`.
    ///
    /// Your curve must have a valid number of control points and knots or the function will panic. A B-spline
    /// curve requires at least as one more control point than the degree (`control_points.len() >
    /// degree`) and the number of knots should be equal to `control_points.len() + degree + 1`.
    pub fn new(degree: usize, control_points: Vec<T>, mut knots: Vec<F>) -> BSpline<T, F> {
        if control_points.len() <= degree {
            panic!("Too few control points for curve");
        }
        if knots.len() != control_points.len() + degree + 1 {
            panic!(format!("Invalid number of knots, got {}, expected {}", knots.len(),
                control_points.len() + degree + 1));
        }
        knots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        BSpline { degree, control_points, knots }
    }
    /// Compute a point on the curve at `t`, the parameter **must** be in the inclusive range
    /// of values returned by `knot_domain`. If `t` is out of bounds this function will assert
    /// on debug builds and on release builds you'll likely get an out of bounds crash.
    pub fn point(&self, t: F) -> T {
        debug_assert!(t >= self.knot_domain().0 && t <= self.knot_domain().1);
        // Find the first index with a knot value greater than the t we're searching for. We want
        // to find i such that: knot[i] <= t < knot[i + 1]
        let i = match upper_bounds(&self.knots[..], t) {
            Some(x) if x == 0 => self.degree,
            Some(x) if x >= self.knots.len() - self.degree - 1 =>
                self.knots.len() - self.degree - 1,
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
    pub fn knots(&self) -> Iter<F> {
        self.knots.iter()
    }
    /// Get the min and max knot domain values for finding the `t` range to compute
    /// the curve over. The curve is only defined over the inclusive range `[min, max]`,
    /// passing a `t` value outside of this range will result in an assert on debug builds
    /// and likely a crash on release builds.
    pub fn knot_domain(&self) -> (F, F) {
        (self.knots[self.degree], self.knots[self.knots.len() - 1 - self.degree])
    }
    /// Iteratively compute de Boor's B-spline algorithm, this computes the recursive
    /// de Boor algorithm tree from the bottom up. At each level we use the results
    /// from the previous one to compute this level and store the results in the
    /// array indices we no longer need to compute the current level (the left one
    /// used computing node j).
    fn de_boor_iterative(&self, t: F, i_start: usize) -> T {
        let mut tmp = Vec::with_capacity(self.degree + 1);
        for j in 0..=self.degree {
            let p = j + i_start - self.degree - 1;
            tmp.push(self.control_points[p]);
        }
        for lvl in 0..self.degree {
            let k = lvl + 1;
            for j in 0..self.degree - lvl {
                let i = j + k + i_start - self.degree;
                let alpha = (t - self.knots[i - 1]) / (self.knots[i + self.degree - k] - self.knots[i - 1]);
                debug_assert!(alpha.is_finite());
                tmp[j] = tmp[j].interpolate(&tmp[j + 1], alpha);
            }
        }
        tmp[0]
    }
}

/// Return the index of the first element greater than the value passed.
/// The data **must** be sorted. If no element greater than the value
/// passed is found the function returns None.
fn upper_bounds<F: na::RealField>(data: &[F], value: F) -> Option<usize> {
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
    use nalgebra as na;
    use std::ops::{Mul, Add};

    /// Check that the bspline returns the values we expect it to at various t values
    fn check_bspline<T: Mul<F, Output = T> + Add<Output = T> + Copy + PartialOrd, F: na::RealField>(spline: &BSpline<T, F>, expect: &Vec<(F, T)>) -> bool {
        expect.iter().fold(true, |ac, &(t, x)| ac && spline.point(t) == x)
    }

    #[test]
    fn linear_bspline() {
        let expect: Vec<(f32, f32)> = vec![(0.0, 0.0), (0.2, 0.2), (0.4, 0.4), (0.6, 0.6),
                          (0.8, 0.8), (1.0, 1.0)];
        let points: Vec<f32> = vec![0.0, 1.0];
        let knots: Vec<f32> = vec![0.0, 0.0, 1.0, 1.0];
        let degree = 1;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn quadratic_bspline() {
        let expect: Vec<(f32, f32)> = vec![(0.0, 0.0), (0.5, 0.125), (1.0, 0.5), (1.4, 0.74), (1.5, 0.75),
                          (1.6, 0.74), (2.0, 0.5), (2.5, 0.125), (3.0, 0.0)];
        let points: Vec<f32> = vec![0.0, 0.0, 1.0, 0.0, 0.0];
        let knots: Vec<f32> = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let degree = 2;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn cubic_bspline() {
        let expect: Vec<(f32, f32)> = vec![(-2.0, 0.0), (-1.5, 0.125), (-1.0, 1.0), (-0.6, 2.488),
                           (0.0, 4.0), (0.5, 2.875), (1.5, 0.12500001), (2.0, 0.0)];
        let points: Vec<f32> = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
        let knots: Vec<f32> = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
        let degree = 3;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn quartic_bspline() {
        let expect: Vec<(f32, f32)> = vec![(0.0, 0.0), (0.4, 0.0010666668), (1.0, 0.041666668),
                          (1.5, 0.19791667), (2.0, 0.4583333), (2.5, 0.5989583),
                          (3.0, 0.4583333), (3.2, 0.35206667), (4.1, 0.02733751),
                          (4.5, 0.002604167), (5.0, 0.0)];
        let points: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0];
        let degree = 4;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
    #[test]
    fn quartic_bspline_f64() {
        let expect: Vec<(f64, f64)> = vec![(0.0, 0.0), (0.4, 0.001066666666666667), (1.0, 0.041666666666666664),
                                           (1.5, 0.19791666666666666), (2.0, 0.45833333333333337), (2.5, 0.5989583333333334),
                                           (3.0, 0.4583333333333333), (3.2, 0.3520666666666666), (4.1, 0.027337500000000046),
                                           (4.5, 0.002604166666666666), (5.0, 0.0)];
        let points: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0];
        let degree = 4;
        let spline = BSpline::new(degree, points, knots);
        assert!(check_bspline(&spline, &expect));
    }
}

