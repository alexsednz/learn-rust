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

//! Here we play with the interior mutability concept. We have a
//! structure called S which implements the trait A. We want to mock
//! this structure and let the mocker implement A differently.The trait
//! A introduces an interface called "a" which is basically a const
//! function in C++ terms. However the mocker is designed to record the
//! number of times the interface a is called. This means when the
//! interface is called, it needs to change itself, which is against the
//! const contract of this interface. Mocker then uses RefCell to
//! circumvent this limitation.

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
    use super::{do_a, do_a_twice, MockS, S};

    #[test]
    fn mock_gets_replaced() {
        let s = S { m: 5 };
        let mock = MockS {
            c: Default::default(),
        };
        assert_eq!(do_a(&s), String::from("a for S: m = 5"));
        assert_eq!(do_a(&mock), String::from("a for MockS: c = 1"));
    }

    #[test]
    fn mock_counts_calls() {
        let mock = MockS {
            c: Default::default(),
        };
        let _ = do_a_twice(&mock);
        assert_eq!(mock.c.into_inner(), 2);
    }
}
