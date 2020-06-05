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

//! "use" unlike "C++ include" is by default very picky. As an example
//! pulling a type to your program is not equal to pulling all the
//! interfaces that such a type implements.

pub trait Name {
    fn set_name(&mut self, n: &str);
    fn name(&self) -> String;
}

pub struct S {
    m: String,
}

impl S {
    pub fn len(&self) -> usize {
        self.m.len()
    }
}

impl Name for S {
    fn set_name(&mut self, n: &str) {
        self.m = String::from(n);
    }

    fn name(&self) -> String {
        self.m.clone()
    }
}

/// ```compile_fail
/// mod another_module {
///     use super::S;
///     pub fn display_name(s: S) {
///         println!("{}", s.name());
///     }
/// }
/// ```
pub fn display_name(s: S) {
    println!("{}", s.name());
}

#[cfg(test)]
mod test {
    use super::S;
    // "use S" would not bring all the trait interfaces that S implements
    // automatically. "use S" gives you access to S and its method "len"
    // but not "name" and "set_name". Comment out the below line and you
    // get "items from traits can only be used if the trait is in scope".
    use super::Name;

    #[test]
    fn trait_interface_call() {
        let mut s = S { m: String::new() };
        let name = "Struct S";
        s.set_name(name);
        assert_eq!(s.name(), String::from(name));
        assert_eq!(s.len(), name.len());
    }
}
