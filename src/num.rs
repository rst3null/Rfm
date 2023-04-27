/**Rust用の任意精度演算モジュールです
金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。
現時点で整数と有理数のみに対応しています。
*/
use crate::arithmetic_util::*;
use std::cmp::*;
use std::ops::*;
use crate::num_traits;

/**
本ライブラリにおける1桁の型
ビット幅を変える必要がある場合は型を変更してください。(ただし整数に限る)
*/
pub type Digit = u128;

/**
 * 符号型として新しく定義する。
 * これによって「負の0」問題を回避する
 */
#[derive(NumOps,Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
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
            (_, Sign::Zero) => panic!("Div by zero"),
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
    number_data: Vec<Digit>,

    ///符号管理フラグ
    ///trueのとき、負数となる。
    sign: Sign,
}

impl Integer {
    pub fn from_u128_value(value: u128) -> Integer {
        return Integer {
            number_data: vec![value],
            sign: match value {
                0 => Sign::Zero,
                _ => Sign::Positive,
            },
        };
    }
    pub fn from_i128_value(value: i128) -> Integer {
        return Integer {
            number_data: vec![value.abs() as Digit],
            sign: match value {
                1.. => Sign::Positive,
                0 => Sign::Zero,
                _ => Sign::Negative,
            },
        };
    }
    pub fn from_number_slice(value: &[Digit], sign: Sign) -> Integer {
        let mut result_sign = sign;
        if value == &[0 as Digit] {
            //絶対値がゼロの場合
            result_sign = Sign::Zero;
        } else if result_sign == Sign::Zero {
            //絶対値が0でもないのにゼロ符号を与えられた場合はpanic!とする
            panic!("non zero value, but zero sign assigned.");
        }
        return Integer {
            number_data: value.to_vec(),
            sign: result_sign,
        };
    }
    pub fn abs(&self) -> Integer {
        return Integer::from_number_slice(&self.number_data, Sign::Positive);
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
    return match (&lhs.sign, &rhs.sign) {
        //正の整数同士の加算
        (Sign::Positive, Sign::Positive) => Integer::from_number_slice(
            &arbitrary_precision_add(&lhs.number_data, &rhs.number_data),
            Sign::Positive,
        ),
        //負の整数同士
        (Sign::Negative, Sign::Negative) => Integer::from_number_slice(
            &arbitrary_precision_add(&lhs.number_data, &rhs.number_data),
            Sign::Negative,
        ),

        //順番を入れ替えて減算として処理
        (Sign::Negative, Sign::Positive) => {
            let result_sub = arbitrary_precision_sub(&rhs.number_data, &lhs.number_data);
            return Integer::from_number_slice(
                &result_sub.0,
                match result_sub.1 {
                    true => Sign::Negative,
                    false => Sign::Positive,
                },
            );
        }
        //相手がマイナスなので引き算と等価
        (Sign::Positive, Sign::Negative) => {
            let result_sub = arbitrary_precision_sub(&lhs.number_data, &rhs.number_data);
            return Integer::from_number_slice(
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
        return Integer::from_number_slice(&self.number_data, -&self.sign);
    }
}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        return Integer::from_number_slice(&self.number_data, -self.sign);
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
        return match (&self.sign, &rhs.sign) {
            (Sign::Zero, _) | (_, Sign::Zero) => Integer::from_u128_value(0),
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
                Integer::from_number_slice(
                    &arbitrary_precision_mul(&self.number_data, &rhs.number_data),
                    Sign::Positive,
                )
            }
            (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
                Integer::from_number_slice(
                    &arbitrary_precision_mul(&self.number_data, &rhs.number_data),
                    Sign::Negative,
                )
            }
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
        self.sign = &self.sign * &rhs.sign;
        self.number_data = arbitrary_precision_mul(&self.number_data, &rhs.number_data);
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
        let sign: Sign = &self.sign / &rhs.sign;
        let (inverse_num, inverse_exp) = calculate_inverse(rhs);
        let mut result_value = self * &inverse_num;
        result_value.number_data.drain(0..inverse_exp.abs() as usize);//桁繰り下げ
        result_value.sign=sign;
        return result_value;
    }
}

fn calculate_inverse(rhs: &Integer) -> (Integer, i128) {
    let calc_number = rhs.number_data.len()+1;
    //有効数字を1多くして処理、収束したら最下桁を消す。
    let src = rhs.abs();
    let mut predict = (
        Integer::from_u128_value(1 as Digit),
        -&(calc_number as i128) + 1,
    );
    loop {
        let mul_predict_src = (&predict.0 * &src, predict.1);
        //2を求める
        let mut two_vec = vec![0 as Digit; (0 - mul_predict_src.1) as usize];
        two_vec.push(2);
        let predict_step2 = (
            Integer::from_number_slice(&two_vec, Sign::Positive) - mul_predict_src.0,
            mul_predict_src.1,
        );
        let mut next_predict = (&predict_step2.0 * &predict.0, &predict_step2.1 + &predict.1);
        let next_predict_len = next_predict.0.number_data.len();

        if next_predict_len > calc_number {
            let diff: usize = next_predict_len - calc_number;
            next_predict.0.number_data.drain(0..diff);
            next_predict.1 += diff as i128;//drainしたので差分調整
        }

        if next_predict.0 == predict.0 {//桁を落としているため、最終的に収束することによって等価演算子による演算は問題ない。
            break;
        }
    
        predict = next_predict;

    }
    //四捨五入相当の操作を実行(乗算して試すより安定チェックで1桁多く取っているので乗算する必要なし)
    if predict.0.number_data[0] >= (1 << (Digit::BITS/2)) {
        predict.0.number_data[1] += 1;

    }
    predict.0.number_data.remove(0);
    predict.1 += 1;//差分調整
    debug_assert!(predict.1 <= 0);
    predict
}

impl Div for Integer {
    type Output = Integer;
    fn div(self, rhs: Self) -> Self::Output {
        return &self / &rhs;
    }
}

impl Rem for &Integer {
    type Output = Integer;
    fn rem(self, rhs: Self) -> Self::Output {
        return self - &(&( self / &rhs ) * &rhs);
    }
}

impl Rem for Integer{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        return &self % &rhs;
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
        };
    }
}

#[cfg(test)]
mod integer_test {
    use super::{Digit, Integer, Sign};

    #[test]
    fn div_test() {
        assert_eq!(
            Integer::from_u128_value(1024),
            Integer::from_u128_value(2048) / Integer::from_u128_value(2)
        );
    }
    #[test]
    fn div_remained_test(){
        assert_eq!(
            Integer::from_u128_value(2),
            Integer::from_u128_value(20) / Integer::from_u128_value(7)
        )
    }

    #[test]
    fn remain_test(){
        assert_eq!(
            Integer::from_u128_value(6),
            Integer::from_u128_value(20) % Integer::from_u128_value(7)
        )
    }

    #[test]
    fn mul_test() {
        let a = Integer::from_number_slice(
            &vec![0 as Digit, 340282366920938463463374607431768211454 as Digit],
            Sign::Positive,
        );
        let b = Integer::from_u128_value(1 as Digit);
        assert_eq!(&a * &b, a);
        assert_eq!(&b * &a, a);
    }
}
