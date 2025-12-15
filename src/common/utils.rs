use lazy_static::lazy_static;
use rstest::rstest;
use std::collections::HashMap;

const PHRED_OFFSET: usize = 33;

lazy_static! {
    pub static ref PHRED_TO_ERROR: [f64; 126] = {
        let mut error_lookup: [f64; 126] = [1.0; 126];

        for i in 0..126 {
            if i >= 33 {
                error_lookup[i] = 10_f64.powf(-((i - PHRED_OFFSET) as f64) / 10.0);
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
    if qual.is_empty() {
        return (0.0, 0);
    }

    let error_sum: f64 = qual
        .iter()
        .map(|phred| PHRED_TO_ERROR[*phred as usize])
        .sum::<f64>();

    let error_mean = error_sum / qual.len() as f64;
    (error_mean, error_to_phred(error_mean))
}

#[inline]
pub fn mean_len(lengths: &[usize]) -> usize {
    if lengths.len() == 0 {
        return 0;
    }

    lengths.iter().sum::<usize>() / lengths.len()
}

#[inline]
pub fn nucleotide_counts(seq: &[u8]) -> (HashMap<&u8, usize>, usize, usize) {
    if seq.len() == 0 {
        return (HashMap::new(), 0, 0);
    }

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
/// Using a pre-allocated vector with a .extend()
/// might improve performance slightly, not sure.
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

#[inline]
pub fn gc_content(seq: &[u8]) -> f64 {
    let mut num_bases: usize = 0;
    let mut gc_count: usize = 0;

    for nt in seq {
        num_bases += 1;

        if nt == &b'G' || nt == &b'C' || nt == &b'g' || nt == &b'c' {
            gc_count += 1;
        };
    }

    match gc_count {
        0 => 0.0,
        _ => gc_count as f64 / num_bases as f64,
    }
}

#[rstest]
#[case(b"", 0.0_f64)]
#[case(b"A", 0.0_f64)]
#[case(b"G", 1.0_f64)]
#[case(b"ATCG", 0.5_f64)]
#[case(b"AATTC", 1.0_f64 / 5.0_f64)]
#[case(b"AAAAAAG", 1.0_f64 / 7.0_f64)]
fn test_gc_content(#[case] seq: &[u8], #[case] expected: f64) {
    assert_eq!(gc_content(seq), expected);
}

#[rstest]
#[case(30 + PHRED_OFFSET, 0.001)]
#[case(40 + PHRED_OFFSET, 0.0001)]
#[case(50 + PHRED_OFFSET, 0.00001)]
fn test_phred_to_error(#[case] phred: usize, #[case] expected: f64) {
    assert_eq!(PHRED_TO_ERROR[phred], expected);
}

#[rstest]
#[case(vec![], 0)]
#[case(vec![10, 20, 30], 20)]
fn test_mean_len(#[case] lengths: Vec<usize>, #[case] expected: usize) {
    assert_eq!(mean_len(&lengths), expected);
}

#[rstest]
#[case(b"A", vec![b'T'])]
#[case(b"ATA", vec![b'T', b'A', b'T'])]
fn test_reverse_complement(#[case] seq: &[u8], #[case] expected: Vec<u8>) {
    assert_eq!(reverse_complement(seq), expected);
}

#[rstest]
#[case(b"N", vec![b'N'])]
#[case(b"TNT", vec![b'A', b'N', b'A'])]
fn test_reverse_complement_ambig(#[case] seq: &[u8], #[case] expected: Vec<u8>) {
    assert_eq!(reverse_complement(seq), expected);
}

#[rstest]
#[case(b"", 0, 0, vec![])]
#[case(b"aaAA", 2, 0, vec![(b'A', 2)])]
#[case(b"aaAAttTTccCCggGGNN", 8, 2, vec![(b'A', 2), (b'T', 2), (b'C', 2), (b'G', 2)])]
fn test_nucleotide_counts(
    #[case] seq: &[u8],
    #[case] expected_softmasked: usize,
    #[case] expected_hardmasked: usize,
    #[case] expected_counts: Vec<(u8, usize)>,
) {
    let (counts, softmasked, hardmasked) = nucleotide_counts(seq);

    assert_eq!(softmasked, expected_softmasked);
    assert_eq!(hardmasked, expected_hardmasked);

    for (nt, expected_count) in expected_counts.into_iter() {
        assert_eq!(
            counts
                .get(&nt)
                .expect("Nucleotide {nt} has incorrect count {expected_count}"),
            &expected_count
        );
    }
}
