use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct FieldElement {
    pub prime: i64,
    pub num: i64,
}

impl FieldElement {
    pub fn new(value: i64, prime: i64) -> FieldElement {
        FieldElement {
            prime,
            num: value.rem_euclid(prime),
        }
    }

    pub fn pow(self, exponent: u32) -> FieldElement {
        let num = self.num.pow(exponent).rem_euclid(self.prime);
        FieldElement {
            prime: self.prime,
            num,
        }
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    // x + y
    // self = x
    // rhs = y (right hand side)
    fn add(self, y: FieldElement) -> Self::Output {
        if self.prime != y.prime {
            panic!("Both elements must belong to fields of the same size.")
        }

        let num = (self.num + y.num).rem_euclid(self.prime);
        FieldElement {
            prime: self.prime,
            num,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, y: FieldElement) -> Self::Output {
        if self.prime != y.prime {
            panic!("Both elements must belong to fields of the same size.")
        }

        let num = (self.num - y.num).rem_euclid(self.prime);
        FieldElement {
            prime: self.prime,
            num,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, y: FieldElement) -> Self::Output {
        if self.prime != y.prime {
            panic!("Both elements must belong to fields of the same size.")
        }

        let num = (self.num * y.num).rem_euclid(self.prime);
        FieldElement {
            prime: self.prime,
            num,
        }
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, y: FieldElement) -> Self::Output {
        if self.prime != y.prime {
            panic!("Both elements must belong to fields of the same size.")
        }
        // x / y == x * y^-1 == x * y^(p - 2)
        let num = self.num * (y.pow((self.prime - 2) as u32)).num;
        FieldElement {
            prime: self.prime,
            num: num.rem_euclid(self.prime),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn field_elements_can_be_created() {
        let field_element = FieldElement::new(18, 17);

        assert_eq!(FieldElement::new(1, 17), field_element);
    }

    #[test]
    #[should_panic]
    fn add_from_different_fields() {
        let x = FieldElement::new(2, 4);
        let y = FieldElement::new(1, 3);

        let _ = x + y;
    }

    #[test]
    fn field_elements_can_be_added() {
        let x = FieldElement::new(2, 3);
        let y = FieldElement::new(1, 3);

        assert_eq!(FieldElement::new(0, 3), x + y)
    }

    #[test]
    fn negative_field_elements_can_be_added() {
        let x = FieldElement::new(-2, 3);
        let y = FieldElement::new(1, 3);

        assert_eq!(FieldElement::new(2, 3), x + y)
    }

    #[test]
    fn field_elements_can_be_substracted() {
        let x = FieldElement::new(2, 5);
        let y = FieldElement::new(3, 5);

        assert_eq!(FieldElement::new(4, 5), x - y)
    }

    #[test]
    fn field_elements_can_be_multiplied() {
        let x = FieldElement::new(74, 5);
        let y = FieldElement::new(2, 5);

        assert_eq!(FieldElement::new(3, 5), x * y)
    }

    #[test]
    fn negative_field_elements_can_be_multiplied() {
        let x = FieldElement::new(74, 5);
        let y = FieldElement::new(-2, 5);

        assert_eq!(FieldElement::new(2, 5), x * y)
    }

    #[test]
    fn field_elements_have_powers() {
        let x = FieldElement::new(3, 13);

        assert_eq!(FieldElement::new(1, 13), x.pow(3))
    }

    #[test]
    fn pow_0_works() {
        let x = FieldElement::new(3, 13);

        assert_eq!(FieldElement::new(1, 13), x.pow(0))
    }

    #[test]
    fn field_elements_can_be_divided() {
        let x = FieldElement::new(2, 19);
        let y = FieldElement::new(7, 19);

        assert_eq!(FieldElement::new(3, 19), x / y)
    }
}
