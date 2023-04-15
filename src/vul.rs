/**Rust用の任意精度演算モジュールです
金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。
現時点で整数と有理数のみに対応しています。
*/
use crate::arithmetic_util::*;
use std::cmp::*;
use std::ops::*;

/**
本ライブラリにおける1桁の型
ビット幅を変える必要がある場合は型を変更してください。(ただし整数に限る)
*/
pub type Digit = u128;

/**
 * 符号型として新しく定義する。
 * これによって「負の0」問題を回避する
 */
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

impl Neg for &Sign {
    type Output = Sign;
    fn neg(self) -> Self::Output {
        match self {
            Sign::Positive => return Sign::Negative,
            Sign::Negative => return Sign::Positive,
            Sign::Zero => self.clone(),
        }
    }
}

impl Neg for Sign {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Sign::Positive => return Sign::Negative,
            Sign::Negative => return Sign::Positive,
            Sign::Zero => self,
        }
    }
}

impl Mul for &Sign {
    type Output = Sign;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
        }
    }
}

impl Mul for Sign {
    type Output = Sign;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
        }
    }
}

impl Div for &Sign {
    type Output = Sign;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (_,Sign::Zero) => panic!("Div by zero"),
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
            (Sign::Zero, _) => Sign::Zero,
        }
    }
}

/**
rfmライブラリにおける整数型の表現です。
Integer expression in rfm library.
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Integer {
    ///整数の絶対値
    ///この配列は絶対値を保持しており、補数表現をしてはならない。
    abs_number: Vec<Digit>,

    ///符号管理フラグ
    ///trueのとき、負数となる。
    sign: Sign,
}

impl Integer {
    pub fn from_u128_value(value: u128) -> Integer {
        return Integer {
            abs_number: vec![value],
            sign: match value {
                0 =>Sign::Zero,
                _ =>Sign::Positive,
            }
        };
    }
    pub fn from_i128_value(value: i128) -> Integer {
        return Integer {
            abs_number: vec![value.abs() as Digit],
            sign: match value {
                1.. => Sign::Positive,
                0 => Sign::Zero,
                _ => Sign::Negative,
            },
        };
    }
    pub fn from_u128_slice(value: &[Digit], sign: Sign) -> Integer {
        let mut result_sign = sign;
        if value == &[0 as Digit] { //絶対値がゼロの場合
            result_sign = Sign::Zero;
        } else if result_sign == Sign::Zero {
            //絶対値が0でもないのにゼロ符号を与えられた場合はpanic!とする
            panic!("non zero value, but zero sign assigned.");
        }
        return Integer {
            abs_number: value.to_vec(),
            sign:result_sign,
        };
    }
}

impl Add for &Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        add_router(&self, &rhs)
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
        *self = add_router(self, &other);
    }
}

fn add_router(lhs: &Integer, rhs: &Integer) -> Integer {
    return match (&lhs.sign,&rhs.sign) {
        //正の整数同士の加算
        (Sign::Positive, Sign::Positive) => Integer::from_u128_slice(
            &arbitrary_precision_add(&lhs.abs_number, &rhs.abs_number),
            Sign::Positive,
        ),
        //負の整数同士
        (Sign::Negative, Sign::Negative) => Integer::from_u128_slice(
            &arbitrary_precision_add(&lhs.abs_number, &rhs.abs_number),
            Sign::Negative,
        ),

        //順番を入れ替えて減算として処理
        (Sign::Negative, Sign::Positive) => {
            let result_sub = arbitrary_precision_sub(&rhs.abs_number, &lhs.abs_number);
            return Integer::from_u128_slice(
                &result_sub.0,
                match result_sub.1 {
                    true => Sign::Negative,
                    false => Sign::Positive,
                },
            );
        }
        //相手がマイナスなので引き算と等価
        (Sign::Positive, Sign::Negative) => {
            let result_sub = arbitrary_precision_sub(&lhs.abs_number, &rhs.abs_number);
            return Integer::from_u128_slice(
                &result_sub.0,
                match result_sub.1 {
                    true => Sign::Negative,
                    false => Sign::Positive,
                },
            );
        }
        (Sign::Zero, _) => rhs.clone(),
        (_, Sign::Zero) => lhs.clone(),
    };
}

impl Neg for &Integer {
    type Output = Integer;
    fn neg(self) -> Self::Output {
        return Integer::from_u128_slice(&self.abs_number, -&self.sign);
    }
}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        return Integer::from_u128_slice(&self.abs_number, -self.sign);
    }
}

impl Sub for &Integer {
    type Output = Integer;
    fn sub(self, rhs: Self) -> Self::Output {
        return add_router(&self, &-rhs);
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
        *self = add_router(self, &-rhs);
    }
}

impl Mul for &Integer {
    type Output = Integer;
    fn mul(self, rhs: Self) -> Self::Output {
        return match (&self.sign,&rhs.sign) {
            (Sign::Zero,_) | (_,Sign::Zero) =>  Integer::from_u128_value(0),
            (Sign::Positive,Sign::Positive) | (Sign::Negative,Sign::Negative) => Integer::from_u128_slice(
                &arbitrary_precision_mul(&self.abs_number, &rhs.abs_number),
                Sign::Positive
            ),
            (Sign::Positive,Sign::Negative) | (Sign::Negative,Sign::Positive) => 
                Integer::from_u128_slice(
                    &arbitrary_precision_mul(&self.abs_number, &rhs.abs_number),
                Sign::Negative
            ),
        }
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
        self.sign = &self.sign * &rhs.sign;
        self.abs_number = arbitrary_precision_mul(&self.abs_number, &rhs.abs_number);
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = self - other;
        match result.sign {
            Sign::Positive => Ordering::Greater,
            Sign::Zero => Ordering::Equal,
            Sign::Negative => Ordering::Less,
        }
    }
}

impl Div for &Integer {
    type Output = Integer;
    fn div(self, rhs: Self) -> Self::Output {
        if *rhs == Integer::from_u128_value(0) {
            panic!("Div by zero");
        }
        //let valid_number:usize = ;
        todo!("imprement required");
    }
}

impl Rem for &Integer {
    type Output = Integer;
    fn rem(self, rhs: Self) -> Self::Output {
        todo!("imprement this");
    }
}

/*
impl std::fmt::Display for &Integer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}
*/

