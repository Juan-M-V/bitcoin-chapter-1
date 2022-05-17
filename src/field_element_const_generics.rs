// const generics
// usize

use std::ops::{Add, Mul, Neg, Sub};

pub type Z2 = FieldElement2<2>; // 0, 1
pub type Z5 = FieldElement2<5>; // 0, 1,2,3,4

// 1 /* Z2 */ + 5 /* Z16 */

#[derive(Debug, PartialEq, Eq)]
pub struct FieldElement2<const P: usize>(pub i64);

impl<const P: usize> Add for FieldElement2<P> {
    type Output = FieldElement2<P>;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % P as i64)
    }
}

impl<const P: usize> Mul for FieldElement2<P> {
    type Output = FieldElement2<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % P as i64)
    }
}

impl<const P: usize> Neg for FieldElement2<P> {
    type Output = FieldElement2<P>;

    fn neg(self) -> Self::Output {
        Self((-self.0) % P as i64)
    }
}

impl<const P: usize> Sub for FieldElement2<P> {
    type Output = FieldElement2<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 - rhs.0) % P as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_z2_values() {
        let x: Z2 = FieldElement2(1);
        let y: Z2 = FieldElement2(1);
        assert_eq!(FieldElement2(0), x + y)
    }

    #[test]
    fn can_multiply_z2_values() {
        let x: Z2 = FieldElement2(1);
        let y: Z2 = FieldElement2(1);
        assert_eq!(FieldElement2(1), x * y)
    }
}
