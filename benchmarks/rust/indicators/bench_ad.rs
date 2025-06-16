use criterion::BenchmarkId;
use rand::{rngs::StdRng, Rng, SeedableRng};
use techalib::{indicators::ad::ad, types::Float};

fn bench_ad(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("ad");

    let cases = vec![(50_000, 30), (1_000_000, 30)];

    for (len, period) in cases {
        let mut rng = StdRng::seed_from_u64(period as u64);

        let high_prices: Vec<Float> = (0..len).map(|_| rng.random_range(60.0..100.0)).collect();
        let close_prices: Vec<Float> = (0..len).map(|_| rng.random_range(40.0..60.0)).collect();
        let low_prices: Vec<Float> = (0..len).map(|_| rng.random_range(12.0..40.0)).collect();
        let volume: Vec<Float> = (0..len).map(|_| rng.random_range(100.0..1000.0)).collect();

        bench_group.bench_with_input(BenchmarkId::new("ad", len), &len, |b, &_len| {
            b.iter(|| {
                let _ = ad(&high_prices, &low_prices, &close_prices, &volume);
            })
        });
    }
}

criterion::criterion_group!(bench, bench_ad);
