#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal, clippy::must_use_candidate)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
