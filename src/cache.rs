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

//! This is very simple implementation of cache for any lazy efficient
//! evaluation of a function. The idea is you have a variable of a certain
//! type (T) and a function/closure (F) over that which returns a result
//! of type (R). You want to execute the function only when the result has
//! become obsolete.

pub struct Cache<T, F, R>
where
    F: Fn(&T) -> R,
{
    t: T,
    f: F,
    r: Option<R>,
}

impl<T, F, R> Cache<T, F, R>
where
    F: Fn(&T) -> R,
    R: Clone,
{
    pub fn new(t: T, f: F) -> Self {
        Self { t, f, r: None }
    }

    pub fn update(&mut self, n: T) {
        self.r = None;
        self.t = n;
    }

    pub fn eval(&mut self) -> R {
        if let Some(r) = &self.r {
            r.clone()
        } else {
            let r = (self.f)(&self.t);
            self.r = Some(r.clone());
            r
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cache;

    #[test]
    fn new() {
        let square: fn(&isize) -> isize = |&x| x * x;
        let c = Cache::new(2isize, square);
        assert_eq!(c.r, None);
        assert_eq!(c.t, 2);
    }

    #[test]
    fn update() {
        let square: fn(&isize) -> isize = |&x| x * x;
        let mut c = Cache::new(2isize, square);
        let _ = c.eval();
        assert_ne!(c.r, None);
        c.update(3);
        assert_eq!(c.r, None);
        assert_eq!(c.t, 3);
    }

    #[test]
    fn eval() {
        let square: fn(&isize) -> isize = |&x| x * x;
        let mut c = Cache::new(2isize, square);
        assert_eq!(c.eval(), 4);
        c.update(3);
        assert_eq!(c.eval(), 9);
        assert_eq!(c.eval(), 9);
    }
}
