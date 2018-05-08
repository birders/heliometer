#[macro_use]
extern crate criterion;
extern crate heliometer;

use criterion::Criterion;
use std::io::Read;

fn bubble_sort(crit: &mut Criterion) {
    crit.bench_function("bubble sort", |b| {
        b.iter(|| {
            let mut file = std::fs::File::open("examples/bsort.bf").unwrap();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let mut fake_input = std::io::Cursor::new(vec![50, 20, 19, 255, 19, 4, 103, 57, 0, 53]);
            let mut fake_output = std::io::Cursor::new(vec![0; 10]);
            heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        })
    });
}

fn insertion_sort(crit: &mut Criterion) {
    crit.bench_function("insertion sort", |b| {
        b.iter(|| {
            let mut file = std::fs::File::open("examples/isort.bf").unwrap();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let mut fake_input = std::io::Cursor::new(vec![50, 20, 19, 255, 19, 4, 103, 57, 0, 53]);
            let mut fake_output = std::io::Cursor::new(vec![0; 10]);
            heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        })
    });
}

fn quick_sort(crit: &mut Criterion) {
    crit.bench_function("quick sort", |b| {
        b.iter(|| {
            let mut file = std::fs::File::open("examples/qsort.bf").unwrap();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let mut fake_input = std::io::Cursor::new(vec![50, 20, 19, 255, 19, 4, 103, 57, 0, 53]);
            let mut fake_output = std::io::Cursor::new(vec![0; 10]);
            heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        })
    });
}

fn rot13(crit: &mut Criterion) {
    crit.bench_function("applying rot13...", |b| {
        b.iter(|| {
            let mut file = std::fs::File::open("examples/rot13.bf").unwrap();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let mut fake_input = std::io::Cursor::new(vec![37, 85, 200, 0]);
            let mut fake_output = std::io::Cursor::new(vec![0; 4]);
            heliometer::execute(&input, &mut fake_input, &mut fake_output).unwrap();
        })
    });
}

criterion_group!(benches, bubble_sort, insertion_sort, quick_sort, rot13);
criterion_main!(benches);
