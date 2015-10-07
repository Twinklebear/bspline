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
        // TODO: Maybe a ctor for cardinal curves that will check we have the right number of
        // knots? Is this check correct?
        if control_points.len() < degree {
            panic!("Too few control points for curve");
        }
        knots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        BSpline { degree: degree, control_points: control_points, knots: knots }
    }
    /// Compute a point on the curve at `t`
    /// TODO: Handling of out of bounds t values? Are t values not in the domain considered
    /// out of range? The extra `degree` values are just for when we're going down the tree
    /// to find the interpolated values there right?
    pub fn point(&self, t: f32) -> T {
        // Find the first index with a knot value greater than the t we're searching for. We want
        // to find i such that: knot[i] <= t < knot[i + 1]
        let (mut i, _) = self.knots.iter().take_while(|&x| !(*x > t)).enumerate().last().unwrap();
        i = i + 1;
        if i == self.knots.len() {
            i = self.knots.len() - self.degree - 1;
        } else if i == 0 {
            i = self.degree;
        }
        /*
        let i = match upper_bounds(&self.knots[self.degree - 1..self.knots.len() - self.degree], t) {
            Some(x) => x,
            None => self.knots.len() - self.degree - 1,
        };
        */
        //println!("Found i = {} for t = {}\n\tknots = {:?}", i, t, self.knots);
        //println!("degree = {}", self.degree);
        self.de_boors(t, self.degree, i)
    }
    /// Recursively compute de Boor's B-spline algorithm. TODO: This is terrible,
    /// compute it iteratively! Recursive version is just for a simple formualation
    /// of the initial implementation.
    fn de_boors(&self, t: f32, k: usize, i: usize) -> T {
        if k == 0 {
            //println!("Returning control point {}", i);
            self.control_points[i - 1]
        } else {
            //println!("Looking at k = {}, i = {}, t = {}", k, i, t);
            //println!("\tknots[i + self.degree - k] = {}, self.knots[i - 1] = {}",
            //         self.knots[i + self.degree - k], self.knots[i - 1]);
            // TODO: This is still broken
            let alpha = (t - self.knots[i - 1]) / (self.knots[i + self.degree - k] - self.knots[i - 1]);
            self.de_boors(t, k - 1, i - 1) * (1.0 - alpha) + self.de_boors(t, k - 1, i) * alpha
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

    //#[test]
    fn linear_bspline() {
        let points = vec![Point::new(-1.0, 0.0), Point::new(0.0, 1.0),
                          Point::new(1.0, 1.0), Point::new(1.0, 2.0)];
        let knots = vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0];
        let spline = BSpline::new(1, points, knots);
        let x = spline.point(1.5);
        println!("spline(1.5) = {:?}", x);
        assert!(x.x == 0.5 && x.y == 1.0);
    }
    //#[test]
    fn quadratic_bspline() {
        let points = vec![Point::new(-2.0, 0.0), Point::new(0.0, 2.0),
                          Point::new(2.0, 0.0), Point::new(0.0, -2.0)];
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0];
        let spline = BSpline::new(3, points, knots);
        let x = spline.point(1.5);
        println!("spline(1.5) = {:?}", x);
        assert!(x.x == 1.25 && x.y == -0.25);
    }
    #[test]
    fn bspline_plot() {
        let points = vec![Pointi::new(0, 5), Pointi::new(9, 20),
                          Pointi::new(29, 20), Pointi::new(39, 5)];
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0];

        // TODO: This doesn't compute or plot the correct curve
        let plot_w = 80;
        let plot_h = 50;
        let mut plot: Vec<_> = iter::repeat(' ').take(plot_w * plot_h).collect();

        let t_start = knots[0];
        let t_end = knots[knots.len() - 1];
        println!("Starting at {}, ending at {}", t_start, t_end);
        let spline = BSpline::new(2, points.clone(), knots);

        let steps = 20;
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
        for i in 0..points.len() {
            let symbol = match i {
                x if x == 0 => 'S',
                x if x == points.len() - 1 => 'E',
                _ => 'C',
            };
            plot[(plot_h - 1 - points[i].y as usize) * plot_w + points[i].x as usize] = symbol;
        }

        for y in 0..plot_h {
            for x in 0..plot_w {
                print!("{}", plot[y * plot_w + x]);
            }
            println!("");
        }
    }
    //#[test]
    fn quadratic_bspline_plot1d() {
        let points = vec![0.0, 0.0, 1.0, 0.0, 0.0];
        let knots = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0];
        let t_start = knots[0];
        let t_end = knots[knots.len() - 1];
        println!("Starting at {}, ending at {}", t_start, t_end);
        let spline = BSpline::new(2, points, knots);
        let t = 0.5;
        println!("spline({}) = {}", t, spline.point(t));
    }
    //#[test]
    fn cubic_bspline_plot1d() {
        let points = vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
        let knots = vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0];
        let t_start = knots[0];
        let t_end = knots[knots.len() - 1];
        println!("Starting at {}, ending at {}", t_start, t_end);
        let spline = BSpline::new(3, points, knots);
        let t = 0.5;
        println!("spline({}) = {}", t, spline.point(t));
    }
    //#[test]
    fn quartic_bspline_plot1d() {
        let points = vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0];
        let t_start = knots[0];
        let t_end = knots[knots.len() - 1];
        println!("Starting at {}, ending at {}", t_start, t_end);
        let spline = BSpline::new(4, points, knots);
        let t = 3.5;
        println!("spline({}) = {}", t, spline.point(t));
    }
}

