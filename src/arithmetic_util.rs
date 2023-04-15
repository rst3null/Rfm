use std::cmp;

use crate::vul::{Integer, Rational,Digit};


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
pub(crate) fn arbitrary_precision_add(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    let argsize = cmp::max(lhs.len(), rhs.len());
    let mut array: Vec<Digit> = Vec::with_capacity(argsize + 1); 
    let mut carry: Digit = 0; 
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
 戻り値は(Vec<Digit>,bool)のタプルで返される
 1. Vec<Digit> 計算後の値
 2. bool 符号(trueのとき負)
 */ 
pub(crate) fn arbitrary_precision_sub(lhs: &[Digit], rhs: &[Digit]) -> (Vec<Digit>, bool) {
    let array_len: usize = cmp::max(lhs.len(), rhs.len());
    let mut array_result: Vec<Digit> = Vec::with_capacity(array_len);
    let mut carry_down: Digit = 0; 
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
pub(crate) fn to_min_complement(number: &[Digit]) -> Vec<Digit> {
    let mut array_result = number.to_vec();
    let mut need_carry: bool = true;
    let mut first_zeros: bool = true;
    for i_ in 0..array_result.len() {

        if first_zeros && array_result[i_] == 0 {
            //ここでは桁上がりできないので0のまま放置
        }
        else {
            first_zeros = false;
            array_result[i_] = Digit::MAX - array_result[i_]; //補数表現から戻す
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
pub(crate) fn cut_upper_zeros(number:&[Digit]) -> Vec<Digit> {
    let mut num_mut = number.to_vec();
    for _i in (1..num_mut.len()).rev() {
        if num_mut[_i] == 0 as Digit {
            num_mut.remove(_i);
        } else {
            return num_mut;
        }
    }
    return num_mut;
}




///128bit符号無し整数同士の安全計算
pub(crate) fn safe_multiply_digit(lhs:Digit,rhs:Digit) -> Vec<Digit>{
    
    const HIGH_MASK:Digit = Digit::MAX << (Digit::BITS/2);
    const LOW_MASK:Digit = Digit::MAX ^ HIGH_MASK;
    
    
    let lhs_sep:(Digit,Digit) = (lhs & LOW_MASK, (lhs & HIGH_MASK) >> (Digit::BITS / 2));
    let rhs_sep:(Digit,Digit) = (rhs & LOW_MASK , (rhs & HIGH_MASK) >> (Digit::BITS / 2));
    let res_high:Digit = lhs_sep.1 * rhs_sep.1;
    let res_middle:Digit = lhs_sep.0 * rhs_sep.1 + lhs_sep.1 * rhs_sep.0;
    let res_low :Digit = lhs_sep.0 * rhs_sep.0;
    return cut_upper_zeros(&vec![res_low+((res_middle & LOW_MASK) << (Digit::BITS / 2)),res_high + ((res_middle & HIGH_MASK) >> (Digit::BITS / 2))]);
}

pub(crate) fn append_upper_zeros(number:&[Digit],needed_size:usize) -> Vec<Digit> {
    let num_append_digits = needed_size-number.len();
    return [number,&vec![0 as Digit;num_append_digits]].concat();
}


///正の整数同士を乗算する関数
///
pub(crate) fn arbitrary_precision_mul(lhs:&[Digit],rhs:&[Digit]) -> Vec<Digit>{
    assert!(lhs.len()!=0 && rhs.len()!=0);

    if &vec![0 as Digit;lhs.len()] == lhs || &vec![0 as Digit;rhs.len()] == rhs {
        return vec![0 as Digit];
    }

    if &vec![1 as Digit] == &cut_upper_zeros(lhs) {
        return rhs.to_vec();
    }

    if &vec![1 as Digit] == &cut_upper_zeros(rhs) {
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
            let result = safe_multiply_digit(lhs[0],rhs[0]);
            return result;
        },
        _ => {
            let split_point = length/2;
            let lhs_split = (&lhs[0..split_point],&lhs[split_point..length]);
            let rhs_split = (&rhs[0..split_point],&rhs[split_point..length]);

            let mul_down = match lhs_split.0.len() {
                0 =>panic!("Error in split logic. (generate zero sized array.)"),
                1 =>safe_multiply_digit(lhs_split.0[0], rhs_split.0[0]),
                _ =>arbitrary_precision_mul(&lhs_split.0, &rhs_split.0),
            };
            let mul_up = match lhs_split.1.len() {
                0 =>panic!("Error in split logic. (generator zero sized array.)"),
                1 =>safe_multiply_digit(lhs_split.1[0], rhs_split.1[0]),
                _=>arbitrary_precision_mul(&lhs_split.1, &rhs_split.1),
            };

            let lhs_sub_result = arbitrary_precision_sub(lhs_split.1,lhs_split.0);
            let rhs_sub_result = arbitrary_precision_sub(rhs_split.0,rhs_split.1);
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
            let middle = [vec![0 as Digit;split_point],karatsuba.0].concat();
            let upper = [vec![0 as Digit;split_point*2],mul_up].concat();

            let low_add_up = arbitrary_precision_add(&lower, &upper);
            let result =  match karatsuba.1 {
                false => cut_upper_zeros(&arbitrary_precision_add(&low_add_up, &middle)),
                true => cut_upper_zeros(&arbitrary_precision_sub(&low_add_up, &middle).0) 
            };
            return result;
        }
    }
    
}



/* 
fn arbitrary_precision_int_to_string(value:&[Digit])->String{
    let base:u64 = 10000000000000000000u64;
}*/

#[cfg(test)]
mod appdend_zeros_test{
    use crate::{arithmetic_util::append_upper_zeros, vul::Digit};

    #[test]
    fn test_append(){
        assert_eq!(append_upper_zeros(&vec![1 as Digit],3),vec![1 as Digit,0 as Digit,0 as Digit]);
    }
}
#[cfg(test)]
mod mul_arbitrary_test{
    use crate::{arithmetic_util::arbitrary_precision_mul, vul::Digit};

    #[test]
    fn test_mul_by_zero(){
        assert_eq!(arbitrary_precision_mul(&vec![1 as Digit;3],&vec![0 as Digit]),vec![0 as Digit]);
        assert_eq!(arbitrary_precision_mul(&vec![0 as Digit],&vec![1 as Digit;3]),vec![0 as Digit]);
    }

    #[test]
    fn test_mul_by_one(){
        let lhs = vec![1 as Digit,2 as Digit,3 as Digit];
        assert_eq!(arbitrary_precision_mul(&lhs,&vec![1 as Digit]),lhs);
        assert_eq!(arbitrary_precision_mul(&vec![1 as Digit],&lhs),lhs);
    }
    #[test]
    fn test_bug_case(){
        assert_eq!(arbitrary_precision_mul(&vec![1 as Digit],&vec![1 as Digit,3 as Digit]),vec![1 as Digit,3 as Digit]);
    }

    #[test]
    fn test_shift(){
        assert_eq!(arbitrary_precision_mul(&vec![0 as Digit,1 as Digit], &vec![1234567 as Digit]),vec![0 as Digit,1234567 as Digit]);
        assert_eq!(arbitrary_precision_mul(&vec![1234567 as Digit], &vec![0 as Digit,1 as Digit]),vec![0 as Digit,1234567 as Digit]);
    }

}

#[cfg(test)]
mod mul_digit_test{
    use crate::{arithmetic_util::safe_multiply_digit,vul::Digit};


    #[test]
    fn test_multiply(){
        assert_eq!(safe_multiply_digit(Digit::MAX,2 as Digit),vec![Digit::MAX - 1 as Digit,1]);
    }

    #[test]
    fn mul_zero(){
        assert_eq!(safe_multiply_digit(Digit::MAX,0 as Digit),vec![0 as Digit]);
    }
}

#[cfg(test)]
mod dedimal_add_kernel_tests {
    use crate::{arithmetic_util::arbitrary_precision_add, vul::Digit};

    #[test]
    fn test_1digit_add() {
        assert_eq!(vec![2 as Digit], arbitrary_precision_add(&vec![1 as Digit], &vec![1 as Digit]));
    }

    #[test]
    fn test_carry() {
        //桁上がり確認
        assert_eq!(vec![0 as Digit, 1 as Digit], arbitrary_precision_add(&vec![1 as Digit], &vec![Digit::MAX]));
        assert_eq!(vec![0 as Digit, 1 as Digit], arbitrary_precision_add(&vec![Digit::MAX], &vec![1 as Digit]));
    }

    #[test]
    fn test_carry_multiple() {
        assert_eq!(
            vec![0 as Digit, 0 as Digit, 0 as Digit, 0 as Digit, 0 as Digit, 0 as Digit, 0 as Digit, 0 as Digit, 2 as Digit],
            arbitrary_precision_add(
                &vec![
                    Digit::MAX,
                    Digit::MAX,
                    Digit::MAX,
                    Digit::MAX,
                    Digit::MAX,
                    Digit::MAX,
                    Digit::MAX,
                    Digit::MAX,
                    1 as Digit
                ],
                &vec![1 as Digit]
            )
        );
    }
}


#[cfg(test)]
mod decimal_sub_kernel_test {
    use crate::{arithmetic_util::arbitrary_precision_sub, vul::Digit};

    #[test]
    fn test_normal_substitute() {
        assert_eq!(
            (vec![90 as Digit], false),
            arbitrary_precision_sub(&vec![100 as Digit], &vec![10 as Digit])
        );
    }

    #[test]
    fn test_carrige_down() {
        assert_eq!(
            (vec![Digit::MAX], false),
            arbitrary_precision_sub(&vec![0 as Digit, 1 as Digit], &vec![1 as Digit])
        );
    }

    #[test]
    fn test_result_negative() {
        assert_eq!(
            (vec![1 as Digit], true),
            arbitrary_precision_sub(&vec![0 as Digit], &vec![1 as Digit])
        );
    }

    #[test]
    fn test_result_negative_carrige_down() {
        assert_eq!(
            (vec![1 as Digit], true),
            arbitrary_precision_sub(&vec![Digit::MAX], &vec![0 as Digit, 1 as Digit])
        );
    }

    #[test]
    fn test_result_negative_carrige_down_multiple() {
        assert_eq!(
            (vec![1 as Digit], true),
            arbitrary_precision_sub(&vec![Digit::MAX,Digit::MAX], &vec![0 as Digit,0 as Digit,1 as Digit])
        );
    }
}


