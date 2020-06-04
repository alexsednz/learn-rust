pub trait A {
    fn a() -> &'static str {
        let res = "A::a";
        println!("{}", res);
        Self::b();
        return res;
    }
    fn b() -> &'static str {
        let res = "A::b";
        println!("{}", res);
        return res;
    }
}

pub struct S;

impl A for S {
    fn b() -> &'static str {
        let res = "S::b";
        println!("{}", res);
        return res;
    }
}

pub struct E;

impl A for E {}

#[cfg(test)]
mod test {
    use super::{A, S, E};

    #[test]
    fn play() {
        // Once a function like b is implemented for a struct like S, there is no way to get to the default implementation of b
        assert_eq!(<S as A>::b(), "S::b");
        assert_eq!(<S as A>::a(), "A::a");

        // assert_eq!(A::b(), "A::b"); // Try uncomment this line and check what error you get
        assert_eq!(E::b(), "A::b");
    }
}