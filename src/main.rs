struct RayTracerTuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: u8,
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
}

fn main() {
    println!("Hello, world!");
}

static EPSILON: f64 = 1e-6;

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
