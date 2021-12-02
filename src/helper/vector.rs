use num;
use std::ops::{Add, Neg, Sub};

/// A vector that implements vector addition, subtraction and negation.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T>
where
    T: num::PrimInt + num::NumCast + Copy + std::fmt::Debug,
{
    pub fn new() -> Self {
        Vector {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> Add for Vector<T>
where
    T: num::PrimInt + num::NumCast + Copy + std::fmt::Debug,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: num::PrimInt + num::NumCast + Copy + std::fmt::Debug,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Neg for Vector<T>
where
    T: num::PrimInt + num::NumCast + Copy + std::fmt::Debug,
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Vector {
            x: T::zero() - self.x,
            y: T::zero() - self.y,
        }
    }
}
