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

# Crate

# Documentation

Rust doc can be found [here](http://www.willusher.io/bspline/bspline/).

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

