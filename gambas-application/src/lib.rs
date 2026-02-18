pub fn add_application(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_application(2, 2);
        assert_eq!(result, 4);
    }
}
