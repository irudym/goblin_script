use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn distance_to(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > f32::EPSILON {
            self * (1.0 / len)
        } else {
            Vector2D::ZERO
        }
    }

    /// Moves this vector toward `to` by `delta`.
    /// Will not overshoot the target.
    #[inline]
    pub fn move_toward(self, to: Self, delta: f32) -> Self {
        let diff = to - self;
        let dist = diff.length();

        if dist <= delta || dist <= f32::EPSILON {
            to
        } else {
            self + diff * (delta / dist)
        }
    }

    #[inline]
    pub fn approx_eq(self, other: &Self) -> bool {
        (self.x - other.x).abs() <= f32::EPSILON && (self.y - other.y).abs() <= f32::EPSILON
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    #[inline]
    fn mul(self, scalar: f32) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vector2D {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Mul<Vector2D> for f32 {
    type Output = Vector2D;

    #[inline]
    fn mul(self, v: Vector2D) -> Vector2D {
        v * self
    }
}

impl Add for Vector2D {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
