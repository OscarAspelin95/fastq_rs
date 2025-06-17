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
/// It is probably not ideal to return a usize
/// since this will cause rounding. A better
/// approach would be to return a rounded i8
/// (max value 256 so we are safe). However,
/// This probably required more computation power.
pub fn error_to_phred(error: f64) -> u8 {
    return (-10_f64 * error.log10()) as u8;
}
