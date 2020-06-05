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

//! If you love the old-style C/C++ enums, you have them in Rust too,
//! but don't forget to cast them to a primitive integer like isize,
//! that's the point that you may initially miss. Have a look at the
//! test below to see how it works.

#[derive(Debug)]
pub enum EnumVal {
    Ent1 = 7,
    Ent2 = 11,
}

#[cfg(test)]
mod test {
    use super::EnumVal;

    #[test]
    fn play() {
        assert_eq!(EnumVal::Ent1 as isize, 7);
        assert_eq!(EnumVal::Ent2 as isize, 11);
    }
}
