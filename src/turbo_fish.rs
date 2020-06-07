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
// The examples in the test reference_examples are taken from
// https://doc.rust-lang.org/reference/paths.html

pub struct S<T> {
    m: T,
}

impl<T: Copy> S<T> {
    pub fn get(&self) -> T {
        self.m
    }
}

impl S<i32> {
    pub fn new() -> S<i32> {
        Self { m: 0 }
    }
}

impl S<f32> {
    pub fn new() -> S<f32> {
        Self { m: 0.0001 }
    }
}

#[cfg(test)]
mod test {
    use super::S;

    #[test]
    fn rust_infers_type() {
        let s1 = S { m: 12i32 };
        let s2 = S { m: 2.4f32 };
        assert_eq!(s1.get(), 12);
        assert_eq!(s2.get(), 2.4);
    }

    #[test]
    fn specify_type_turbo_fish() {
        let s1 = S::<i32>::new();
        let s2 = S::<f32>::new();
        assert_eq!(s1.get(), 0);
        assert_eq!(s2.get(), 0.0001);
    }

    #[test]
    fn reference_examples() {
        let v1 = (0..10).collect::<Vec<_>>();
        let v2: Vec<i32> = (0..10).collect();
        assert_eq!(v1, v2);
        let v3 = Vec::<u8>::with_capacity(1024);
        let v4: Vec<u8> = Vec::with_capacity(1024);
        assert_eq!(v3.capacity(), v4.capacity());
        assert_eq!(v3, v4);
    }
}
