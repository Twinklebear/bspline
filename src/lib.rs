use std::ops::{Mul, Add};

/// Structure for computing the B-spline with the given control points
/// and knots.
pub struct BSpline<T: Mul<f32, Output = T> + Add<Output = T> + Copy> {
    /// Degree of the polynomial that we use to make the curve segments
    degree: usize,
    /// Control points for the curve
    control_points: Vec<T>,
    /// TODO: What is a good description for the knots?
    knots: Vec<f32>,
}

impl<T: Mul<f32, Output = T> + Add<Output = T> + Copy> BSpline<T> {
    /// Create a new B-spline curve of the desired degree that will blend between
    /// the passed control points using the knots. The knots should be sorted in ascending
    /// order, otherwise they will be sorted for you which may lead to undesired knots
    /// for control points
    pub fn new(degree: usize, control_points: Vec<T>, mut knots: Vec<f32>) -> BSpline<T> {
        knots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        BSpline { degree: degree, control_points: control_points, knots: knots }
    }
    /// Compute a point on the curve at `t`
    /// TODO: Handling of out of bounds t values? Are t values not in the domain considered
    /// out of range? The extra `degree` values are just for when we're going down the tree
    /// to find the interpolated values there right?
    pub fn point(&self, t: f32) -> T {
        // Find the first index with a knot value greater than the t we're searching for. We want
        // to find i such that: knot[i - 1] <= t < knot[i]
        let i = match upper_bounds(&self.knots[self.degree - 1..self.knots.len() - self.degree], t) {
            Some(x) => x,
            None => self.knots.len() - self.degree - 1,
        };
        self.de_boors(t, self.degree, i)
    }
    /// Recursively compute de Boor's B-spline algorithm. TODO: This is terrible,
    /// compute it iteratively! Recursive version is just for a simple formualation
    /// of the initial implementation.
    fn de_boors(&self, t: f32, k: usize, i: usize) -> T {
        if k == 0 {
            self.control_points[i]
        } else {
            println!("Looking at k = {}, i = {}", k, i);
            let denom = self.knots[i + self.degree - k] - self.knots[i - 1];
            let a = (self.knots[i + self.degree - k] - t) / denom;
            let b = (t - self.knots[i - 1]) / denom;
            self.de_boors(t, k - 1, i - 1) * a + self.de_boors(t, k - 1, i) * b
        }
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
    use std::ops::{Mul, Add};
    use std::iter;

    use super::BSpline;

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

    #[derive(Copy, Clone, Debug)]
    struct Pointi {
        x: i32,
        y: i32,
    }
    impl Pointi {
        fn new(x: i32, y: i32) -> Pointi {
            Pointi { x: x, y: y }
        }
    }
    impl Mul<f32> for Pointi {
        type Output = Pointi;
        fn mul(self, rhs: f32) -> Pointi {
            Pointi { x: (self.x as f32 * rhs) as i32,
                    y: (self.y as f32 * rhs) as i32 }
        }
    }
    impl Add for Pointi {
        type Output = Pointi;
        fn add(self, rhs: Pointi) -> Pointi {
            Pointi { x: self.x + rhs.x, y: self.y + rhs.y }
        }
    }

    #[test]
    fn linear_bspline(){
        let points = vec![Point::new(-1.0, 0.0), Point::new(0.0, 1.0),
                          Point::new(1.0, 1.0), Point::new(1.0, 2.0)];
        let knots = vec![0.0, 1.0, 2.0, 3.0];
        let spline = BSpline::new(1, points, knots);
        let x = spline.point(1.5);
        println!("spline(1.5) = {:?}", x);
        assert!(x.x == 0.5 && x.y == 1.0);
    }

    #[test]
    fn bspline_plot(){
        let points = vec![Pointi::new(10, 10), Pointi::new(20, 20), Pointi::new(10, 20)];
        let knots = vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0];
        let spline = BSpline::new(2, points, knots);
        let x = spline.point(1.5);
        println!("spline(1.5) = {:?}", x);

        // TODO: This doesn't compute or plot the correct curve
        let plot_w = 40;
        let plot_h = 40;
        let mut plot: Vec<_> = iter::repeat(' ').take(plot_w * plot_h).collect();

        let t_start = 1.0;
        let t_end = 2.0;
        let steps = 10;
        let step_size = (t_end - t_start) / steps as f32;
        for s in 0..steps {
            let t = step_size * s as f32 + t_start;
            println!("t = {}", t);
            let x = spline.point(t);
            println!("x = {:?}", x);
            if x.x > -1 && x.x < plot_w as i32 && x.y > -1 && x.y < plot_h as i32 {
                plot[(plot_h - 1 - x.y as usize) * plot_w + x.x as usize] = 'X';
            }
        }

        for y in 0..plot_h {
            for x in 0..plot_w {
                print!("{}", plot[y * plot_w + x]);
            }
            println!("");
        }
    }
}

