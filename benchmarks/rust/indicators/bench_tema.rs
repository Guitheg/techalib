use criterion::BenchmarkId;
use rand::{rngs::StdRng, Rng, SeedableRng};
use techalib::{indicators::tema::tema, types::Float};

fn bench_tema(c: &mut criterion::Criterion) {
    let mut bench_group = c.benchmark_group("tema");

    let cases = vec![(50_000, 12), (1_000_000, 30)];

    for (len, period) in cases {
        let mut rng = StdRng::seed_from_u64(period as u64);
        let data: Vec<Float> = (0..len).map(|_| rng.random_range(0.0..100.0)).collect();

        bench_group.bench_with_input(
            BenchmarkId::new(format!("len={len}"), period),
            &period,
            |b, &period| {
                b.iter(|| {
                    let _ = tema(&data, period, None);
                })
            },
        );
    }
}

criterion::criterion_group!(bench, bench_tema);
