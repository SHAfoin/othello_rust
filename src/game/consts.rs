pub const SIZE: usize = 8;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_even() {
        assert_eq!(SIZE % 2, 0);
    }
}
