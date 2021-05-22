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

//! Here we test different aspect of references vs values when moved or
//! copied across. We also test dereference the coercion feature.
//! Finally borrowing rvalues is showcased through a test.

#[cfg(test)]
mod test {
    #[test]
    fn value_vs_ref() {
        #[derive(Clone)]
        struct Point {
            x: isize,
            y: isize,
        }

        impl Point {
            // The second argument is a value for the sake of the test. Otherwise a
            // reference was a much better choice See how both self and p are
            // used the same while self is a reference and p is a value
            fn sqr_dst(&self, p: Self) -> isize {
                (self.x - p.x).pow(2) + (self.y - p.y).pow(2)
            }
        }

        fn sqr_distance(p1: &Point, p2: &Point) -> isize {
            // In the following line of code we couldn't pass p2 directly as the
            // reference behavior in Rust is not like C++ We couldn't do
            // *p2 either as Copy is not implemented for Point
            p1.sqr_dst(p2.clone())
        }

        let mut p1 = Point { x: 0, y: -1 };
        let p2 = Point { x: 1, y: 2 };

        let p3 = &mut p1;
        p3.x = -1; // We can do this, as p3 is a mutable ref of p1 and p1 is mutable.

        let mut p4 = p2.clone();
        // If we had ```p4 = p2;``` in the previous line, we couldn't use p2 below
        p4.x = p2.x + 2;

        assert_eq!(sqr_distance(p3, &p4), 25); // p4 should be borrowed
    }

    #[test]
    fn deref_coercion() {
        fn str_len(s: &str) -> usize {
            s.len()
        }
        let x_str = Box::<String>::new(String::from("Hello"));
        assert_eq!(str_len(&x_str), 5);
    }

    #[test]
    fn borrow_rvalue() {
        // The following lines are taken from https://stackoverflow.com/a/51346943/2746280
        fn get_u32() -> u32 {
            3
        }
        let x = &mut get_u32();
        *x = 20;
        assert_eq!(*x, 20);
    }
}
