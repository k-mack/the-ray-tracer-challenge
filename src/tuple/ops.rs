use super::Tuple;
use std::ops::{Add, Div, Mul, Neg, Sub};

//
// Implement the `Add` trait for a tuple.
//

impl Add for Tuple {
    type Output = Tuple;

    /// Add two tuples, consuming both and returning a new tuple.
    fn add(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Add<&Tuple> for Tuple {
    type Output = Tuple;

    /// Add a reference tuple to a tuple, consuming the left-hand-side tuple, borrowing the right-hand-side tuple, and returning a new tuple.
    fn add(self, rhs: &Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

//
// Implement the `Add` trait for a tuple reference.
//

impl Add<Tuple> for &Tuple {
    type Output = Tuple;

    /// Add a tuple to a tuple reference, borrowing the left-hand-side tuple, consuming the right-hand-side tuple, and returning a new tuple.
    fn add(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// For any tuple reference with lifetime `a`, implement `Add` for it such that it can be added with another tuple reference with a different lifetime `b`.
// We want to implement this trait for reference tuples because we want to be able to use the operands afterwards
// (i.e., we do not want the `add` function to own the operands).
impl<'a, 'b> Add<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    /// Add two tuple references, borrowing both and returning a new tuple.
    fn add(self, rhs: &'b Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

//
// Implement the `Sub` trait for a tuple.
//

impl Sub for Tuple {
    type Output = Tuple;

    /// Subtract two tuples, consuming both and returning a new tuple.
    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub<&Tuple> for Tuple {
    type Output = Tuple;

    /// Subtract a reference tuple from a tuple, consuming the left-hand-side tuple, borrowing the right-hand-side tuple, and returning a new tuple.
    fn sub(self, rhs: &Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

//
// Implement the `Sub` trait for a tuple reference.
//

impl Sub<Tuple> for &Tuple {
    type Output = Tuple;

    /// Subtract a tuple from a tuple reference, borrowing the left-hand-side tuple, consuming the right-hand-side tuple, and returning a new tuple.
    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// For any tuple reference with lifetime `a`, implement `Sub` for it such that it can be added with another tuple reference with a different lifetime `b`.
// We want to implement this trait for reference tuples because we want to be able to use the operands afterwards
// (i.e., we do not want the `sub` function to own the operands).
impl<'a, 'b> Sub<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    /// Add two tuple references, borrowing both and returning a new tuple.
    fn sub(self, rhs: &'b Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

//
// Implement the `Neg` trait for a tuple.
//

impl Neg for Tuple {
    type Output = Tuple;

    /// Negate tuple, consuming the tuple and returning a new tuple.
    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

//
// Implement the `Neg` trait for a tuple reference.
//

impl Neg for &Tuple {
    type Output = Tuple;

    /// Negate tuple reference, borrowing the tuple reference and returning a new tuple.
    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

//
// Implement the `Mul` trait for a tuple for it to be multiplied by an f64.
//

impl Mul<f64> for Tuple {
    type Output = Tuple;

    /// Multiply a tuple by an f64, consuming the left-hand-side tuple, consuming the right-hand-side tuple, and returning a new tuple.
    fn mul(self, rhs: f64) -> Tuple {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

//
// Implement the `Mul` trait for a tuple reference for it to be multiplied by an f64.
//

impl Mul<f64> for &Tuple {
    type Output = Tuple;

    /// Multiply a tuple reference by an f64, borrowing the left-hand-side tuple, consuming the right-hand-side tuple, and returning a new tuple.
    fn mul(self, rhs: f64) -> Tuple {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

///
// Implement the `Div` trait for a tuple for it to be divided by an f64.
//

impl Div<f64> for Tuple {
    type Output = Tuple;

    /// Divide a tuple by an f64, consuming the left-hand-side tuple, consuming the right-hand-side f64, and returning a new tuple.
    fn div(self, rhs: f64) -> Tuple {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Div<f64> for &Tuple {
    type Output = Tuple;

    /// Divide a tuple reference by an f64, borrowing the left-hand-side tuple, consuming the right-hand-side f64, and returning a new tuple.
    fn div(self, rhs: f64) -> Tuple {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::EPSILON;
    use super::*;

    #[test]
    fn tuple_new_point() {
        let tuple = Tuple::new_point(4.3, -4.2, 3.1);
        assert!((tuple.x - 4.3).abs() < EPSILON);
        assert!((tuple.y - -4.2).abs() < EPSILON);
        assert!((tuple.z - 3.1).abs() < EPSILON);
        assert!((tuple.w - 1.0).abs() < EPSILON);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_new_vector() {
        let tuple = Tuple::new_vector(4.3, -4.2, 3.1);
        assert!((tuple.x - 4.3).abs() < EPSILON);
        assert!((tuple.y - -4.2).abs() < EPSILON);
        assert!((tuple.z - 3.1).abs() < EPSILON);
        assert!(tuple.w.abs() < EPSILON);
        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }

    #[test]
    fn tuple_is_equal_to() {
        let point = Tuple::new_point(4.3, -4.2, 3.1);
        let vector = Tuple::new_vector(4.3, -4.2, 3.1);
        assert!(point.is_equal_to(&vector));
        assert!(vector.is_equal_to(&point));

        let not_quite_different = Tuple::new_point(4.3 + 1e-7, -4.2, 3.1);
        assert!(point.is_equal_to(&not_quite_different));

        let barely_different = Tuple::new_point(4.3 + EPSILON, -4.2, 3.1);
        assert!(!point.is_equal_to(&barely_different));
    }

    #[test]
    fn tuple_add() {
        let point1 = Tuple::new_point(3.0, -2.0, 5.0);
        let vector1 = Tuple::new_vector(-2.0, 3.0, 1.0);

        let point1_plus_vector1 = point1 + vector1;
        assert!(point1_plus_vector1.is_equal_to(&Tuple::new_point(1.0, 1.0, 6.0)));
        assert!(point1_plus_vector1.is_point());

        // Add tuples
        let point1 = Tuple::new_point(3.0, -2.0, 5.0);
        let point2 = Tuple::new_point(3.0, -2.0, 5.0);
        let point1_plus_point2 = point1 + point2;
        assert!(point1_plus_point2.is_equal_to(&Tuple::new_point(6.0, -4.0, 10.0)));
        assert!((point1_plus_point2.w - 2.0).abs() < EPSILON); // a weird reality

        // Add a reference tuple to a tuple
        let point1 = Tuple::new_point(3.0, -2.0, 5.0);
        let point2 = Tuple::new_point(3.0, -2.0, 5.0);
        let point1_plus_point2 = point1 + &point2;
        assert!(point1_plus_point2.is_equal_to(&Tuple::new_point(6.0, -4.0, 10.0)));
        assert!((point1_plus_point2.w - 2.0).abs() < EPSILON);

        // Add a tuple to a reference tuple
        let point1 = Tuple::new_point(3.0, -2.0, 5.0);
        let point1_plus_point2 = &point1 + point2;
        assert!(point1_plus_point2.is_equal_to(&Tuple::new_point(6.0, -4.0, 10.0)));
        assert!((point1_plus_point2.w - 2.0).abs() < EPSILON);

        // Add two reference tuples
        let point2 = Tuple::new_point(3.0, -2.0, 5.0);
        let point1_plus_point2 = &point1 + &point2;
        assert!(point1_plus_point2.is_equal_to(&Tuple::new_point(6.0, -4.0, 10.0)));
        assert!((point1_plus_point2.w - 2.0).abs() < EPSILON);
    }

    #[test]
    fn tuple_sub() {
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let point2 = Tuple::new_point(5.0, 6.0, 7.0);

        // Subtract tuples
        let point1_minus_point2 = point1 - point2;
        assert!(point1_minus_point2.is_equal_to(&Tuple::new_vector(-2.0, -4.0, -6.0)));
        assert!(point1_minus_point2.is_vector());

        // Subtract a reference tuple from a tuple
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let vector1 = Tuple::new_vector(5.0, 6.0, 7.0);
        let point1_minus_vector1 = point1 - &vector1;
        assert!(point1_minus_vector1.is_equal_to(&Tuple::new_vector(-2.0, -4.0, -6.0)));
        assert!(point1_minus_vector1.is_point());

        // Subtract a tuple from a reference tuple
        let vector2 = Tuple::new_vector(3.0, 2.0, 1.0);
        let vector2_minus_vector1 = &vector2 - vector1;
        assert!(vector2_minus_vector1.is_equal_to(&Tuple::new_vector(-2.0, -4.0, -6.0)));
        assert!(vector2_minus_vector1.is_vector());

        // Subtract two reference tuples
        let point1 = Tuple::new_point(3.0, 2.0, 1.0);
        let vector1 = Tuple::new_vector(5.0, 6.0, 7.0);
        let vector1_minus_point1 = &vector1 - &point1;
        assert!(vector1_minus_point1.is_equal_to(&Tuple::new_point(2.0, 4.0, 6.0)));
        assert!((vector1_minus_point1.w - -1.0).abs() < EPSILON); // a weird reality
    }

    #[test]
    fn tuple_neg() {
        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let neg_tuple = -tuple;
        assert!((neg_tuple.x - -1.0).abs() < EPSILON);
        assert!((neg_tuple.y - 2.0).abs() < EPSILON);
        assert!((neg_tuple.z - -3.0).abs() < EPSILON);
        assert!((neg_tuple.w - 4.0).abs() < EPSILON);

        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let neg_tuple = -&tuple;
        assert!((neg_tuple.x - -1.0).abs() < EPSILON);
        assert!((neg_tuple.y - 2.0).abs() < EPSILON);
        assert!((neg_tuple.z - -3.0).abs() < EPSILON);
        assert!((neg_tuple.w - 4.0).abs() < EPSILON);

        let neg_neg_tuple = -&-&tuple; // reference types are fun :)
        assert!((neg_neg_tuple.x - 1.0).abs() < EPSILON);
        assert!((neg_neg_tuple.y - -2.0).abs() < EPSILON);
        assert!((neg_neg_tuple.z - 3.0).abs() < EPSILON);
        assert!((neg_neg_tuple.w - -4.0).abs() < EPSILON);
    }

    #[test]
    fn tuple_mul() {
        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let tuple_mul = tuple * 3.5;
        assert!((tuple_mul.x - 3.5).abs() < EPSILON);
        assert!((tuple_mul.y - -7.0).abs() < EPSILON);
        assert!((tuple_mul.z - 10.5).abs() < EPSILON);
        assert!((tuple_mul.w - -14.0).abs() < EPSILON);

        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let tuple_mul = &tuple * 0.5;
        assert!((tuple_mul.x - 0.5).abs() < EPSILON);
        assert!((tuple_mul.y - -1.0).abs() < EPSILON);
        assert!((tuple_mul.z - 1.5).abs() < EPSILON);
        assert!((tuple_mul.w - -2.0).abs() < EPSILON);
    }

    #[test]
    fn tuple_div() {
        let tuple = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let tuple_div = &tuple / 2.0;
        assert!((tuple_div.x - 0.5).abs() < EPSILON);
        assert!((tuple_div.y - -1.0).abs() < EPSILON);
        assert!((tuple_div.z - 1.5).abs() < EPSILON);
        assert!((tuple_div.w - -2.0).abs() < EPSILON);

        let tuple_div = tuple / 2.0;
        assert!((tuple_div.x - 0.5).abs() < EPSILON);
        assert!((tuple_div.y - -1.0).abs() < EPSILON);
        assert!((tuple_div.z - 1.5).abs() < EPSILON);
        assert!((tuple_div.w - -2.0).abs() < EPSILON);
    }

    #[test]
    fn tuple_magnitude() {
        let mut tuple = Tuple::new_vector(1.0, 0.0, 0.0);
        assert!((tuple.magnitude() - 1.0).abs() < EPSILON);

        tuple = Tuple::new_vector(0.0, 1.0, 0.0);
        assert!((tuple.magnitude() - 1.0).abs() < EPSILON);

        tuple = Tuple::new_vector(0.0, 0.0, 1.0);
        assert!((tuple.magnitude() - 1.0).abs() < EPSILON);

        tuple = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!((tuple.magnitude() - 14.0_f64.sqrt()).abs() < EPSILON);

        tuple = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert!((tuple.magnitude() - 14.0_f64.sqrt()).abs() < EPSILON);
    }

    #[test]
    fn tuple_normalize() {
        let mut tuple = Tuple::new_vector(4.0, 0.0, 0.0);
        assert!(tuple
            .normalize()
            .is_equal_to(&Tuple::new_vector(1.0, 0.0, 0.0)));

        tuple = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(tuple.normalize().is_equal_to(&Tuple::new_vector(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt()
        )));

        assert!((tuple.normalize().magnitude() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn tuple_dot_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert!((a.dot_product(&b) - 20.0).abs() < EPSILON);
        assert!((b.dot_product(&a) - 20.0).abs() < EPSILON);
    }

    #[test]
    fn tuple_cross_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert!(a
            .cross_product(&b)
            .is_equal_to(&Tuple::new_vector(-1.0, 2.0, -1.0)));
        assert!(b
            .cross_product(&a)
            .is_equal_to(&Tuple::new_vector(1.0, -2.0, 1.0)));
    }
}
