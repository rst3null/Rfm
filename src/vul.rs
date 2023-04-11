use crate::arithmetic_util::*;
///Rust用の多倍長精度演算プロジェクトrfmです。
///
/// 金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。
use std::ops::*;
use std::cmp::*;

///rfmライブラリにおける整数型の表現です。
///Integer expression in rfm library.
#[derive(Debug,PartialEq,Eq,PartialOrd,Clone)]
pub struct Integer {
    ///整数の絶対値
    ///整数の絶対値を18446744073709551616進数で表記する配列
    ///この配列は絶対値を保持しており、補数表現をしてはならない。
    abs_number: Vec<u64>,

    ///符号管理フラグ
    ///trueのとき、負数となる。
    sign: bool,
}

impl Integer {

    fn with_u64_value(value:u64) -> Integer{
        return Integer{abs_number:vec![value],sign:false};
    }
    fn with_i64_value(value:i64) -> Integer{
        return Integer{abs_number:vec![value.abs() as u64],sign:0>value};
    }
}

impl Add for &Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        add_router(&self,&rhs)
    }
}

impl Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        return &self + &rhs;
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

impl Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        return Integer {
            sign: !self.sign,
            abs_number: self.abs_number.clone(),
        };
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

impl Sub for &Integer {
    type Output = Integer;
    fn sub(self, rhs: Self) -> Self::Output {
        return add_router(&self,&-rhs);
    }
}

impl Sub for Integer {
    type Output = Integer;
    fn sub(self, rhs: Self) -> Self::Output {
        return &self - &rhs;
    }
}

impl SubAssign for Integer {
    fn sub_assign(&mut self, rhs: Self) {
        *self = add_router(self,&-rhs);
    }
}

impl Mul for &Integer {
    type Output = Integer;
    fn mul(self, rhs: Self) -> Self::Output {
        return Integer {
            sign: self.sign ^ rhs.sign,
            abs_number: arbitrary_precision_mul(&self.abs_number, &rhs.abs_number),
        };
    }
}

impl Mul for Integer {
    type Output = Integer;
    fn mul(self, rhs: Self) -> Self::Output {
        return &self * &rhs;
    }
}

impl MulAssign for Integer {
    fn mul_assign(&mut self, rhs: Self) {
        self.sign = self.sign ^ rhs.sign;
        self.abs_number = arbitrary_precision_mul(&self.abs_number, &rhs.abs_number);
    }
}

impl Ord for Integer {
   
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = self - other;
        if result.abs_number == vec![0u64] {
            return Ordering::Equal;
        }
        if result.sign { //負数
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}

impl Div for &Integer {
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        return Rational::new(&self,&rhs);
    }
}

/**
 * 余剰を求める
 */
impl Rem for &Integer {
    type Output = Integer;
    fn rem(self, rhs: Self) -> Self::Output {
        todo!("imprement this")
    }
}


///rfmライブラリにおける有理数型の表現です。
///有理数は2つの整数型を組み合わせた分数で表現されます。
pub struct Rational {
    positive: Integer,
    divider: Integer,
}

impl Rational {

    /**
    新たな分数を作成する
    # Arguments
    * positive - 分子
    * divider - 分母
    # Returns
    指定された引数で構成される分数を返す。
    # Panics
    dividerに0を指定した場合、ゼロ除算の扱いとなり、処理を中止します。
     */
    fn new(positive:&Integer,divider:&Integer) -> Rational{
        if divider == &Integer::with_u64_value(0) {
            panic!("Divide by zero");//ゼロ除算防止
        }
        return Rational {
            positive:positive.clone(),
            divider:divider.clone(),
        }
    }

}

impl Add for &Rational{
    type Output = Rational;
    fn add(self, rhs: Self) -> Self::Output {

        return Rational::new(
            &(&self.positive * &rhs.divider + &rhs.positive * &self.divider),
            &(&self.divider * &rhs.divider)
        )
    }
}

impl Add for Rational{
    type Output = Rational;
    fn add(self, rhs: Self) -> Self::Output {
        return &self + &rhs;
    }
}

impl AddAssign for Rational{
    fn add_assign(&mut self, rhs: Self) {
        self.positive = &self.positive * &rhs.divider + &rhs.positive * &self.divider;
        self.divider  = &self.divider * &rhs.divider;
    }
}


impl Sub for &Rational{
    type Output = Rational;
    fn sub(self, rhs: Self) -> Self::Output {
        return Rational::new(
            &(&self.positive * &rhs.divider - &rhs.positive * &self.divider),
            &(&self.divider * &rhs.divider),
        )
    }
}
impl Sub for Rational{
    type Output = Rational;
    fn sub(self, rhs:Self) ->Rational{
        return &self - &rhs;
    }
}

impl SubAssign for Rational{
    fn sub_assign(&mut self, rhs: Self) {
        self.positive = &self.positive * &rhs.divider - &rhs.positive * &self.divider;
        self.divider = &self.divider * &rhs.divider;
    }
}

impl Mul for &Rational{
    type Output = Rational;
    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(
            &(&self.positive * &rhs.positive),
            &(&self.divider * &rhs.divider)
        )
    }
}

impl Mul for Rational{
    type Output = Rational;
    fn mul(self, rhs: Self) -> Self::Output {
        return &self * &rhs;
    }
}

impl MulAssign for Rational{
    fn mul_assign(&mut self, rhs: Self) {
        self.positive = &self.positive * &rhs.positive;
        self.positive = &self.divider * &rhs.divider;
    }
}

impl Div for &Rational{
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        Rational::new(
            &(&self.positive * &rhs.divider),
            &(&self.divider * &rhs.positive)
        ) 
    }
}

impl Div for Rational{
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        return &self * &rhs;
    }
}

impl DivAssign for Rational{
    fn div_assign(&mut self, rhs: Self) {
        if rhs.positive == Integer::with_u64_value(0) {
            panic!("Divide by zero")
        }
        self.positive = &self.positive * &rhs.divider;
        self.divider = &self.divider * &rhs.positive;
    }
}
