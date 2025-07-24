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
