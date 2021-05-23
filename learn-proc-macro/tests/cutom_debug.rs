#![allow(dead_code)]

use learn_proc_macro::CustomDebug;

#[derive(CustomDebug)]
struct S1 {
    i: isize,
}

#[derive(Debug)]
struct S2 {
    i: isize,
}

#[test]
fn custom_debug() {
    let s1 = S1 { i: -2 };
    let s2 = S2 { i: -2 };
    let f = format!("CustomDebug {:?} vs Debug {:?}", s1, s2);

    assert_eq!(f, String::from("CustomDebug S1 vs Debug S2 { i: -2 }"));
}
