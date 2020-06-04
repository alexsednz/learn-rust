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
        assert_eq!(z, "short"); // How come a reference (z) is outliving a value (y)?

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
