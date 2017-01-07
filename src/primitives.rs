use num::Float;
use ffi;

/// A 2D point.
#[repr(C)]
pub struct Point<T: Float> {
    pub x: T,
    pub y: T,
}

/// A directed line segment between two points
#[repr(C)]
pub struct LineSegment<T: Float> {
    pub from: Point<T>,
    pub to: Point<T>,
}

/// Represents a position of a point relative to a geometric primitive
///  such as a line segment or circle.
#[derive(PartialEq, Debug)]
pub enum Orientation {
    /// Inside a ccw circle, inside a ccw polygon, or to the left of a line segment
    /// Note: clockwise primitives give reversed orientation.
    Inside,
    /// Exactly on a geometric primitive (circle, polygon, line segment, ...)
    On,
    /// Outside a ccw circle, outside a ccw polygon, or to the right of a line segment.
    /// Note: clockwise primitives give reversed orientation.
    Outside,
}

impl<T: Float> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}

impl Point<f64> {
    /// Relates this point to a line through two points. Inside means to the left
    /// , that is, the argument points followed by this point makes a left turn.
    pub fn orient_to_points(&self, p1: &Point<f64>, p2: &Point<f64>) -> Orientation {
        unsafe {
            let t = ffi::orient2d(&p1.x, &p2.x, &self.x);
            Orientation::from_signed_distance(t)
        }
    }

    /// Relates this point to a circle through three points.
    /// The three circle points must form a counterclockwise turn or relation will be reversed.
    pub fn orient_to_circle(&self,
                            p1: &Point<f64>,
                            p2: &Point<f64>,
                            p3: &Point<f64>)
                            -> Orientation {
        unsafe {
            let t = ffi::incircle(&p1.x, &p2.x, &p3.x, &self.x);
            Orientation::from_signed_distance(t)
        }
    }
}

impl Orientation {
    /// Constructs a new `Orientation` from a signed distance.
    pub fn from_signed_distance<T: Float>(t: T) -> Orientation {
        if t.is_zero() {
            Orientation::On
        } else if t.is_sign_positive() {
            Orientation::Inside
        } else {
            Orientation::Outside
        }
    }
}


#[cfg(test)]
mod tests {
    use ::arithmetic;
    use primitives::Point;
    use primitives::Orientation;


    #[test]
    fn orient_points() {
        let _ = arithmetic::Library::init();
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(1.0, 1.0);
        let d = Point::new(2.0, 0.0);

        // Normal
        assert!(matches!(c.orient_to_points(&a, &b), Orientation::Inside));
        assert!(matches!(c.orient_to_points(&b, &a), Orientation::Outside));
        assert!(matches!(d.orient_to_points(&a, &b), Orientation::On));
        // Degenerate
        assert!(matches!(a.orient_to_points(&a, &b), Orientation::On));
        assert!(matches!(b.orient_to_points(&a, &b), Orientation::On));
        assert!(matches!(a.orient_to_points(&a, &a), Orientation::On));
    }

    #[test]
    fn in_circle() {
        let _ = arithmetic::Library::init();

        // Circle c=(0,1) r=1
        let a = Point::new(-1.0, 1.0);
        let b = Point::new(1.0, 1.0);
        let c = Point::new(0.0, 2.0);

        // Test points
        let p1 = Point::new(0.0, 0.0); // On
        let p2 = Point::new(0.1, 0.5); // Inside
        let p3 = Point::new(0.1, -0.5); // Outside

        assert!(matches!(p1.orient_to_circle(&a, &b, &c), Orientation::On));
        assert!(matches!(p2.orient_to_circle(&a, &b, &c), Orientation::Inside));
        assert!(matches!(p3.orient_to_circle(&a, &b, &c), Orientation::Outside));
    }
}
