use crate::utils::*;
///Rust用の多倍長精度演算プロジェクトrfmです。
///
/// 金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。
use std::ops::*;

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

///rfmライブラリにおける有理数型の表現です。
///有理数は2つの整数型を組み合わせた分数で表現されます。
pub struct Rational {
    positive: Integer,
    inverse: Integer,
}

impl Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        add_router(&self,&rhs)
    }
}

impl AddAssign for Integer {
    fn add_assign(&mut self, other: Self) {
        *self = add_router(self,&other);
    }
}

fn add_router(lhs: &Integer, rhs: &Integer) -> Integer {
    match (lhs.sign, rhs.sign) {
        //正の整数同士の加算
        (false, false) => Integer {
            sign: false,
            abs_number: arbitrary_precision_add(&lhs.abs_number, &rhs.abs_number),
        },
        //負数同士の加算なので加算結果を負数にする。
        (true, true) => Integer {
            sign: true,
            abs_number: arbitrary_precision_add(&lhs.abs_number, &rhs.abs_number),
        },
        //両辺に-1を掛けて減算し、最後に結果の符号を反転
        (true, false) => {
            let result_sub = arbitrary_precision_sub(&lhs.abs_number, &rhs.abs_number);
            return Integer {
                sign: !result_sub.1,
                abs_number: result_sub.0,
            };
        }
        //相手がマイナスなので引き算と等価
        (false, true) => {
            let result_sub = arbitrary_precision_sub(&lhs.abs_number, &rhs.abs_number);
            return Integer {
                sign: result_sub.1,
                abs_number: result_sub.0,
            };
        }
    }
}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        return Integer {
            sign: !self.sign,
            abs_number: self.abs_number,
        };
    }
}

impl Sub for Integer {
    type Output = Integer;
    fn sub(self, rhs: Self) -> Self::Output {
        return add_router(&self,&-rhs);
    }
}

impl SubAssign for Integer {
    fn sub_assign(&mut self, rhs: Self) {
        *self = add_router(self,&-rhs);
    }
}

impl Mul for Integer {
    type Output = Integer;
    fn mul(self, rhs: Self) -> Self::Output {
        return Integer {
            sign: self.sign ^ rhs.sign,
            abs_number: arbitrary_precision_mul(&self.abs_number, &rhs.abs_number),
        };
    }
}

impl MulAssign for Integer {
    fn mul_assign(&mut self, rhs: Self) {
        self.sign = self.sign ^ rhs.sign;
        self.abs_number = arbitrary_precision_mul(&self.abs_number, &rhs.abs_number);
    }
}

impl PartialOrd for Integer {
    fn ge(&self, other: &Self) -> bool {
        
    }
    fn gt(&self, other: &Self) -> bool {
        
    }
    fn le(&self, other: &Self) -> bool {
        
    }
    fn lt(&self, other: &Self) -> bool {
        
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        if self.abs_number == other.abs_number {
            return !(self.sign ^ other.sign) && self.abs_number== vec![0u64] 
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(&other);
    }
}


impl Div for Integer {
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        todo!("Not Impremented!!!");
    }
}

