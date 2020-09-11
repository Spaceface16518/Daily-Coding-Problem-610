//#![cfg_attr(test, no_std)]

pub fn div(lhs: u32, rhs: u32) -> u32 {
    // find the difference between the most significant bits of the two numbers
    let misalign = if let Some(diff) = dbg!(rhs.leading_zeros()).checked_sub(dbg!(lhs.leading_zeros())) {
        diff
    } else {
        // if the denominator is greater than the numerator, return 0
        return 0;
    };
    let mut n = dbg!(lhs as u64);
    let d = dbg!((rhs as u64) << dbg!(misalign) as u64);

    let mut quotient = 1;

    for _ in 0..32 {
        if n >= d {
            quotient = quotient | 1; // set lsb to 1
            quotient <<= 1;
            n -= d;
        }
        n <<= 1;
    }

    quotient >> misalign
}

#[cfg(test)]
mod special_cases {
    use super::div;
    use paste::paste;
    use static_assertions::const_assert_eq;

    macro_rules! gen_special_case {
        ($lhs:expr , $rhs:expr => $ans:expr) => {
            paste! {
                #[test]
                fn [<test_ $lhs _div_ $rhs>]() {
                    const_assert_eq!($lhs / $rhs, $ans);
                    assert_eq!(div($lhs, $rhs), $ans)
                }
            }
        };
    }

    gen_special_case!(0, 1 => 0);
    //gen_special_case!(134217728, 4096 => 32768);
    gen_special_case!(70, 8 => 8);
    gen_special_case!(9, 3 => 3);
}
