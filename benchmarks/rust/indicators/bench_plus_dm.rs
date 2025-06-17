use criterion::BenchmarkId;
use rand::{rngs::StdRng, Rng, SeedableRng};
use techalib::{indicators::plus_dm::plus_dm, types::Float};

fn bench_plus_dm(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("plus_dm");

    let cases = vec![(50_000, 30), (1_000_000, 30)];

    for (len, period) in cases {
        let mut rng = StdRng::seed_from_u64(period as u64);

        let high_prices: Vec<Float> = (0..len).map(|_| rng.random_range(12.0..100.0)).collect();
        let low_prices: Vec<Float> = high_prices
            .iter()
            .map(|&h| h - rng.random_range(0.5..10.0))
            .collect();

        bench_group.bench_with_input(
            BenchmarkId::new(format!("len={len}"), period),
            &period,
            |b, &period| {
                b.iter(|| {
                    let _ = plus_dm(&high_prices, &low_prices, period);
                })
            },
        );
    }
}

criterion::criterion_group!(bench, bench_plus_dm);
