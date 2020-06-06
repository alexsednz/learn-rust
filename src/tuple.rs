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

//! Here we test the construction and deconstruction of structs, tuple
//! structs and tuples

pub struct S<T> {
    m: T,
}

pub struct D<T, U>(S<(T, U)>);

pub enum E {
    First(isize, f32),
    Second { m1: isize, m2: f32 },
    Third(D<isize, f32>),
    None,
}

impl From<E> for String {
    fn from(e: E) -> Self {
        // The following "match" helps us in deconstructing an enum entry to its
        // building blocks
        match e {
            E::First(i, f) => format!("First({}, {:.1})", i, f),
            E::Second { m1: i, m2: f } => format!("Second({}, {:.1})", i, f),
            E::Third(D(S { m: (i, f) })) => format!("Third({}, {:.1})", i, f),
            E::None => format!("None"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{D, E, S};

    #[test]
    fn tuple_struct() {
        let i = 5isize;
        let f = 4.3f32;

        let e1 = E::First(i, f);
        let e2 = E::Second { m1: i, m2: f };
        let e3 = E::Third(D(S { m: (i, f) }));

        assert_eq!(String::from(e1), String::from("First(5, 4.3)"));
        assert_eq!(String::from(e2), String::from("Second(5, 4.3)"));
        assert_eq!(String::from(e3), String::from("Third(5, 4.3)"));
    }
}
