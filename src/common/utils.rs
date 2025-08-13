use lazy_static::lazy_static;

lazy_static! {
    pub static ref PHRED_TO_ERROR: [f64; 126] = {
        let mut error_lookup: [f64; 126] = [1.0; 126];

        for i in 0..126 {
            if i >= 33 {
                error_lookup[i] = 10_f64.powf(-1.0 * ((i - 33) as f64) / 10.0);
            };
        }

        return error_lookup;
    };
}

#[inline]
pub fn error_to_phred(error: f64) -> u8 {
    return (-10_f64 * error.log10()) as u8;
}

#[inline]
pub fn mean_error_and_phred(qual: &[u8]) -> (f64, u8) {
    let error_sum: f64 = qual
        .iter()
        .map(|phred| {
            return PHRED_TO_ERROR[*phred as usize];
        })
        .sum::<f64>();

    let error_mean = error_sum / qual.len() as f64;
    return (error_mean, error_to_phred(error_mean));
}

#[inline]
pub fn mean_len(lengths: &[usize]) -> usize {
    return lengths.iter().sum::<usize>() / lengths.len();
}
