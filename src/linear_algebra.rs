use ordered_float::OrderedFloat;
use overload::overload;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V2f {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
}

overload!((a: ?V2f) + (b: ?V2f) -> V2f {
    V2f {
        x: OrderedFloat::from(a.x.into_inner() + b.x.into_inner()),
        y: OrderedFloat::from(a.y.into_inner() + b.y.into_inner())
    }
});
overload!((a: ?V2f) - (b: ?V2f) -> V2f {
    V2f {
        x: OrderedFloat::from(a.x.into_inner() - b.x.into_inner()),
        y: OrderedFloat::from(a.y.into_inner() - b.y.into_inner())
    }
});
overload!((a: ?V2f) * (b: ?V2f) -> V2f {
    V2f {
        x: OrderedFloat::from(a.x.into_inner() * b.x.into_inner()),
        y: OrderedFloat::from(a.y.into_inner() * b.y.into_inner())
    }
});
overload!((a: ?V2f) / (b: ?V2f) -> V2f {
    V2f {
        x: OrderedFloat::from(a.x.into_inner() / b.x.into_inner()),
        y: OrderedFloat::from(a.y.into_inner() / b.y.into_inner())
    }
});
overload!((a: ?V2f) * (b: f64) -> V2f {
    V2f {
        x: OrderedFloat::from(a.x.into_inner() * b),
        y: OrderedFloat::from(a.y.into_inner() * b)
    }
});
overload!((a: ?V2f) / (b: f64) -> V2f {
    V2f {
        x: OrderedFloat::from(a.x.into_inner() / b),
        y: OrderedFloat::from(a.y.into_inner() / b)
    }
});

overload!((a: &mut V2f) += (b: ?V2f) {
    a.x = OrderedFloat::from(a.x.into_inner() + b.x.into_inner());
    a.y = OrderedFloat::from(a.y.into_inner() + b.y.into_inner());
});
overload!((a: &mut V2f) -= (b: ?V2f) {
    a.x = OrderedFloat::from(a.x.into_inner() - b.x.into_inner());
    a.y = OrderedFloat::from(a.y.into_inner() - b.y.into_inner());
});
overload!((a: &mut V2f) *= (b: ?V2f) {
    a.x = OrderedFloat::from(a.x.into_inner() * b.x.into_inner());
    a.y = OrderedFloat::from(a.y.into_inner() * b.y.into_inner());
});
overload!((a: &mut V2f) /= (b: ?V2f) {
    a.x = OrderedFloat::from(a.x.into_inner() / b.x.into_inner());
    a.y = OrderedFloat::from(a.y.into_inner() / b.y.into_inner());
});
overload!((a: &mut V2f) *= (b: f64) {
    a.x = OrderedFloat::from(a.x.into_inner() * b);
    a.y = OrderedFloat::from(a.y.into_inner() * b);
});
overload!((a: &mut V2f) /= (b: f64) {
    a.x = OrderedFloat::from(a.x.into_inner() / b);
    a.y = OrderedFloat::from(a.y.into_inner() / b);
});

impl V2f {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: OrderedFloat::from(x),
            y: OrderedFloat::from(y),
        }
    }

    pub fn dot_product(&self, other: V2f) -> f64 {
        let temp = self * other;
        temp.x.into_inner() + temp.y.into_inner()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> V2f {
        self / self.magnitude()
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
    assert_eq!(
        OrderedFloat::from(v1.dot_product(v2)),
        OrderedFloat::from(11.0)
    );
}

#[test]
fn can_get_magnitude() {
    assert_eq!(
        OrderedFloat::from(V2f::new(1.0, 1.0).magnitude()),
        OrderedFloat::from(2.0f64.sqrt())
    );
}

#[test]
fn can_normalize() {
    assert_eq!(V2f::new(2.0, 0.0).normalize(), V2f::new(1.0, 0.0));
    assert_eq!(V2f::new(0.0, 2.0).normalize(), V2f::new(0.0, 1.0));
    let lol: f64 = 1.0 / 2.0f64.sqrt();
    assert_eq!(V2f::new(2.0, 2.0).normalize(), V2f::new(lol, lol));
}
