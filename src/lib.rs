pub use schemes::*;
use statrs::function::{erf::erfc, gamma::gamma_ur};

mod schemes;

pub fn entry_scheme_transpose(bits: &[u8], pattern: &Scheme) -> Vec<u8> {
    let mut result = Vec::new();
    for index in pattern {
        result.push(bits[*index as usize]);
    }
    result
}

pub fn retrieval_scheme_transpose(bits: &[u8], pattern: &Scheme) -> Vec<u8> {
    let mut result = vec![0; 64];
    for (i, index) in pattern.iter().enumerate() {
        result[*index as usize] = bits[i];
    }
    result
}

pub fn monobit(bits: &[u8]) -> f64 {
    let n = bits.len();
    let mut sn = 0_i32;
    for bit in bits {
        sn += *bit as i32 * 2 - 1;
    }
    let s = sn.abs() as f64 / (n as f64).sqrt();
    erfc(s / 2_f64.sqrt())
}

#[allow(non_snake_case)]
pub fn blockbit(bits: &[u8], M: usize) -> f64 {
    let n = bits.len();
    let N = n / M;

    let mut pis = vec![];
    for i in 0..N {
        let mut ones = 0;
        for j in 0..M {
            if bits[(i * M) + j] == 1 {
                ones += 1
            }
        }
        pis.push(ones as f64 / M as f64);
        if pis.len() == N {
            break;
        }
    }
    let pis = pis
        .into_iter()
        .map(|ones| (ones - 0.5).powf(2_f64))
        .sum::<f64>();

    let x2 = (4 * M) as f64 * pis;
    gamma_ur(N as f64 / 2.0, x2 / 2.0)
}

pub fn runs(bits: &[u8]) -> f64 {
    let n = bits.len();
    let pi = bits.iter().filter(|b| **b == 1).count() as f64 / n as f64;
    let tau = 2.0 / (n as f64).sqrt();
    if (pi - 0.5).abs() >= tau {
        return 0.0;
    }
    let mut vn: usize = 1;
    for i in 0..n - 1 {
        if bits[i] == bits[i + 1] {
            vn += 0;
        } else {
            vn += 1;
        }
    }
    let numerator = (vn as f64 - 2.0 * n as f64 * pi * (1.0 - pi)).abs();
    let denominator = 2.0 * (2.0 * n as f64).sqrt() * pi * (1.0 - pi);
    erfc(numerator / denominator)
}

#[cfg(test)]
mod tests {

    use crate::{blockbit, monobit, runs};

    fn str_bit_to_vec_u8(input: &str) -> Vec<u8> {
        input
            .chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .collect::<Vec<u8>>()
    }

    #[test]
    fn monobit_test() {
        let input = str_bit_to_vec_u8("1011010101");
        assert_eq!(&format!("{:.6}", monobit(&input)), "0.527089");

        let input = str_bit_to_vec_u8(
            "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000",
        );
        assert_eq!(&format!("{:.6}", monobit(&input)), "0.109599");
    }

    #[test]
    fn blockbit_test() {
        let input = str_bit_to_vec_u8("0110011010");
        assert_eq!(&format!("{:.6}", blockbit(&input, 3)), "0.801252");

        let input = str_bit_to_vec_u8(
            "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000",
        );
        assert_eq!(&format!("{:.6}", blockbit(&input, 10)), "0.706438");
    }

    #[test]
    fn runs_test() {
        let input = str_bit_to_vec_u8("1001101011");
        assert_eq!(&format!("{:.6}", runs(&input)), "0.147232");
        let input = str_bit_to_vec_u8(
            "1100100100001111110110101010001000100001011010001100001000110100110001001100011001100010100010111000",
        );
        assert_eq!(&format!("{:0.6}", runs(&input)), "0.500798");
    }
}
