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
