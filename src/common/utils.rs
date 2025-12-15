use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PHRED_TO_ERROR: [f64; 126] = {
        let mut error_lookup: [f64; 126] = [1.0; 126];

        for i in 0..126 {
            if i >= 33 {
                error_lookup[i] = 10_f64.powf(-((i - 33) as f64) / 10.0);
            };
        }

        return error_lookup;
    };
}

#[inline]
pub fn error_to_phred(error: f64) -> u8 {
    (-10_f64 * error.log10()) as u8
}

#[inline]
pub fn mean_error_and_phred(qual: &[u8]) -> (f64, u8) {
    let error_sum: f64 = qual
        .iter()
        .map(|phred| {
            PHRED_TO_ERROR[*phred as usize]
        })
        .sum::<f64>();

    let error_mean = error_sum / qual.len() as f64;
    (error_mean, error_to_phred(error_mean))
}

#[inline]
pub fn mean_len(lengths: &[usize]) -> usize {
    lengths.iter().sum::<usize>() / lengths.len()
}

#[inline]
pub fn nucleotide_counts(seq: &[u8]) -> (HashMap<&u8, usize>, usize, usize) {
    let mut canonical: HashMap<&u8, usize> = HashMap::with_capacity(4);

    // Counts of non-canonical nucleotides.
    let mut softmasked_count: usize = 0;
    let mut ambiguous_count: usize = 0;

    for nt in seq.iter() {
        match nt {
            // Canonical.
            b'A' | b'C' | b'G' | b'T' => {
                canonical
                    .entry(nt)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            // Softmasked.
            b'a' | b'c' | b'g' | b't' => {
                softmasked_count += 1;
            }

            // Ambiguous
            _ => {
                ambiguous_count += 1;
            }
        }
    }

    (canonical, softmasked_count, ambiguous_count)
}

#[inline]
pub fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    let reverse_complement: Vec<u8> = seq
        .iter()
        .rev()
        .map(|nt| match nt {
            // Canonical
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            // Ambiguous
            b'R' => b'Y', // AG <-> CT
            b'Y' => b'R', // CT <-> AG
            b'S' => b'S', // GC
            b'W' => b'W', // AT
            b'K' => b'M', // GT <-> AC
            b'M' => b'K', // AC <-> GT
            b'B' => b'V', // CGT <-> ACG
            b'D' => b'H', // AGT <-> ACT
            b'H' => b'D', // ACT <-> AGT
            b'V' => b'B', // ACG <-> CGT
            b'N' => b'N', //

            _ => panic!("Invalid nucleotide {}", *nt as char),
        })
        .collect();

    reverse_complement
}
