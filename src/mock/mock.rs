use crate::common::general_bufwriter;
use anyhow::Result;
use rand::{prelude::*, random_range, rng};
use std::path::PathBuf;

const NTS: [u8; 4] = [b'A', b'C', b'G', b'T'];

fn mock_fix_qual(fix: &Option<String>, actual_phred: u8) -> Option<Vec<u8>> {
    fix.as_ref().map(|s| {
        let mut v = Vec::with_capacity(s.len());
        for _ in 0..s.len() {
            v.push(actual_phred);
        }
        v
    })
}

pub fn fastq_mock(
    num_reads: usize,
    min_len: usize,
    max_len: usize,
    phred: u8,
    prefix_seq: Option<String>,
    suffix_seq: Option<String>,
    outfile: Option<PathBuf>,
) -> Result<()> {
    let mut writer = general_bufwriter(outfile)?;
    // +33 offset is the default.
    let actual_phred = phred + 33;

    assert!(num_reads != 0);
    assert!(min_len != 0);
    assert!(phred != 0);
    assert!(max_len > min_len);

    let prefix_qual = mock_fix_qual(&prefix_seq, actual_phred);
    let suffix_qual = mock_fix_qual(&suffix_seq, actual_phred);

    // Pre-allocate buffers for seq and qual
    let mut seq: Vec<u8> = Vec::with_capacity(max_len);
    let mut qual: Vec<u8> = Vec::with_capacity(max_len);

    let mut rng = rng();

    for i in 0..num_reads {
        seq.clear();
        qual.clear();

        let seq_len = random_range(min_len..max_len);

        seq.extend((0..seq_len).map(|_| NTS[rng.random_range(0..NTS.len())]));
        qual.resize(seq_len, actual_phred);

        assert_eq!(seq.len(), qual.len());

        // Read name.
        writer.write_all(b"@read_").unwrap();
        writer.write_all((i + 1).to_string().as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();

        // Sequence
        if let Some(s) = prefix_seq
            .as_ref() { writer.write_all(s.as_bytes()).unwrap() }
        writer.write_all(&seq).unwrap();
        if let Some(s) = suffix_seq
            .as_ref() { writer.write_all(s.as_bytes()).unwrap() }
        writer.write_all(b"\n").unwrap();
        writer.write_all(b"+\n").unwrap();

        // Qual
        if let Some(q) = prefix_qual.as_ref() { writer.write_all(q).unwrap() }
        writer.write_all(&qual).unwrap();
        if let Some(q) = suffix_qual.as_ref() { writer.write_all(q).unwrap() }
        writer.write_all(b"\n").unwrap();
    }

    writer.flush()?;

    Ok(())
}