/**rfmライブラリにおける有理数型の表現です。

 有理数は2つの整数型を組み合わせた分数で表現されます。
 # Panics
 ゼロ除算となるような分数を作成しようとした場合panic!により停止します。
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
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
    pub fn new(positive: &Integer, divider: &Integer) -> Rational {
        if divider == &Integer::from_u128_value(0) {
            panic!("Divide by zero"); //ゼロ除算防止
        }
        return Rational {
            positive: positive.clone(),
            divider: divider.clone(),
        };
    }

    pub fn from_intager(val: &Integer) -> Rational {
        return Rational {
            positive: val.clone(),
            divider: Integer::from_u128_value(1),
        };
    }
}

impl Add for &Rational {
    type Output = Rational;
    fn add(self, rhs: Self) -> Self::Output {
        return Rational::new(
            &(&self.positive * &rhs.divider + &rhs.positive * &self.divider),
            &(&self.divider * &rhs.divider),
        );
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Self) -> Self::Output {
        return &self + &rhs;
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        self.positive = &self.positive * &rhs.divider + &rhs.positive * &self.divider;
        self.divider = &self.divider * &rhs.divider;
    }
}

impl Sub for &Rational {
    type Output = Rational;
    fn sub(self, rhs: Self) -> Self::Output {
        return Rational::new(
            &(&self.positive * &rhs.divider - &rhs.positive * &self.divider),
            &(&self.divider * &rhs.divider),
        );
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, rhs: Self) -> Rational {
        return &self - &rhs;
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        self.positive = &self.positive * &rhs.divider - &rhs.positive * &self.divider;
        self.divider = &self.divider * &rhs.divider;
    }
}

impl Mul for &Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(
            &(&self.positive * &rhs.positive),
            &(&self.divider * &rhs.divider),
        )
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Self::Output {
        return &self * &rhs;
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        self.positive = &self.positive * &rhs.positive;
        self.positive = &self.divider * &rhs.divider;
    }
}

impl Div for &Rational {
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        Rational::new(
            &(&self.positive * &rhs.divider),
            &(&self.divider * &rhs.positive),
        )
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, rhs: Self) -> Self::Output {
        return &self * &rhs;
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.positive == Integer::from_u128_value(0) {
            panic!("Divide by zero")
        }
        self.positive = &self.positive * &rhs.divider;
        self.divider = &self.divider * &rhs.positive;
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self - other;
        return match result.positive.sign * result.divider.sign {
            Sign::Positive => Ordering::Greater,
            Sign::Negative => Ordering::Less,
            Sign::Zero => Ordering::Equal,
        }
    }
}
