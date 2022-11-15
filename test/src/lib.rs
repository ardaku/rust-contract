#[cfg(test)]
mod tests {
    use rust_contract::contract;

    // === Panicing tests ===

    #[test]
    #[should_panic]
    fn placeholder_param_name() {
        #[contract("_ = vary -> vary")]
        fn test_fn(_: u32) -> u32 {
            0
        }
    }

    // === Non-panicing tests ===

    #[test]
    fn vary_vary() {
        #[contract("x = vary -> vary")]
        pub fn identity(x: u32) -> u32 {
            x
        }
        assert_eq!(identity(0), 0);
        assert_eq!(identity(1), 1);
        assert_eq!(identity(2), 2);
        // It can be assumed that it works at this point
    }
}
