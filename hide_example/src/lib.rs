pub fn gdext_rust_init() {
    println!("gdext_rust_init");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}
