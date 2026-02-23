#![allow(dead_code)]
mod engine;

pub fn add_domain(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_domain(2, 2);
        assert_eq!(result, 4);
    }
}
