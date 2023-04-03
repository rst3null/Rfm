///Rust用の多倍長精度演算プロジェクトrfmです。
///
/// 金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。

use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::utils::*;


///rfmライブラリにおける整数型の表現です。
///Integer expression in rfm library.
pub struct Integer {
    
    ///整数の絶対値
    ///整数の絶対値を18446744073709551616進数で表記する配列
    ///この配列は絶対値を保持しており、補数表現をしてはならない。
    abs_number: Vec<u64>,

    ///符号管理フラグ
    ///trueのとき、負数となる。
    sign: bool,
}

impl Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            //正の整数同士の加算
            (false, false) => Integer {
                sign: false,
                abs_number: arbitrary_precision_add(&self.abs_number, &rhs.abs_number),
            },
            //負数同士の加算なので加算結果を負数にする。
            (true, true) => Integer {
                sign: true,
                abs_number: arbitrary_precision_add(&self.abs_number, &rhs.abs_number),
            },
            //両辺に-1を掛けて減算し、最後に結果の符号を反転
            (true, false) => {
                let result_sub = arbitrary_precision_sub(&self.abs_number, &rhs.abs_number);
                return Integer {
                    sign: !result_sub.1,
                    abs_number: result_sub.0,
                };
            }
            //相手がマイナスなので引き算と等価
            (false, true) => {
                let result_sub = arbitrary_precision_sub(&self.abs_number, &rhs.abs_number);
                return Integer {
                    sign: result_sub.1,
                    abs_number: result_sub.0,
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
            sign: !self.sign,
            abs_number: self.abs_number,
        };
    }
}

