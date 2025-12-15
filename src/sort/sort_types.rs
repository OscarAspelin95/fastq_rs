use crate::common::{PHRED_TO_ERROR, gc_content, mean_error_and_phred};
use minimizer_iter::MinimizerBuilder;

#[inline]
fn get_minimizers(seq: &[u8], kmer_size: usize, window_size: usize) -> Vec<(u64, usize)> {
    // Move later on.
    // assert!(window_size <= seq.len());
    // assert!(kmer_size <= window_size);

    let m_iter: Vec<(u64, usize)> = MinimizerBuilder::<u64>::new()
        .canonical()
        .minimizer_size(kmer_size)
        .width(window_size as u16)
        .iter(seq)
        .map(|(mm_seq, mm_pos, _)| (mm_seq, mm_pos))
        .collect();

    m_iter
}

#[inline]
fn is_significant_minimizer(mm_qual: &[u8], max_err: f64) -> bool {
    let mut err = 1.0;

    mm_qual.iter().for_each(|mm_phred| {
        err *= PHRED_TO_ERROR[*mm_phred as usize];
    });

    err < max_err
}

fn get_num_significant_minimizers(
    seq: &[u8],
    qual: &[u8],
    window_size: usize,
    kmer_size: usize,
    max_minimizer_error: f64,
) -> usize {
    let mms = get_minimizers(seq, kmer_size, window_size);

    let mut num_significant: usize = 0;
    // Each minimizer in the read.
    for (_, mm_pos) in mms {
        // Extract quality slice from minimizer position.
        let mm_qual = &qual[mm_pos..mm_pos + kmer_size];

        if is_significant_minimizer(mm_qual, max_minimizer_error) {
            num_significant += 1;
        }
    }

    num_significant
}

pub trait Score: Send + Sync {
    fn score(&self, seq: &[u8], qual: &[u8]) -> f64;
}

pub struct GcContent {}
pub struct ReadLength {}
pub struct ReadError {}

pub struct Minimizer {
    pub window_size: usize,
    pub kmer_size: usize,
    pub max_minimizer_error: f64,
    pub max_read_error: f64,
}

impl Minimizer {
    fn required_read_len(&self) -> usize {
        self.window_size + self.kmer_size - 1
    }
}

impl Score for GcContent {
    fn score(&self, seq: &[u8], _qual: &[u8]) -> f64 {
        gc_content(seq)
    }
}

impl Score for ReadLength {
    fn score(&self, seq: &[u8], _qual: &[u8]) -> f64 {
        seq.len() as f64
    }
}

impl Score for ReadError {
    fn score(&self, _seq: &[u8], qual: &[u8]) -> f64 {
        let (mean_error, _) = mean_error_and_phred(qual);
        mean_error
    }
}

impl Score for Minimizer {
    fn score(&self, seq: &[u8], qual: &[u8]) -> f64 {
        if self.required_read_len() <= seq.len() {
            return 0.0_f64;
        };

        let (mean_error, _) = mean_error_and_phred(qual);

        if mean_error > self.max_read_error {
            return 0.0_f64;
        }

        get_num_significant_minimizers(
            seq,
            qual,
            self.window_size,
            self.kmer_size,
            self.max_minimizer_error,
        ) as f64
    }
}
