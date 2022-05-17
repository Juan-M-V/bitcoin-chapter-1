use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct FieldElement {
    pub prime: i64,
    pub num: i64
}

impl FieldElement {
    pub fn new(value: i64, prime: i64) -> FieldElement {
        FieldElement { prime, num: (value % prime) }
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

        FieldElement {prime: self.prime, num: (self.num + y.num) % self.prime}
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, y: FieldElement) -> Self::Output {
        if self.prime != y.prime {
            panic!("Both elements must belong to fields of the same size.")
        }

        let num = (self.num - y.num).rem_euclid(self.prime);
        FieldElement {prime: self.prime, num }
    }
}

#[test]
fn field_elements_can_be_created() {
    let field_element = FieldElement::new(18, 17);

    assert_eq!(FieldElement::new(1, 17), field_element);
}

#[test]
fn field_elements_can_be_added() {
    let x = FieldElement::new(2, 3);
    let y = FieldElement::new(1, 3);

    assert_eq!(FieldElement::new(0, 3), x + y)
}

#[test]
fn field_elements_can_be_substracted() {
    let x = FieldElement::new(2,5);
    let y = FieldElement::new(3,5);

    assert_eq!(FieldElement::new(4,5), x - y)
}

