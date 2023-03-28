use std::ops::{Add, Div, Mul, Neg, Sub};
///Rust用の多倍長精度演算プロジェクトrfmです。
///
/// 金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。

use crate::utils::*;


///rfmライブラリにおける整数型の表現です。
///Integer expression in rfm library.

pub struct Integer {
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
            //減算後に
            (true, false) => {
                let result_sub = decimal_sub_kernel(&self.value_array, &rhs.value_array);
                return Integer {
                    negative: !result_sub.1,
                    value_array: result_sub.0,
                };
            }
            //相手がマイナスなので引き算と等価
            (false, true) => {
                let result_sub = decimal_sub_kernel(&self.value_array, &rhs.value_array);
                return Integer {
                    negative: result_sub.1,
                    value_array: result_sub.0,
                };
            }
        }
    }
}

impl Sub for Integer {
    type Output = Integer;
    fn sub(self, rhs: Self) -> Self::Output {
        return self + -rhs;
    }
}
/* 
impl Mul for Integer {
    type Output = Integer;
    fn mul(self, rhs: Self) -> Self::Output {
        
    }
}
*/

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        return Integer {
            negative: !self.negative,
            value_array: self.value_array,
        };
    }
}

