/// Epsilon used for floating-point comparisons
const EPSILON: f64 = 1e-6;

pub mod ops;

#[derive(Debug, Default)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    /// Create a point tuple.
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    /// Create a vector tuple
    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Test if the tuple is a point.
    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < EPSILON
    }

    /// Test if the tuple is a vector.
    pub fn is_vector(&self) -> bool {
        self.w.abs() < EPSILON
    }

    /// Test if this tuple is equal to another.
    /// Note that this only considers the cartesian coordinates of the two tuples.
    pub fn is_equal_to(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }

    /// Compute the magnitude of the tuple.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Return a new tuple that is this tuple normalized.
    pub fn normalize(&self) -> Tuple {
        self / self.magnitude()
    }

    /// Compute the dot product of the tuple.
    pub fn dot_product(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Compute the cross product between this vector and another.
    pub fn cross_product(&self, other: &Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
