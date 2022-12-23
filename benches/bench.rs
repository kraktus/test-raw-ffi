use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use test_raw_ffi::{Person, PersonRawFFI};

fn bench_raw_ffi(c: &mut Criterion) {
    let age_name = (12, ['i', 'n', 't', 'e', 'l']);
    let p = Person::new(10, ['i', 'n', 't', 'e', 'l']);
    let raw_p = PersonRawFFI::new(10, ['i', 'n', 't', 'e', 'l']);

    {
        let mut group = c.benchmark_group("new");
        group.bench_with_input(
            BenchmarkId::new("Native", ""),
            &age_name,
            |b, age_name_ref| b.iter(|| Person::new(age_name_ref.0, age_name_ref.1)),
        );
        group.bench_with_input(
            BenchmarkId::new("RawFFI", ""),
            &age_name,
            |b, age_name_ref| b.iter(|| PersonRawFFI::new(age_name_ref.0, age_name_ref.1)),
        );
        group.finish()
    }

    {
        let mut group = c.benchmark_group("is_adult");
        group.bench_with_input(BenchmarkId::new("Native", ""), &p, |b, p_ref| {
            b.iter(|| p_ref.is_adult())
        });
        group.bench_with_input(BenchmarkId::new("RawFFI", ""), &raw_p, |b, raw_p_ref| {
            b.iter(|| raw_p_ref.is_adult())
        });
        group.finish()
    }

    {
        let mut group = c.benchmark_group("bday");
        group.bench_function("Native", |b| {
            b.iter_batched(
                || p.clone(),
                |mut person_input| {
                    person_input.bday();
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_function("RawFFI", |b| {
            b.iter_batched(
                || raw_p.clone(),
                |mut raw_person_input| {
                    raw_person_input.bday();
                },
                BatchSize::SmallInput,
            );
        });
        group.finish()
    }
    let mut group = c.benchmark_group("compute_hard");
    group.bench_with_input(BenchmarkId::new("Native", ""), &p, |b, p_ref| {
        b.iter(|| p_ref.compute_hard())
    });
    group.bench_with_input(BenchmarkId::new("RawFFI", ""), &raw_p, |b, raw_p_ref| {
        b.iter(|| raw_p_ref.compute_hard())
    });
    group.finish()
}

criterion_group!(benches, bench_raw_ffi);
criterion_main!(benches);
