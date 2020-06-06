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

//! Here we evaluate the access scope of struct types. Every rust type,
//! including structs, make the following different relationships with the
//! world around it: Same Scope, Parent Scope, Child Scope, Sister Scope.


#![allow(dead_code)]

mod a {
    struct S {
        m: isize,
    }
    /// It tests that inside the same module, a private struct and its
    /// private member variable are accessible
    fn inc(s: &mut S) {
        s.m += 1;
    }

    pub struct Y {
        m: isize,
    }
    impl Y {
        pub fn get(&self) -> isize {
            self.m
        }
    }

    // A true developer should hate the following type :)
    pub struct Z {
        pub m: isize,
    }
}

struct X {
    m: isize,
}

/// It tests that we have don't have access to S (child's private struct),
/// though the access to X is ok as expected
/// ```compile_fail
/// fn sub(s1: &a::S, s2: &a::S) -> isize {
///     s1.m - s2.m
/// }
/// ```
fn sub(x1: &X, x2: &X) -> isize {
    x1.m - x2.m
}

mod b {
    use super::X;
    /// I tests that we have access to X (parent's private struct) but not
    /// S (sister's private struct)
    /// ```compile_fail
    /// use super::a::S;
    /// fn dec(s: &mut S) {
    ///     s.m -= 1;
    /// }
    /// ```
    fn dec(x: &mut X) {
        // "X" and its member are even accessible inside a child module
        x.m -= 1;
    }

    use super::a::Y;
    /// It tests that we have access to Y (sister's public struct) but not
    /// its member variable.
    /// ```compile_fail
    /// fn comp(y1: &Y, y2: &Y) -> bool {
    ///     y1.m < y2.m
    /// }
    /// ```
    fn comp(y1: &Y, y2: &Y) -> bool {
        y1.get() < y2.get()
    }

    /// It tests that we have access to both Z (sister's public struct)
    /// and its pubic member variable
    use super::a::Z;
    fn add(z1: &Z, z2: &Z) -> isize {
        z1.m + z2.m
    }
}
