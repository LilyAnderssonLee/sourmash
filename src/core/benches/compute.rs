#[macro_use]
extern crate criterion;

use std::fs::File;
use std::io::{Cursor, Read};

use needletail::parse_sequence_reader;
use sourmash::cmd::ComputeParameters;
use sourmash::signature::Signature;

use criterion::Criterion;

fn add_sequence(c: &mut Criterion) {
    let cp = ComputeParameters::default();
    let template_sig = Signature::from_params(&cp);

    let mut data: Vec<u8> = vec![];
    let mut f = File::open("../../tests/test-data/ecoli.genes.fna").unwrap();
    let _ = f.read_to_end(&mut data);

    let data = data.repeat(10);

    let data_upper = data.to_ascii_uppercase();
    let data_lower = data.to_ascii_lowercase();
    let data_errors: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(i, x)| if i % 89 == 1 { 'N' as u8 } else { *x })
        .collect();

    let mut group = c.benchmark_group("add_sequence");
    group.sample_size(10);

    group.bench_function("valid", |b| {
        b.iter(|| {
            let fasta_data = Cursor::new(data_upper.clone());
            let mut sig = template_sig.clone();
            parse_sequence_reader(
                fasta_data,
                |_| {},
                |rec| {
                    sig.add_sequence(&rec.seq, false).unwrap();
                },
            )
            .unwrap();
        });
    });

    group.bench_function("lowercase", |b| {
        b.iter(|| {
            let fasta_data = Cursor::new(data_lower.clone());
            let mut sig = template_sig.clone();
            parse_sequence_reader(
                fasta_data,
                |_| {},
                |rec| {
                    sig.add_sequence(&rec.seq, false).unwrap();
                },
            )
            .unwrap();
        });
    });

    group.bench_function("invalid kmers", |b| {
        b.iter(|| {
            let fasta_data = Cursor::new(data_errors.clone());
            let mut sig = template_sig.clone();
            parse_sequence_reader(
                fasta_data,
                |_| {},
                |rec| {
                    sig.add_sequence(&rec.seq, true).unwrap();
                },
            )
            .unwrap();
        });
    });

    group.bench_function("force with valid kmers", |b| {
        b.iter(|| {
            let fasta_data = Cursor::new(data_upper.clone());
            let mut sig = template_sig.clone();
            parse_sequence_reader(
                fasta_data,
                |_| {},
                |rec| {
                    sig.add_sequence(&rec.seq, true).unwrap();
                },
            )
            .unwrap();
        });
    });
}

criterion_group!(compute, add_sequence);
criterion_main!(compute);
