//! The arrangement of definitions and the test here showcase the rust capability to infer and
//! fetch types by looking ahead to the lines further down.

pub fn init_a(a: &mut A) {
    init_b(&mut a.m);
}

pub fn init_b(b: &mut B) {
    b.m1 = 1;
    b.m2 = String::from("STR");
}

impl Name for A {
    fn name(&self) -> &'static str {
        "Struct A"
    }
}

impl Name for B {
    fn name(&self) -> &'static str {
        "Struct B"
    }
}

#[derive(Default)]
pub struct A {
    m: B,
}

#[derive(Default)]
pub struct B {
    m1: BM1,
    m2: String,
}

pub trait Name {
    fn name(&self) -> &'static str;
}

type BM1 = isize;

#[cfg(test)]
mod test {
    use super::{init_a, init_b, Name};

    #[test]
    fn infer_type() {
        let mut x = Default::default();
        let mut y = Default::default();

        init_b(&mut x);
        init_a(&mut y);

        assert_eq!(String::from(x.name()), String::from("Struct B"));
        assert_eq!(String::from(y.name()), String::from("Struct A"));
    }
}
