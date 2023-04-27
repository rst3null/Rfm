///数値演算に関するtrait
/// 
use std::cmp::*;
use std::ops::*;

trait Zero{
    fn zero()->Self;
}

trait One{
    fn one()->Self;
}

///四則演算を定義するトレイト
///このトレイトを実装することにより四則演算が定義されていることを保証する。
trait NumOps:Add + Sub + Mul + Div + Rem{

}