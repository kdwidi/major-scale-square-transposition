use major_scale_square_transposition::*;

const PLAINTEXT: &str = "FTI UKSW";

fn main() {
    real_deal_mode();
}

#[allow(dead_code)]
fn real_deal_mode() {
    let plaintext = str_to_binary(PLAINTEXT);
    // #[rustfmt::skip]
    // let plaintext: Vec<u8> = vec![
    //     1, 1, 1, 1, 1, 1, 1, 1,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     1, 1, 1, 1, 1, 1, 1, 1,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     1, 1, 1, 1, 1, 1, 1, 1,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     1, 1, 1, 1, 1, 1, 1, 1,
    //     0, 0, 0, 0, 0, 0, 0, 1,
    // ];

    let mut test_result = vec![];
    for (i, e) in ENTRY_SCHEMES.iter().enumerate() {
        for (j, r) in RETRIEVAL_SCHEMES.iter().enumerate() {
            let entry = entry_scheme_transpose(&plaintext, e);
            let retrieval = retrieval_scheme_transpose(&entry, r);

            let monobit = monobit(&retrieval);
            let blockbit = blockbit(&retrieval, 8);
            let runs = runs(&retrieval);

            test_result.push((i, j, monobit, blockbit, runs));
        }
    }

    test_result.sort_by(|(_, _, _, _, a), (_, _, _, _, b)| b.total_cmp(a));

    #[allow(clippy::needless_range_loop)]
    for i in 0..4 {
        let (ei, ri, monobit, blockbit, runs) = test_result[i];
        let entry = entry_scheme_transpose(&plaintext, &ENTRY_SCHEMES[ei]);
        let retrieval = retrieval_scheme_transpose(&entry, &RETRIEVAL_SCHEMES[ri]);
        print_full(&plaintext, &entry, &retrieval, ei, ri);

        println!(
            "plainbit  : {}",
            plaintext
                .iter()
                .enumerate()
                .map(|(i, b)| if (i + 1) % 8 == 0 {
                    format!("{b} ")
                } else {
                    b.to_string()
                })
                .collect::<String>()
        );
        println!(
            "transposed: {}",
            retrieval
                .iter()
                .enumerate()
                .map(|(i, b)| if (i + 1) % 8 == 0 {
                    format!("{b} ")
                } else {
                    b.to_string()
                })
                .collect::<String>()
        );
        println!();

        println!("test results:");
        println!("=> monobit : {:.6}", monobit);
        println!("=> blockbit: {:.6}", blockbit);
        println!("=> runs    : {:.6}\n\n", runs);
    }
}

#[allow(dead_code)]
fn index_mode(entry: &[u8; 64], retrieval: &[u8; 64], ei: usize, ri: usize) {
    let plaintext = (0..64).collect::<Vec<u8>>();
    let entry = entry_scheme_transpose(&plaintext, entry);
    let retrieval = retrieval_scheme_transpose(&entry, retrieval);
    print_full(&plaintext, &entry, &retrieval, ei, ri);
}

fn print_full(plaintext: &[u8], entry: &[u8], retrieval: &[u8], ei: usize, ri: usize) {
    println!(
        "plaintext \"FTI UKSW\"              \
        entry {: >2}                          \
        retrieval {: >2}",
        ei + 1,
        ri + 1,
    );
    for i in 0..8 {
        for j in 0..8 {
            print!("{: >2}, ", plaintext[(i * 8) + j]);
        }
        print!("  ");
        for j in 0..8 {
            print!("{: >2}, ", entry[(i * 8) + j]);
        }
        print!("  ");
        // let mut byte = String::new();
        for j in 0..8 {
            print!("{: >2}, ", retrieval[(i * 8) + j]);
            // byte.push_str(&format!("{}", retrieval[(i * 8) + j]));
        }
        // if let Ok(num) = u8::from_str_radix(&byte, 2) {
        //     print!("  => {byte} | {num: >3}",);
        // }
        println!();
    }
    println!();
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

pub fn print_bits(bits: &[u8]) {
    for (i, bit) in bits.iter().enumerate() {
        print!("{bit},");
        if i > 0 && (i + 1) % 8 == 0 {
            println!();
        }
    }
}

#[allow(dead_code)]
fn print_indexes(entry: &[u8], retrieval: &[u8]) {
    for (i, e) in entry.iter().enumerate() {
        print!("{},", e + 1);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }
    println!();

    let mut result = vec![0; 64];
    for (i, e) in entry.iter().enumerate() {
        result[retrieval[i] as usize] = *e;
        // print!("{: >2} ({: >2}), ", e + 1, retrieval[i] + 1);
        print!("{} ({}), ", e + 1, retrieval[i] + 1);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }

    for (i, r) in result.iter().enumerate() {
        print!("{},", r + 1);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }

    for i in &result {
        println!("{}", i + 1);
    }
}
