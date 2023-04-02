#[cfg(all(not(feature = "other"), feature = "normal"))]
pub const MODE: &str = "NORMAL";
#[cfg(all(not(feature = "normal"), feature = "other"))]
pub const MODE: &str = "OTHER";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
