///数学でよく使われるアルゴリズムを収録したモジュール


use crate::num::Integer;
use crate::math_traits::*;

/// 与えられた2つの数値の最小公倍数を求めます
pub fn gcd(lhs:&Integer,rhs:&Integer) -> Integer{
    if lhs == rhs {
        return lhs.clone();
    }

    let mut data = match lhs > rhs {
        true =>lhs.clone(),
        false =>rhs.clone()
    };
    let mut rem = match lhs > rhs {
        true =>rhs.clone(),
        false =>lhs.clone()
    };
    loop {
        let result = data.div_rem(&rem);
        if result.1 == Integer::zero() {
            return rem;
        }
        data = rem;
        rem = result.1;
    }
}


#[cfg(test)]
mod integer_test {
    use crate::{num::Integer, math_traits::FromPrimitiveNumber};
    use super::gcd;


    #[test]
    fn test_gcd(){
        assert_eq!(gcd(&Integer::from_i128(10),&Integer::from_i128(8)),Integer::from_i128(2));
    }


}