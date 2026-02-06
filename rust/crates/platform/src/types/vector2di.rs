use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2Di {
    pub x: i32,
    pub y: i32,
}

impl Vector2Di {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn length_squared(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(&self) -> f32 {
        let len = self.length_squared() as f32;
        len.sqrt()
    }

    #[inline]
    pub fn distance_to(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        ((dx * dx + dy * dy) as f32).sqrt()
    }
}

impl Mul<f32> for Vector2Di {
    type Output = Vector2Di;

    #[inline]
    fn mul(self, scalar: f32) -> Vector2Di {
        Vector2Di {
            x: self.x * scalar as i32,
            y: self.y * scalar as i32,
        }
    }
}

impl MulAssign<f32> for Vector2Di {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar as i32;
        self.y *= scalar as i32;
    }
}

impl Mul<Vector2Di> for f32 {
    type Output = Vector2Di;

    #[inline]
    fn mul(self, v: Vector2Di) -> Vector2Di {
        v * self
    }
}

impl Add for Vector2Di {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2Di {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
