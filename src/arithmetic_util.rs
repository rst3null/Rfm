use std::cmp;


/**巨大な整数の加算処理(筆算アルゴリズム)
Internal decimal adder
# Arguments
 * 'lhs' - 右辺値
 * 'rhs' - 左辺値
配列を多倍長整数と見做して加算する。
なお、この関数がサポートする数は自然数と0のみ。
# Returns
計算結果を表す任意精度整数値
# Compute cost
この計算量はO(N)である。
 */
pub(crate) fn arbitrary_precision_add(lhs: &[u64], rhs: &[u64]) -> Vec<u64> {
    let argsize = cmp::max(lhs.len(), rhs.len());
    let mut array: Vec<u64> = Vec::with_capacity(argsize + 1); 
    let mut carry: u64 = 0; 
    for i_ in 0..argsize {
        let lhs_digit = *lhs.get(i_).unwrap_or(&0);
        let rhs_digit = *rhs.get(i_).unwrap_or(&0);
        let result_add = lhs_digit.overflowing_add(rhs_digit); 
        let result_number = result_add.0.overflowing_add(carry); 
        carry = 0;
        if true == result_add.1 {
            carry += 1; 
        }
        if true == result_number.1 {
            carry += 1; 
        }
        array.push(result_number.0);
    }
    if 0 < carry {
        array.push(carry); 
    }
    return array;
}


/**  減算計算を行う
 Internal substitutor imprements.
 
 # 引数 Arguments
  * 'lhs' - 左辺値
  * 'rhs' - 右辺値
 
 # 戻り値 returns
 戻り値は(Vec<u64>,bool)のタプルで返される
 1. Vec<u64> 計算後の値
 2. bool 符号(trueのとき負)
 */ 
pub(crate) fn arbitrary_precision_sub(lhs: &[u64], rhs: &[u64]) -> (Vec<u64>, bool) {
    let array_len: usize = cmp::max(lhs.len(), rhs.len());
    let mut array_result: Vec<u64> = Vec::with_capacity(array_len);
    let mut carry_down: u64 = 0; 
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
        0 => (cut_upper_zeros(&array_result), false),
        1 => {
            return (cut_upper_zeros(&to_min_complement(&array_result)), true);
        }
        _ => panic!("Something wrong in substitution algorithm. Please contact to developer.\nERROR_TYPE:Multiple carry down occured."), //余分に桁が降りてきているので計算ミス
    };
}



/**最小の補数表現にする
 * また、補数から絶対値に戻すこともできる。
 * */
pub(crate) fn to_min_complement(number: &[u64]) -> Vec<u64> {
    let mut array_result = number.to_vec();
    let mut need_carry: bool = true;
    let mut first_zeros: bool = true;
    for i_ in 0..array_result.len() {

        if first_zeros && array_result[i_] == 0 {
            //ここでは桁上がりできないので0のまま放置
        }
        else {
            first_zeros = false;
            array_result[i_] = u64::MAX - array_result[i_]; //補数表現から戻す
            if need_carry {
                array_result[i_] += 1;//最初の桁だけ1加算する。
                need_carry = false;
            }
        }
    }
    return array_result;
}



///上位桁の余った桁を除去する。
///cuting uppernumber zeros.
///
pub(crate) fn cut_upper_zeros(number:&[u64]) -> Vec<u64> {
    let mut num_mut = number.to_vec();
    for _i in (1..num_mut.len()).rev() {
        if num_mut[_i] == 0u64 {
            num_mut.remove(_i);
        } else {
            return num_mut;
        }
    }
    return num_mut;
}


 

