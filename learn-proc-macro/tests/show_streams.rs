use learn_proc_macro::show_streams;

#[show_streams]
pub fn invoke1() {}

#[show_streams(bar)]
pub fn invoke2() {}

#[show_streams(multiple => tokens)]
pub fn invoke3() {}

#[show_streams { delimiters }]
pub fn invoke4() {}
