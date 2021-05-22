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

//! A trait with default implementations and some structures which
//! implement the trait but have their own methods and functions with the
//! same signatures as those in the trait. Obviously you should avoid
//! putting yourself in such a messy situation. However, there would be
//! always that one developer before us who will put us in such mess. This
//! may also happen as a collective result of several developers working
//! around the same code. Fully qualified calls are for resolving these
//! cases.

pub trait TikTok {
    fn tik() -> String {
        String::from("default tik")
    }
    fn tok() -> String {
        format!("default tok calls {}", Self::tik())
    }
    fn tik_tok(&mut self) {} // Do nothing by default
}

pub struct FlipFlop {
    b: bool,
}
impl TikTok for FlipFlop {}
impl FlipFlop {
    pub fn tik_tok(&mut self) {
        self.b = !self.b;
    }
}
// TikTok::tik() or TikTok::tok() direct calls both fail.
// You can call functions through types not traits.
/// ```compile_fail
/// TikTok::tik();
/// ```
pub fn tik() {
    FlipFlop::tik();
}

pub struct UpCounter {
    c: isize,
}
impl TikTok for UpCounter {
    fn tik() -> String {
        String::from("tik for UpCounter")
    }
    fn tik_tok(&mut self) {
        self.c += 1;
    }
}
impl UpCounter {
    pub fn tik() -> String {
        String::from("tik of UpCounter")
    }
}

pub struct DownCounter {
    c: isize,
}
impl TikTok for DownCounter {
    fn tik_tok(&mut self) {
        self.c -= 1;
    }
}
impl DownCounter {
    pub fn tik() -> String {
        String::from("tik of DownCounter")
    }
    pub fn tok() -> String {
        String::from("tok of DownCounter")
    }
    pub fn tik_tok(&mut self) {
        self.c -= 2;
    }
}

#[cfg(test)]
mod test {
    use super::{DownCounter, FlipFlop, TikTok, UpCounter};

    #[test]
    fn flip_flop_functions() {
        assert_eq!(FlipFlop::tik(), String::from("default tik"));
        assert_eq!(
            FlipFlop::tok(),
            String::from("default tok calls default tik")
        );
    }

    #[test]
    fn down_counter_functions() {
        assert_eq!(DownCounter::tik(), String::from("tik of DownCounter"));
        assert_eq!(DownCounter::tok(), String::from("tok of DownCounter"));
        assert_eq!(<DownCounter as TikTok>::tik(), String::from("default tik"));
        assert_eq!(
            <DownCounter as TikTok>::tok(),
            String::from("default tok calls default tik")
        );
    }

    #[test]
    fn up_counter_functions() {
        assert_eq!(UpCounter::tik(), String::from("tik of UpCounter"));
        assert_eq!(
            UpCounter::tok(),
            String::from("default tok calls tik for UpCounter")
        );

        // Default tik is not available to UpCounter anymore
        assert_eq!(
            <UpCounter as TikTok>::tik(),
            String::from("tik for UpCounter")
        );
    }

    #[test]
    fn flip_flop_method() {
        let mut t = FlipFlop { b: false };

        t.tik_tok();
        assert_eq!(t.b, true);

        t.tik_tok();
        assert_eq!(t.b, false);

        <FlipFlop as TikTok>::tik_tok(&mut t); // default trait impl does nothing
        assert_eq!(t.b, false);
    }

    #[test]
    fn up_counter_method() {
        let mut s = UpCounter { c: 0 };

        s.tik_tok();
        assert_eq!(s.c, 1);

        <UpCounter as TikTok>::tik_tok(&mut s);
        assert_eq!(s.c, 2);
    }

    #[test]
    fn down_counter_method() {
        let mut e = DownCounter { c: 3 };

        e.tik_tok();
        assert_eq!(e.c, 1);

        <DownCounter as TikTok>::tik_tok(&mut e);
        assert_eq!(e.c, 0);
    }
}
