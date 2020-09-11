use nodiv::div;
use proptest::prelude::*;

proptest! {
    #[test]
    fn matches_native_division(lhs in 0u32.., rhs in 1u32..) {
        assert_eq!(div(lhs, rhs), lhs / rhs)
    }

    #[test]
    fn zero_by_anything_is_zero(rhs in 1u32..) {
        assert_eq!(div(0, rhs), 0)
    }
}
