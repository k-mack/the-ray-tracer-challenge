use std::ops::{Add, Neg, Sub};

/// Epsilon used for floating-point comparisons
const EPSILON: f64 = 1e-6;

struct RayTracerTuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: i8,
}

impl RayTracerTuple {
    /// Create a point tuple.
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1 }
    }

    /// Create a vector tuple
    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0 }
    }

    /// Test if the tuple is a point.
    pub fn is_point(&self) -> bool {
        self.w == 1
    }

    /// Test if the tuple is a point.
    pub fn is_vector(&self) -> bool {
        self.w == 0
    }

    /// Test if this tuple is equal to another.
    /// Note that this only considers the cartesian coordinates of the two tuples.
    pub fn is_equal_to(&self, other: &RayTracerTuple) -> bool {
        if (self.x - other.x).abs() < EPSILON {
            if (self.y - other.y).abs() < EPSILON {
                if (self.z - other.z).abs() < EPSILON {
                    return true;
                }
            }
        }
        false
    }
}

/// Implement the `Add` trait specifically for tuple references.
/// For any tuple reference with lifetime `a`, implement `Add` for it such that it can be added with another tuple reference with a different lifetime `b`.
/// We want to implement this trait for reference tuples because we want to be able to use the operands afterwards
/// (i.e., we do not want the `add` function to own the operands).
impl<'a, 'b> Add<&'b RayTracerTuple> for &'a RayTracerTuple {
    type Output = RayTracerTuple;

    /// Add two tuple references.
    fn add(self, rhs: &'b RayTracerTuple) -> RayTracerTuple {
        RayTracerTuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

/// Similar comments to those for `Add`.
impl<'a, 'b> Sub<&'b RayTracerTuple> for &'a RayTracerTuple {
    type Output = RayTracerTuple;

    /// Subtract two tuple reference.
    fn sub(self, rhs: &'b RayTracerTuple) -> RayTracerTuple {
        RayTracerTuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

//// Similar comment to those for `Add`.
impl<'a> Neg for &'a RayTracerTuple {
    type Output = RayTracerTuple;

    fn neg(self) -> RayTracerTuple {
        RayTracerTuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_new_point() {
        let tuple = RayTracerTuple::new_point(4.3, -4.2, 3.1);
        assert!((tuple.x - 4.3).abs() < EPSILON);
        assert!((tuple.y - -4.2).abs() < EPSILON);
        assert!((tuple.z - 3.1).abs() < EPSILON);
        assert_eq!(tuple.w, 1);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_new_vector() {
        let tuple = RayTracerTuple::new_vector(4.3, -4.2, 3.1);
        assert!((tuple.x - 4.3).abs() < EPSILON);
        assert!((tuple.y - -4.2).abs() < EPSILON);
        assert!((tuple.z - 3.1).abs() < EPSILON);
        assert_eq!(tuple.w, 0);
        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }

    #[test]
    fn tuple_is_equal_to() {
        let point = RayTracerTuple::new_point(4.3, -4.2, 3.1);
        let vector = RayTracerTuple::new_vector(4.3, -4.2, 3.1);
        assert!(point.is_equal_to(&vector));
        assert!(vector.is_equal_to(&point));

        let not_quite_different = RayTracerTuple::new_point(4.3 + 1e-7, -4.2, 3.1);
        assert!(point.is_equal_to(&not_quite_different));

        let barely_different = RayTracerTuple::new_point(4.3 + EPSILON, -4.2, 3.1);
        assert!(!point.is_equal_to(&barely_different));
    }

    #[test]
    fn tuple_add_refs() {
        let point1 = RayTracerTuple::new_point(3.0, -2.0, 5.0);
        let vector1 = RayTracerTuple::new_vector(-2.0, 3.0, 1.0);

        let point1_plus_vector1 = &point1 + &vector1;
        assert!(point1_plus_vector1.is_equal_to(&RayTracerTuple::new_point(1.0, 1.0, 6.0)));
        assert!(point1_plus_vector1.is_point());

        let point2 = RayTracerTuple::new_point(3.0, -2.0, 5.0);
        let point1_plus_point2 = &point1 + &point2;
        assert!(point1_plus_point2.is_equal_to(&RayTracerTuple::new_point(6.0, -4.0, 10.0)));
        assert_eq!(2, point1_plus_point2.w); // a weird reality
    }

    #[test]
    fn tuple_sub_refs() {
        let point1 = RayTracerTuple::new_point(3.0, 2.0, 1.0);
        let point2 = RayTracerTuple::new_point(5.0, 6.0, 7.0);

        let point1_minus_point2 = &point1 - &point2;
        assert!(point1_minus_point2.is_equal_to(&RayTracerTuple::new_vector(-2.0, -4.0, -6.0)));
        assert!(point1_minus_point2.is_vector());

        let vector1 = RayTracerTuple::new_vector(5.0, 6.0, 7.0);
        let point1_minus_vector1 = &point1 - &vector1;
        assert!(point1_minus_vector1.is_equal_to(&RayTracerTuple::new_vector(-2.0, -4.0, -6.0)));
        assert!(point1_minus_vector1.is_point());

        let vector2 = RayTracerTuple::new_vector(3.0, 2.0, 1.0);
        let vector2_minus_vector1 = &vector2 - &vector1;
        assert!(vector2_minus_vector1.is_equal_to(&RayTracerTuple::new_vector(-2.0, -4.0, -6.0)));
        assert!(vector2_minus_vector1.is_vector());

        let vector1_minus_point1 = &vector1 - &point1;
        assert!(vector1_minus_point1.is_equal_to(&RayTracerTuple::new_point(2.0, 4.0, 6.0)));
        assert_eq!(-1, vector1_minus_point1.w); // a weird reality
    }

    #[test]
    fn tuple_neg_refs() {
        let tuple = RayTracerTuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4,
        };
        let neg_tuple = -&tuple;
        assert!((neg_tuple.x - -1.0).abs() < EPSILON);
        assert!((neg_tuple.y - 2.0).abs() < EPSILON);
        assert!((neg_tuple.z - -3.0).abs() < EPSILON);
        assert_eq!(neg_tuple.w, 4);

        let neg_neg_tuple = -&-&tuple; // reference types are fun :)
        assert!((neg_neg_tuple.x - 1.0).abs() < EPSILON);
        assert!((neg_neg_tuple.y - -2.0).abs() < EPSILON);
        assert!((neg_neg_tuple.z - 3.0).abs() < EPSILON);
        assert_eq!(neg_neg_tuple.w, -4);
    }
}
