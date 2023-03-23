///Rust用の多倍長精度演算プロジェクトrfmです。
///金融計算など、高精度な計算が必要な場面でpure rustで計算を実行します。

pub mod rfm {
    ///多倍長演算プロジェクトの各種
    /// 
    
    use std::cmp;
    //use std::ops::{Add, Div, Mul, Sub};

    ///rfmライブラリにおける整数型の表現です。
    pub struct Integer {
        negative: bool,
        value_array: Vec<u8>,
    }
    






    ///加算(内部用)
    ///この計算量はO(N)である。
    fn decimal_add_kernel(lhs: &Vec<u8>, rhs: &Vec<u8>) -> Vec<u8>{
        let argsize = cmp::max(lhs.len(), rhs.len());
        let mut array: Vec<u8> = Vec::with_capacity(argsize + 1); //桁上がりの範囲は予測可能
        let mut carry: u8 = 0; //桁上がり
        for i_ in 0..argsize {
            
            let lhs_digit:u8 = *lhs.get(i_).unwrap_or(&0);
            let rhs_digit:u8 = *rhs.get(i_).unwrap_or(&0);
            let result_add: u8 = lhs_digit + rhs_digit + carry;
            let this_digit: u8 = result_add % 10; //1桁目を取り出す。
            carry = (result_add - this_digit) / 10; //桁上がりを記録する。
            array.push(this_digit);
        }
        if 0 < carry {
            array.push(carry);//最後の桁上がりを処理する。
        }
        return array;
    }


    #[cfg(test)]
    mod dedimal_add_kernel_tests {
        use crate::rfm::decimal_add_kernel;

        #[test]
        fn test_1digit_add(){
            assert_eq!(vec![2],decimal_add_kernel(&vec![1],&vec![1]));
        }

        #[test]
        fn test_carry(){
            //二桁以上の数値は順番を反転しないとおかしくなる。
            assert_eq!(vec![0,1],decimal_add_kernel(&vec![9],&vec![1]));
            assert_eq!(vec![0,1],decimal_add_kernel(&vec![1],&vec![9]));
        }

        #[test]
        fn test_carry_multiple(){
            assert_eq!(vec![0,1,1,1,1,1,1,1,1,1],decimal_add_kernel(&vec![1,2,3,4,5,6,7,8,9],&vec![9,8,7,6,5,4,3,2,1]));
        }
    }

    //減算(内部用)
    /* unimpremented
    fn decimal_sub_kernel(lhs: vec<u8>,rhs: vec<u8>) -> vec<u8>{
        let argsize = cmp::max(lhs.value_array.len(), rhs.value_array.len());
        let mut array: Vec<u8> = Vec::with_capacity(argsize);
        let mut carry_down:u8 = 0;//桁落とし処理
        for i_ in 0..argsize {
            let lhs_digit:u8 = lhs.get().unwrap_or(0);
            let rhs_digit:u8 = rhs.get().unwrap_or(0);
        }
    }*/
}
