// This file is part of https://github.com/alexsednz/learn-rust.
//
// learn-rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// learn-rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with learn-rust.  If not, see <https://www.gnu.org/licenses/>.

//! Traits with default implementations and overlapping names. The test
//! shows how we can call each individual implementation and which one
//! gets masked fully. By being masked we mean the implementation
//! wouldn't be reachable through the structure which has apparently
//! implemented the corresponding trait.

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
    use super::{A, E, S};

    #[test]
    fn play() {
        // Once a function like b is implemented for a struct like S, there is
        // no way to get to the default implementation of b
        assert_eq!(<S as A>::b(), "S::b");
        assert_eq!(<S as A>::a(), "A::a");

        // assert_eq!(A::b(), "A::b"); // Try uncomment this line and check the
        // error
        assert_eq!(E::b(), "A::b");
    }
}
