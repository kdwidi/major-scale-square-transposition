pub use patterns::*;

mod patterns;

const BIT_SIZE: usize = 64;

pub fn entry_scheme(bits: &[u8], pattern: &Pattern) -> Vec<u8> {
    let mut result = Vec::new();
    for index in pattern {
        result.push(bits[*index as usize]);
    }
    result
}

pub fn retrieval_scheme(bits: &[u8], pattern: &Pattern) -> Vec<u8> {
    let mut result = vec![0; 64];
    for (i, index) in pattern.iter().enumerate() {
        result[*index as usize] = bits[i];
    }
    result
}

pub fn print_all(plaintext: &[u8], entry: &[u8], retrieval: &[u8]) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{: >2}, ", plaintext[(i * 8) + j]);
        }
        print!("  ");
        for j in 0..8 {
            print!("{: >2}, ", entry[(i * 8) + j]);
        }
        print!("  ");
        let mut byte = String::new();
        for j in 0..8 {
            print!("{: >2}, ", retrieval[(i * 8) + j]);
            byte.push_str(&format!("{}", retrieval[(i * 8) + j]));
        }
        if let Ok(num) = u8::from_str_radix(&byte, 2) {
            print!("  => {byte} | {num: >3}",);
        }
        println!();
    }
    println!();
}
