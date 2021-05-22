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

//! Until Rust 1.51 adds support for generic const, we can only get help from macros to avoid
//! duplications where there is a need to implement a trait for the types such as arrays which
//! have constant values in their construct. Another workaround for arrays is looking at slices.

pub trait Fibonacci {
    fn fib(&mut self);
}
macro_rules! impl_fibonacci {
    (1) => {
        impl Fibonacci for [usize; 1] {
            fn fib(&mut self) {
                self[0] = 1;
            }
        }
    };
    (2) => {
        impl Fibonacci for [usize; 2] {
            fn fib(&mut self) {
                self[0] = 1;
                self[1] = 1;
            }
        }
    };
    ($x:literal) => {
        impl Fibonacci for [usize; $x] {
            fn fib(&mut self) {
                self[0] = 1;
                self[1] = 1;
                for i in 2..$x {
                    self[i] = self[i - 1] + self[i - 2];
                }
            }
        }
    };
}

impl_fibonacci!(1);
impl_fibonacci!(2);
impl_fibonacci!(7);

/// ```compile_fail
/// fn foo() -> usize {
///     let mut s = [0usize; 8];
///     s.fib();
///     s[7]
/// }
/// ```
pub fn fibonacci_not_implemented_for_eight_element_array() {}

pub fn fibonacci(s: &mut [usize]) {
    if s.len() > 0 {
        s[0] = 1;
    }
    if s.len() > 1 {
        s[1] = 1;
    }
    for i in 2..s.len() {
        s[i] = s[i - 1] + s[i - 2];
    }
}

#[cfg(test)]
mod test {
    use super::{fibonacci, Fibonacci};

    #[test]
    fn one_element() {
        let mut s = [0usize; 1];
        s.fib();
        assert_eq!(s, [1usize]);
    }

    #[test]
    fn two_elements() {
        let mut s = [0usize; 2];
        s.fib();
        assert_eq!(s, [1usize, 1]);
    }

    #[test]
    fn seven_elements() {
        let mut s = [0usize; 7];
        s.fib();
        assert_eq!(s, [1usize, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn one_element_slice_workaround() {
        let mut s = [0usize; 1];
        fibonacci(&mut s);
        assert_eq!(s, [1usize]);
    }

    #[test]
    fn two_elements_slice_workaround() {
        let mut s = [0usize; 2];
        fibonacci(&mut s);
        assert_eq!(s, [1usize, 1]);
    }

    #[test]
    fn seven_elements_slice_workaround() {
        let mut s = [0usize; 7];
        fibonacci(&mut s);
        assert_eq!(s, [1usize, 1, 2, 3, 5, 8, 13]);
    }
}
