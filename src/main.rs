use diatonic_st::*;

fn main() {
    let plaintext = str_to_binary("FTI UKSW");
    // let plaintext = (0..64).collect::<Vec<u8>>();

    let patterns = [PATTERN_1, PATTERN_2, PATTERN_3];

    for (i, c) in patterns.iter().enumerate() {
        for (j, c2) in patterns.iter().enumerate() {
            if i == j {
                continue;
            }

            println!("===   COMBINATION {} & {}   ===", i + 1, j + 1);
            println!("PLAINTEXT");
            print_bits(&plaintext);
            println!();

            let entry_scheme = entry_scheme(&plaintext, c);
            println!("ENTRY SCHEME");
            print_bits(&entry_scheme);
            println!();

            let retrieval_scheme = retrieval_scheme(&entry_scheme, c2);
            println!("RETRIEVAL SCHEME");
            print_bits(&retrieval_scheme);
            println!();

            println!("TESTS:");
            monobit_test(&retrieval_scheme);
        }
    }
}

fn monobit_test(bits: &[u8]) {
    let n = bits.len();
    let mut sn = 0_i32;
    for bit in bits {
        sn += if *bit == 0 { -1 } else { 1 };
    }
    let sobs = sn.abs() as f64 / (n as f64).sqrt();
    // fix erfc formula
    let erfc = sobs / 2_f64.sqrt();
    dbg!(n, sn, sobs, erfc);
}

fn str_to_binary(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for ch in input.as_bytes() {
        let binary = format!("{ch:0>8b}");
        for bit in binary.chars() {
            if bit == '0' {
                result.push(0);
                continue;
            }
            result.push(1);
        }
    }
    result
}

fn print_bits(bits: &[u8]) {
    for (i, bit) in bits.iter().enumerate() {
        print!("{bit: >3},");
        if i > 0 && (i + 1) % 8 == 0 {
            println!();
        }
    }
}
