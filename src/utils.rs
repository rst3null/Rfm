use std::cmp;

/// 巨大な整数の加算処理(筆算アルゴリズム)
/// Internal decimal adder
/// # Arguments
/// * 'lhs' - 右辺値
/// * 'rhs' - 左辺値
/// 配列を多倍長整数と見做して加算する。
/// なお、この関数がサポートする数は自然数と0のみ。
///この計算量はO(N)である。
///
pub fn decimal_add_kernel(lhs: &Vec<u64>, rhs: &Vec<u64>) -> Vec<u64> {
    let argsize = cmp::max(lhs.len(), rhs.len());
    let mut array: Vec<u64> = Vec::with_capacity(argsize + 1); //桁上がりの範囲として+1の範囲を予約
    let mut carry: u64 = 0; //桁上がり
    for i_ in 0..argsize {
        let lhs_digit = *lhs.get(i_).unwrap_or(&0);
        let rhs_digit = *rhs.get(i_).unwrap_or(&0);
        let result_add = lhs_digit.overflowing_add(rhs_digit); //加算処理
        let result_number = result_add.0.overflowing_add(carry); //桁上がりを足して桁確定
        carry = 0;
        if true == result_add.1 {
            carry += 1; //加算による桁上がり
        }
        if true == result_number.1 {
            carry += 1; //桁上がり処理による数字確定字の桁上がり
        }
        array.push(result_number.0);
    }
    if 0 < carry {
        array.push(carry); //最後の桁上がりを処理する。
    }
    return array;
}


/// 減算計算を行う
/// Internal substitutor imprements.
///
/// # 引数 Arguments
/// * 'lhs' - 左辺値
/// * 'rhs' - 右辺値
///
/// # 戻り値 returns
/// 戻り値は(Vec<u64>,bool)のタプルで返される
/// 1. Vec<u64> 計算後の値
/// 2. bool 符号(trueのとき負)
///

pub fn decimal_sub_kernel(lhs: &Vec<u64>, rhs: &Vec<u64>) -> (Vec<u64>, bool) {
    let array_len: usize = cmp::max(lhs.len(), rhs.len());
    let mut array_result: Vec<u64> = Vec::with_capacity(array_len);
    let mut carry_down: u64 = 0; //桁下がり
    for _i in 0..array_len {
        let rhs_digit = *rhs.get(_i).unwrap_or(&0);
        let lhs_digit = *lhs.get(_i).unwrap_or(&0);
        let result_substitute = lhs_digit.overflowing_sub(rhs_digit);
        let result_number = result_substitute.0.overflowing_sub(carry_down);
        carry_down = 0;
        if result_substitute.1 {
            carry_down += 1;
        }
        if result_number.1 {
            carry_down += 1;
        }
        array_result.push(result_number.0);
    }

    return match carry_down {
        0 => (cut_upper_zeros(array_result), false),
        1 => {//キャリーが残ってしまった場合、補数表現になっているので元に戻す
            //答えは負数()
            let mut need_carry: bool = true;
            for i_ in 0..array_result.len() {
                if 0 != array_result[i_] {
                    array_result[i_] = u64::MAX - array_result[i_]; //補数表現になってしまうので戻す
                    if need_carry {
                        array_result[i_] += 1;
                        need_carry = false;
                    }
                }
            }
            return (cut_upper_zeros(array_result), true);
        }
        _ => panic!("It seems someting wrong in substitusion algorithm."), //余分に桁が降りてきているので計算ミス(panic)
    };
}


//上位桁の余った桁を除去する。
// cuting uppernumber zeros.
pub fn cut_upper_zeros(mut number: Vec<u64>) -> Vec<u64> {
    for _i in (1..number.len()).rev() {
        if number[_i] == 0u64 {
            number.remove(_i);
        } else {
            return number;
        }
    }
    return number;
}

//指数法則と分割統治法から、32bitごとに分割して計算することで1桁の64進数同士の乗算を安全に行うことを証明することができる。
//2^32 * 2^32 = 2^(32+32) = 2^64 
//つまりu32::MAX同士の乗算でも64bit整数ならギリギリ表現できるのである。
//そして、筆算をすることで計算量はO(1)である
pub fn safe_multiply_digit_64bit(lhs:u64,rhs:u64) -> (u64,u64){
    let lhs_sep:(u64,u64) = (lhs & 0xFFFFFFFFu64 , (lhs & 0xFFFFFFFF00000000u64)>>32 );
    let rhs_sep:(u64,u64) = (rhs & 0xFFFFFFFFu64 , (rhs & 0xFFFFFFFF00000000u64)>>32 );
    //2の64乗の桁と、2の32乗の桁、2の0乗の計算が必要
    let mut res_2p64:u64 = lhs_sep.1 * rhs_sep.1;
    let mut res_2p32:u64 = lhs_sep.0 * rhs_sep.1 + lhs_sep.1 * rhs_sep.0;
    let mut res_2p0 :u64 = lhs_sep.0 * rhs_sep.0;
    return (res_2p0+((res_2p32 & 0xFFFFFFFFu64)<<32),res_2p64 + ((res_2p32 & 0xFFFFFFFF00000000u64)>> 32));//2桁目へのはみ出し
}

#[cfg(test)]
mod mul_digit_test{
    #[test]
    fn test_multiply(){
        
    }
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
        assert_eq!(vec![0, 1u64], decimal_add_kernel(&vec![1], &vec![u64::MAX]));
        assert_eq!(vec![0, 1u64], decimal_add_kernel(&vec![u64::MAX], &vec![1]));
    }

    #[test]
    fn test_carry_multiple() {
        assert_eq!(
            vec![0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 2u64],
            decimal_add_kernel(
                &vec![
                    u64::MAX,
                    u64::MAX,
                    u64::MAX,
                    u64::MAX,
                    u64::MAX,
                    u64::MAX,
                    u64::MAX,
                    u64::MAX,
                    1u64
                ],
                &vec![1u64]
            )
        );
    }
}


#[cfg(test)]
mod decimal_sub_kernel_test {
    use crate::vul::decimal_sub_kernel;

    #[test]
    fn test_normal_substitute() {
        assert_eq!(
            (vec![90u64], false),
            decimal_sub_kernel(&vec![100u64], &vec![10u64])
        );
    }

    #[test]
    fn test_carrige_down() {
        assert_eq!(
            (vec![u64::MAX], false),
            decimal_sub_kernel(&vec![0u64, 1u64], &vec![1u64])
        );
    }

    #[test]
    fn test_result_negative() {
        assert_eq!(
            (vec![1u64], true),
            decimal_sub_kernel(&vec![0u64], &vec![1u64])
        );
    }

    #[test]
    fn test_result_negative_carrige_down() {
        assert_eq!(
            (vec![1u64], true),
            decimal_sub_kernel(&vec![u64::MAX], &vec![0u64, 1u64])
        );
    }

    #[test]
    fn test_result_negative_carrige_down_multiple() {
        assert_eq!(
            (vec![1u64], true),
            decimal_sub_kernel(&vec![u64::MAX,u64::MAX], &vec![0u64,0u64,1u64])
        );
    }
}

