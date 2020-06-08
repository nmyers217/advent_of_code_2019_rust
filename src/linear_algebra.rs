use overload::overload;
use std::ops;

/// FIXME this is probably a stupid and expensive way to compare floats for equality
fn float_cmp(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < std::f64::EPSILON
}

#[derive(Debug, Clone, Copy)]
pub struct V2f {
    x: f64,
    y: f64,
}

overload!((a: ?V2f) + (b: ?V2f) -> V2f { V2f { x: a.x + b.x, y: a.y + b.y } });
overload!((a: ?V2f) - (b: ?V2f) -> V2f { V2f { x: a.x - b.x, y: a.y - b.y } });
overload!((a: ?V2f) * (b: ?V2f) -> V2f { V2f { x: a.x * b.x, y: a.y * b.y } });
overload!((a: ?V2f) / (b: ?V2f) -> V2f { V2f { x: a.x / b.x, y: a.y / b.y } });
overload!((a: ?V2f) * (b: f64) -> V2f { V2f { x: a.x * b, y: a.y * b } });
overload!((a: ?V2f) / (b: f64) -> V2f { V2f { x: a.x / b, y: a.y / b } });
overload!((a: &mut V2f) += (b: ?V2f) { a.x += b.x; a.y += b.y });
overload!((a: &mut V2f) -= (b: ?V2f) { a.x -= b.x; a.y -= b.y });
overload!((a: &mut V2f) *= (b: ?V2f) { a.x *= b.x; a.y *= b.y });
overload!((a: &mut V2f) /= (b: ?V2f) { a.x /= b.x; a.y /= b.y });
overload!((a: &mut V2f) *= (b: f64) { a.x *= b; a.y *= b });
overload!((a: &mut V2f) /= (b: f64) { a.x /= b; a.y /= b });

impl PartialEq for V2f {
    fn eq(&self, other: &Self) -> bool {
        float_cmp(self.x, other.x) && float_cmp(self.y, other.y)
    }
}

impl V2f {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dot_product(&self, other: V2f) -> f64 {
        let temp = self * other;
        temp.x + temp.y
    }

    fn magnitude_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn normalize(&self) -> V2f {
        self / self.magnitude()
    }

    fn rotate(&self, degrees: f64) -> V2f {
        let rads: f64 = degrees.to_radians();
        let cos: f64 = rads.cos();
        let sin: f64 = rads.sin();
        V2f::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }
}

#[test]
fn can_create_a_vec2d() {
    let v = V2f::new(std::f64::consts::PI, 2.0);
    assert_eq!(v, V2f::new(std::f64::consts::PI, 2.0));
}

#[test]
fn can_get_dot_product() {
    let v1 = V2f::new(1.0, 2.0);
    let v2 = V2f::new(3.0, 4.0);
    assert!(float_cmp(v1.dot_product(v2), 11.0));
}

#[test]
fn can_get_magnitude() {
    assert!(float_cmp(V2f::new(1.0, 1.0).magnitude(), 2.0f64.sqrt()));
}

#[test]
fn can_normalize() {
    assert_eq!(V2f::new(2.0, 0.0).normalize(), V2f::new(1.0, 0.0));
    assert_eq!(V2f::new(0.0, 2.0).normalize(), V2f::new(0.0, 1.0));
    let lol: f64 = 1.0 / 2.0f64.sqrt();
    assert_eq!(V2f::new(2.0, 2.0).normalize(), V2f::new(lol, lol));
}

#[test]
fn can_rotate() {
    assert_eq!(V2f::new(1.0, 0.0).rotate(-90.0), V2f::new(0.0, -1.0));
    assert_eq!(V2f::new(0.0, 1.0).rotate(90.0), V2f::new(-1.0, 0.0));
}
