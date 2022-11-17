
/// Internally, this just creates it in documentation with the `compile_fail` flag.
///
/// Syntax:
/// `assert_compile_fail!` followed by a block of code, and then a `;` and the name of the test
#[macro_export]
macro_rules! assert_compile_fail {
    ($name:ident, $($tt:tt)*) => {
        #[cfg(doctest)]
        #[doc = "```compile_fail"]
        #[doc = std::stringify!($($tt)*)]
        #[doc = "```"]
        #[allow(dead_code)]
        pub const $name: () = ();
    }
}

#[macro_export]
macro_rules! assert_eq_list {
    ($left:expr, $right:expr) => {
        assert_eq!($left.len(), $right.len());
        for (l, r) in $left.iter().zip($right.iter()) {
            assert_eq!(l, r);
        }
    };
}

/// Should run on cfg(doctest) and cfg(test)
#[cfg(any(doctest, test))]
mod tests {
    use rust_contract::contract;

    // === Panicing/erroring tests ===

    assert_compile_fail!(
        test_placeholder_param,
        #[contract("_ = vary -> vary")]
        fn test_fn(_: u32) -> u32 {
            0
        }
    );

    assert_compile_fail!(
        test_default_output_type,
        #[contract("x = vary -> ()")]
        fn test_fn(_x: u32) {}
    );

    assert_compile_fail!(
        test_unit_output_type,
        #[contract("x = vary -> ()")]
        fn test_fn(_x: u32) -> () {}
    );

    assert_compile_fail!(
        test_no_contracts,
        #[contract()]
        fn test_fn(x: u32) -> u32 {
            x
        }
    );

    // === Non-panicing tests ===

    // Ignore the IntelliJ error here, it works fine
    //noinspection RsUnresolvedReference
    #[test]
    fn test_vary_vary() {
        #[contract("x = vary -> vary")]
        pub fn test_fn(x: u32) -> u32 {
            x
        }
        let left: Vec<u32> = (0..1000).map(test_fn).collect();
        let right: Vec<u32> = left.iter().map(|x| test_fn(*x)).collect::<Vec<_>>();
        assert_eq_list!(left, right);
        // It can be assumed that it works at this point
    }
}
