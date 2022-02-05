[![logo](http://i.imgur.com/dnpEXyh.jpg)](http://i.imgur.com/RUEw8EW.png)
===
A library for computing B-spline interpolating curves on generic control points. bspline can
be used to evaluate B-splines of varying orders on any type that can be linearly interpolated,
ranging from floats, positions, RGB colors to transformation matrices and so on.

The bspline logo was generated using this library with a cubic B-spline in 2D for the positioning
of the curve and a quadratic B-spline in RGB space to color it (check out the
[logo](https://github.com/Twinklebear/bspline/blob/master/examples/logo.rs) example!). Other
much simpler examples of 1D and 2D quadratic, cubic and quartic B-splines can also be found in
the [examples](https://github.com/Twinklebear/bspline/tree/master/examples).

# Installation

Just grab the crate on [crates.io](https://crates.io/crates/bspline) and you're set!

[![Crates.io](https://img.shields.io/crates/v/bspline.svg)](https://crates.io/crates/bspline)
![Build Status](https://github.com/Twinklebear/bspline/workflows/CI/badge.svg)

# Documentation

Rust doc can be found [here](https://docs.rs/bspline/).

# 1D Example

This example shows how to create the 1D cardinal cubic B-spline example shown on [Wikipedia's
B-splines page](https://en.wikipedia.org/wiki/B-spline). For examples of evaluating the spline
to an image and saving the output see the [examples](https://github.com/Twinklebear/bspline/tree/master/examples).

```rust
let points = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0]
let knots = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
let degree = 3;
let spline = bspline::BSpline::new(degree, points, knots);
```

## Readings on B-splines
The library assumes you are familiar at some level with how B-splines work, e.g. how
control points and knots and effect the curve produced. No interactive
editor is provided (at least currently). Some good places to start reading about B-splines to
effectively use this library can be found below.

- [Wikipedia page on B-splines](https://en.wikipedia.org/wiki/B-spline)
- [Fundamentals of Computer Graphics](http://www.amazon.com/Fundamentals-Computer-Graphics-Peter-Shirley/dp/1568814690)
(has a good chapter on curves)
- [Splines and B-splines: An Introduction](http://www.uio.no/studier/emner/matnat/ifi/INF-MAT5340/v07/undervisningsmateriale/kap1.pdf)
- [Geometric Modeling](http://atrey.karlin.mff.cuni.cz/projekty/vrr/doc/grafika/geometric%20modelling.pdf)
- [A nice set of interactive examples](https://www.ibiblio.org/e-notes/Splines/Intro.htm)

# nalgebra support

[nalgerba](https://docs.rs/nalgebra/latest/nalgebra/) is one of the most popular linear algbera packages for Rust. To make this create compatible with it, you need to enable the `nalgebra-support` feature and then you can recreate the above example:

```rust
use nalgebra as na;

let points = na::DVector::from(vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0])
let knots = na::DVector::from(vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0]);
let degree = 3;
let spline = bspline::BSpline::new(degree, points, knots);
```