//指数法則と分割統治法から、32bitごとに分割して計算することで1桁の64進数同士の乗算を安全に行うことを証明することができる。
//2^32 * 2^32 = 2^(32+32) = 2^64 
//つまりu32::MAX同士の乗算でも64bit整数ならギリギリ表現できるのである。
//そして、筆算をすることで計算量はO(1)である
pub(crate) fn safe_multiply_digit_64bit(lhs:u64,rhs:u64) -> Vec<u64>{
    let lhs_sep:(u64,u64) = (lhs & 0xFFFFFFFFu64 , (lhs & 0xFFFFFFFF00000000u64)>>32 );
    let rhs_sep:(u64,u64) = (rhs & 0xFFFFFFFFu64 , (rhs & 0xFFFFFFFF00000000u64)>>32 );
    let res_2p64:u64 = lhs_sep.1 * rhs_sep.1;
    let res_2p32:u64 = lhs_sep.0 * rhs_sep.1 + lhs_sep.1 * rhs_sep.0;
    let res_2p0 :u64 = lhs_sep.0 * rhs_sep.0;
    return cut_upper_zeros(&vec![res_2p0+((res_2p32 & 0xFFFFFFFFu64)<<32),res_2p64 + ((res_2p32 & 0xFFFFFFFF00000000u64)>> 32)]);
}

pub(crate) fn append_upper_zeros(number:&[u64],needed_size:usize) -> Vec<u64> {
    let num_append_digits = needed_size-number.len();
    return [number,&vec![0u64;num_append_digits]].concat();
}


///正の整数同士を乗算する関数
///
pub(crate) fn arbitrary_precision_mul(lhs:&[u64],rhs:&[u64]) -> Vec<u64>{
    assert!(lhs.len()!=0 && rhs.len()!=0);

    if &vec![0u64;lhs.len()] == lhs || &vec![0u64;rhs.len()] == rhs {
        return vec![0u64];
    }

    if &vec![1u64] == &cut_upper_zeros(lhs) {
        return rhs.to_vec();
    }

    if &vec![1u64] == &cut_upper_zeros(rhs) {
        return lhs.to_vec();
    }

    if lhs.len() < rhs.len() {
        return arbitrary_precision_mul(&append_upper_zeros(lhs,rhs.len()),rhs);
    }
    if lhs.len() > rhs.len() {
        return arbitrary_precision_mul(lhs,&append_upper_zeros(rhs,lhs.len()));
    }

    let length = lhs.len();

    match length {
        1 => {
            let result = safe_multiply_digit_64bit(lhs[0],rhs[0]);
            return result;
        },
        _ => {
            let split_point = length/2;
            let lhs_split = (&lhs[0..split_point],&lhs[split_point..length]);
            let rhs_split = (&rhs[0..split_point],&rhs[split_point..length]);

            let mul_down = match lhs_split.0.len() {
                0 =>panic!("Error in split logic. (generate zero sized array.)"),
                1 =>safe_multiply_digit_64bit(lhs_split.0[0], rhs_split.0[0]),
                _ =>arbitrary_precision_mul(&lhs_split.0, &rhs_split.0),
            };
            let mul_up = match lhs_split.1.len() {
                0 =>panic!("Error in split logic. (generator zero sized array.)"),
                1 =>safe_multiply_digit_64bit(lhs_split.1[0], rhs_split.1[0]),
                _=>arbitrary_precision_mul(&lhs_split.1, &rhs_split.1),
            };

            let lhs_sub_result = arbitrary_precision_sub(lhs_split.1,lhs_split.0);
            let rhs_sub_result = arbitrary_precision_sub(rhs_split.0,rhs_split.1);
            dbg!(&rhs_sub_result);
            let mul_karatsuba = arbitrary_precision_mul(
                &lhs_sub_result.0,
                &rhs_sub_result.0
            );
            let add_karatsuba = arbitrary_precision_add(&mul_down, &mul_up);
            let karatsuba = match lhs_sub_result.1 ^ rhs_sub_result.1 {
                true => arbitrary_precision_sub(&add_karatsuba, &mul_karatsuba),
                false => (arbitrary_precision_add(&add_karatsuba, &mul_karatsuba),lhs_sub_result.1)
            };
            let lower = mul_down;
            let middle = [vec![0u64;split_point],karatsuba.0].concat();
            let upper = [vec![0u64;split_point*2],mul_up].concat();

            let low_add_up = arbitrary_precision_add(&lower, &upper);
            let result =  match karatsuba.1 {
                false => cut_upper_zeros(&arbitrary_precision_add(&low_add_up, &middle)),
                true => cut_upper_zeros(&arbitrary_precision_sub(&low_add_up, &middle).0) 
            };
            return result;
        }
    }
    
}


