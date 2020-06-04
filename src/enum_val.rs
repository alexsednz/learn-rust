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