//#![cfg_attr(test, no_std)]

pub fn div(mut lhs: u32, rhs: u32) -> u32 {
    if rhs > lhs {
        return 0;
    }
    let mut quotient = 0;

    for i in (0..32).rev() {
        quotient <<= 1;
        if rhs <= lhs >> i {
            quotient |= 1;
            lhs -= rhs << i;
        }
    }

    quotient
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
    gen_special_case!(134217728, 4096 => 32768);
    gen_special_case!(70, 8 => 8);
    gen_special_case!(9, 3 => 3);
}
