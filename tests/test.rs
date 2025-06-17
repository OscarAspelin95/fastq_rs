use fastq_rs::fastq::utils::{PHRED_TO_ERROR, error_to_phred};
use rstest::rstest;

#[rstest]
// Base cases
#[case(10)]
#[case(20)]
#[case(30)]
#[case(40)]
// Arbitrary phreds
#[case(15)]
#[case(17)]
#[case(27)]
#[case(31)]
#[case(39)]
#[case(47)]

/// Test that converting phred -> error -> phred is consistent.
fn test_error_to_phred(#[case] phred: u8) {
    let error = PHRED_TO_ERROR[(phred + 33) as usize];

    let reverted_phred = error_to_phred(error);

    assert_eq!(phred, reverted_phred);
}