///逆数を求める
///逆数は分数形式(分子、分母)で出力されるが、
///分母は必ず2^64nの形式になる。
///なお、逆数が存在しない場合はNoneを返す
/*fn calculate_inverse(arg:&[u64]) -> Option<(Vec<u64>,Vec<u64>)>{
    if arg == vec![0u64;arg.len()] {
        return None;//解なし
    }
    //予測値
    let mut predict:Rational = ;
}*/

#[cfg(test)]
mod appdend_zeros_test{
    use crate::arithmetic_util::append_upper_zeros;

    #[test]
    fn test_append(){
        assert_eq!(append_upper_zeros(&vec![1u64],3),vec![1u64,0u64,0u64]);
    }
}
#[cfg(test)]
mod mul_arbitrary_test{
    use crate::arithmetic_util::arbitrary_precision_mul;

    #[test]
    fn test_mul_by_zero(){
        assert_eq!(arbitrary_precision_mul(&vec![1u64;3],&vec![0u64]),vec![0u64]);
        assert_eq!(arbitrary_precision_mul(&vec![0u64],&vec![1u64;3]),vec![0u64]);
    }

    #[test]
    fn test_mul_by_one(){
        let lhs = vec![1u64,2u64,3u64];
        assert_eq!(arbitrary_precision_mul(&lhs,&vec![1u64]),lhs);
        assert_eq!(arbitrary_precision_mul(&vec![1u64],&lhs),lhs);
    }
    #[test]
    fn test_bug_case(){
        assert_eq!(arbitrary_precision_mul(&vec![1u64],&vec![1u64,3u64]),vec![1u64,3u64]);
    }

    #[test]
    fn test_shift(){
        assert_eq!(arbitrary_precision_mul(&vec![0u64,1u64], &vec![1234567u64]),vec![0u64,1234567u64]);
        assert_eq!(arbitrary_precision_mul(&vec![1234567u64], &vec![0u64,1u64]),vec![0u64,1234567u64]);
    }

}

#[cfg(test)]
mod mul_digit_test{
    use crate::arithmetic_util::safe_multiply_digit_64bit;


    #[test]
    fn test_multiply(){
        assert_eq!(safe_multiply_digit_64bit(u64::MAX,2u64),vec![u64::MAX-1u64,1]);
    }

    #[test]
    fn mul_zero(){
        assert_eq!(safe_multiply_digit_64bit(u64::MAX,0u64),vec![0u64]);
    }
}

#[cfg(test)]
mod dedimal_add_kernel_tests {
    use crate::arithmetic_util::arbitrary_precision_add;

    #[test]
    fn test_1digit_add() {
        assert_eq!(vec![2u64], arbitrary_precision_add(&vec![1u64], &vec![1u64]));
    }

    #[test]
    fn test_carry() {
        //桁上がり確認
        assert_eq!(vec![0, 1u64], arbitrary_precision_add(&vec![1], &vec![u64::MAX]));
        assert_eq!(vec![0, 1u64], arbitrary_precision_add(&vec![u64::MAX], &vec![1]));
    }

    #[test]
    fn test_carry_multiple() {
        assert_eq!(
            vec![0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 2u64],
            arbitrary_precision_add(
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
    use crate::arithmetic_util::arbitrary_precision_sub;

    #[test]
    fn test_normal_substitute() {
        assert_eq!(
            (vec![90u64], false),
            arbitrary_precision_sub(&vec![100u64], &vec![10u64])
        );
    }

    #[test]
    fn test_carrige_down() {
        assert_eq!(
            (vec![u64::MAX], false),
            arbitrary_precision_sub(&vec![0u64, 1u64], &vec![1u64])
        );
    }

    #[test]
    fn test_result_negative() {
        assert_eq!(
            (vec![1u64], true),
            arbitrary_precision_sub(&vec![0u64], &vec![1u64])
        );
    }

    #[test]
    fn test_result_negative_carrige_down() {
        assert_eq!(
            (vec![1u64], true),
            arbitrary_precision_sub(&vec![u64::MAX], &vec![0u64, 1u64])
        );
    }

    #[test]
    fn test_result_negative_carrige_down_multiple() {
        assert_eq!(
            (vec![1u64], true),
            arbitrary_precision_sub(&vec![u64::MAX,u64::MAX], &vec![0u64,0u64,1u64])
        );
    }
}


