use std::cell::RefCell;

pub trait A {
    fn a(&self) -> String;
}

struct S {
    m: isize,
}

impl A for S {
    fn a(&self) -> String {
        format!("a for S: m = {}", self.m)
    }
}

struct MockS {
    c: RefCell<usize>,
}

impl A for MockS {
    fn a(&self) -> String {
        let mut rc = self.c.borrow_mut();
        *rc += 1;
        format!("a for MockS: c = {}", *rc)
    }
}

pub fn do_a(x: &dyn A) -> String {
    x.a()
}

pub fn do_a_twice(x: &dyn A) -> String {
    x.a();
    x.a()
}

#[cfg(test)]
mod test {
    use super::{S, MockS, do_a, do_a_twice};

    #[test]
    fn mock_gets_replaced() {
        let s = S { m: 5 };
        let mock = MockS { c: Default::default() };
        assert_eq!(do_a(&s), String::from("a for S: m = 5"));
        assert_eq!(do_a(&mock), String::from("a for MockS: c = 1"));
    }

    #[test]
    fn mock_counts_calls() {
        let mock = MockS { c: Default::default() };
        let _ = do_a_twice(&mock);
        assert_eq!(mock.c.into_inner(), 2);
    }
}



