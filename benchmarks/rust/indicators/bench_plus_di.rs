use criterion::BenchmarkId;
use rand::{rngs::StdRng, Rng, SeedableRng};
use techalib::{indicators::plus_di::plus_di, types::Float};

fn bench_plus_di(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("plus_di");

    let cases = vec![(50_000,), (1_000_000,)];

    for (len,) in cases {
        let mut rng = StdRng::seed_from_u64(42);

        let high: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();
        let low: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();
        let close: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();
        let period: usize = 14;

        bench_group.bench_with_input(BenchmarkId::new("plus_di", len), &len, |b, &_len| {
            b.iter(|| {
                let _ = plus_di(&high, &low, &close, period);
            })
        });
    }
}

criterion::criterion_group!(bench, bench_plus_di);
