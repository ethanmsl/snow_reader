//! Look -- I'm the `lib.rs`'s doc string! :)

/// An example function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// An example test
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
