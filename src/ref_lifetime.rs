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
//
// The function "longest" used an example here is taken from
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

//! Every reference has a lifetime associated to it. However, not all
//! lifetimes should be explicitly annotated. Here you can see a few
//! examples of lifetime inference through the tests.

/// ```compile_fail
/// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
///     if x.len() > y.len() {
///         x
///     } else {
///         y
///     }
/// }
/// ```
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod test {
    use super::longest;

    #[test]
    fn static_lifetime() {
        let z;
        {
            let x = "long";
            let y = "short";

            z = longest(x, y);
        }
        assert_eq!(z, "short"); // How come a reference (z) is outliving
                                // a value (y)?

        // Answer: string literals have the type &'static str
    }

    #[test]
    fn scoped_lifetime() {
        let r;
        {
            let x = 5;
            r = &x;

            assert_eq!(*r, 5);
        }
        // assert_eq!(*r, 5); // This will be compile_fail if uncommented
    }
}
