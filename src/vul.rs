///Rust用の多倍長精度演算プロジェクトrfmです。
///
/// 金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。

use std::cmp;
use std::ops::{Add, Div, Mul, Neg, Sub};

///rfmライブラリにおける整数型の表現です。
///Integer expression in rfm library.
/// 
/// # 補足事項
/// 
/// 
pub struct Integer {
    ///
    negative: bool,
    value_array: Vec<u64>,
}

impl Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.negative, rhs.negative) {
            //正の整数同士の加算
            (false, false) => Integer {
                negative: false,
                value_array: decimal_add_kernel(&self.value_array, &rhs.value_array),
            },
            //負数同士の加算なので加算結果を負数にする。
            (true, true) => Integer {
                negative: true,
                value_array: decimal_add_kernel(&self.value_array, &rhs.value_array),
            },
            //相手方が負数なので補数表現を求めて加算で処理する。
            (true, false) => panic!("STUB!"),
            (false, true) => panic!("STUB!"),
        }
    }
}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        return Integer {
            negative: !self.negative,
            value_array: self.value_array,
        };
    }
}

/// 巨大な整数の加算処理(筆算アルゴリズム)
/// Internal decimal adder
/// # Arguments
/// * 'lhs' - 右辺値
/// * 'rhs' - 左辺値
/// 配列を多倍長整数と見做して加算する。
/// なお、この関数がサポートする数は自然数と0のみ。
///この計算量はO(N)である。
///
fn decimal_add_kernel(lhs: &Vec<u64>, rhs: &Vec<u64>) -> Vec<u64> {
    let argsize = cmp::max(lhs.len(), rhs.len());
    let mut array: Vec<u64> = Vec::with_capacity(argsize + 1); //桁上がりの範囲として+1の範囲を予約
    let mut carry: u64 = 0; //桁上がり
    for i_ in 0..argsize {
        let lhs_digit: u64 = *lhs.get(i_).unwrap_or(&0);
        let rhs_digit: u64 = *rhs.get(i_).unwrap_or(&0);
        let result_add: (u64,bool) = lhs_digit.overflowing_add(rhs_digit);//加算処理
        let result_number: (u64,bool) =result_add.0.overflowing_add(carry);//桁上がりを足して桁確定
        carry = 0;
        if true == result_add.1 {
            carry += 1;//加算による桁上がり
        } 
        if true == result_number.1 {
            carry += 1;//桁上がり処理による数字確定字の桁上がり
        }
        array.push(result_number.0);
    }
    if 0 < carry {
        array.push(carry); //最後の桁上がりを処理する。
    }
    return array;
}

#[cfg(test)]
mod dedimal_add_kernel_tests {
    use crate::vul::decimal_add_kernel;

    #[test]
    fn test_1digit_add() {
        assert_eq!(vec![2u64], decimal_add_kernel(&vec![1u64], &vec![1u64]));
    }

    #[test]
    fn test_carry() {
        //桁上がり確認
        assert_eq!(vec![0,1u64], decimal_add_kernel(&vec![1], &vec![u64::MAX]));
        assert_eq!(vec![0,1u64], decimal_add_kernel(&vec![u64::MAX], &vec![1]));
    }

    #[test]
    fn test_carry_multiple() {
        assert_eq!(
            vec![0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 2u64],
            decimal_add_kernel(
                &vec![u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, 1u64],
                &vec![1u64]
            )
        );
    }
}

/// 内部的な引き算の実装
/// Internal substitutor imprements.
///
/// # 引数 Arguments
/// * 'lhs' - 左辺値
/// * 'rhs' - 右辺値

fn internal_substitutor(vec_int: &Vec<u8>, base: u64) -> Vec<u8> {
    //for _i in 0..vec_int.len() {}
    panic!("stub!");
}
