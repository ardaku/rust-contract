
/// Internally, this just creates it in documentation with the `compile_fail` flag., and everything
/// prefixed with # (to hide the documentation.
///
/// It's also in a `cfg(doctest)` attr, so it's only compiled when running `cargo test --doc`.
macro_rules! assert_compile_fail {
    ($($tt:tt)*) => {
        #[cfg(doctest)]
        #[doc = "# ```compile_fail"]
        #[doc = $crate::stringify!($($tt)*)]
        #[doc = "# ```"]
        #[allow(dead_code)]
        pub const _: () = ();
    }
}

// /// Should produce a compile error
// /// ```compile_fail
// /// fn placeholder_param_name() {
// ///     #[contract("_ = vary -> vary")]
// ///     fn test_fn(_: u32) -> u32 {
// ///         0
// ///     }
// /// }
// /// ```
// #[cfg(doctest)]
// #[allow(dead_code)]
// pub struct PlaceholderParamNameDoctest;

macro_rules! assert_eq_list {
    ($left:expr, $right:expr) => {
        assert_eq!($left.len(), $right.len());
        for (l, r) in $left.iter().zip($right.iter()) {
            assert_eq!(l, r);
        }
    };
}

#[cfg(test)]
mod tests {
    use rust_contract::contract;

    // === Panicing/erroring tests ===

    assert_compile_fail!(
        #[contract("x = vary -> vary")]
        fn test_fn(x: u32) -> u32 {
            x
        }
    );

    // === Non-panicing tests ===

    #[test]
    fn vary_vary() {
        #[contract("x = vary -> vary")]
        pub fn identity(x: u32) -> u32 {
            x
        }
        let left: Vec<u32> = vec![1, 2, 3, 4, 5];
        let right: Vec<u32> = left.iter().map(|x| identity(*x)).collect::<Vec<_>>();
        assert_eq_list!(left, right);
        // It can be assumed that it works at this point
    }
}
