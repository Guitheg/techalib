use criterion::BenchmarkId;
use rand::{rngs::StdRng, Rng, SeedableRng};
use techalib::indicators::bbands::{bbands, BBandsMA, DeviationMulipliers};
use techalib::types::Float;

fn bench_bbands(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("bbands");

    let cases = vec![(50_000, 20), (1_000_000, 20), (5_000_000, 20)];

    for (len, period) in cases {
        let mut rng = StdRng::seed_from_u64(period as u64);
        let data: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();

        bench_group.bench_with_input(
            BenchmarkId::new(format!("len={len}"), period),
            &period,
            |b, &period| {
                b.iter(|| {
                    let _ = bbands(
                        &data,
                        period,
                        DeviationMulipliers { up: 2.0, down: 2.0 },
                        BBandsMA::SMA,
                    );
                })
            },
        );
    }
}

criterion::criterion_group!(bench, bench_bbands);
