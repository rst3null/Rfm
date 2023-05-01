/// 数値演算に関するtraitの集合体
/// 

use std::ops::*;


///プリミティブからの変換トレイト
pub trait FromPrimitiveNumber:Sized{
    fn from_i8(val:i8) -> Self{
        return Self::from_i128(val as i128);
    }
    fn from_u8(val:u8) -> Self{
        return Self::from_u128(val as u128);
    }
    fn from_i16(val:i16) -> Self{
        return Self::from_i128(val as i128);
    }
    fn from_u16(val:u16) -> Self{
        return Self::from_u128(val as u128);
    }
    fn from_i32(val:i32) -> Self{
        return Self::from_i128(val as i128);
    }
    fn from_u32(val:u32) -> Self{
        return Self::from_u128(val as u128);
    }
    fn from_i64(val:i64) -> Self{
        return Self::from_i128(val as i128);
    }
    fn from_u64(val:u64) -> Self{
        return Self::from_u128(val as u128);
    }
    fn from_i128(val:i128) -> Self;
    fn from_u128(val:u128) -> Self;
}

///乗除算における単位元の定義
pub trait Zero:Add + Sub + Mul + Div + Rem + Sized{
    fn zero()->Self;
}

///加減算における単位元の定義
pub trait One:Add + Sub + Mul + Div + Rem + Sized{
    fn one()->Self;
}

///偶奇判定
pub trait EvenOdd:Add + Sub + Mul + Div + Rem + Sized{
    fn is_even(&self)-> bool;
    fn is_odd(&self)-> bool;
}

///累乗計算
pub trait Pow:EvenOdd + FromPrimitiveNumber + Div + Sized{
    fn pow(&self,exp:Self) -> Self;
}

///平方根計算
pub trait Sqrt:Mul + Sized{
    
}

///割り算と同時に余りを求める効率化
pub trait DivRem:Div + Rem + Sized{
    fn div_rem(&self,rhs:&Self) -> (Self,Self);
}
