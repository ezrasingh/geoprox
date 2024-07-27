use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use geoprox_core::cache::SpatialIndex;
use rand::prelude::*;

fn random_geohash(rng: &mut ThreadRng) -> String {
    geohash::encode(
        [rng.gen_range(-180f64..180f64), rng.gen_range(-90f64..90f64)].into(),
        SpatialIndex::DEFAULT_DEPTH,
    )
    .unwrap()
}

fn seed_index(size: &i32, rng: &mut ThreadRng) -> SpatialIndex<String> {
    let mut geo_index = SpatialIndex::new(*size as usize);
    for n in 0..*size {
        geo_index.place_resource(&n.to_string(), &random_geohash(rng));
    }
    geo_index
}

const UNIT_CAPACITY: i32 = 10;
const CAPACITY_RANGE: [i32; 6] = [
    UNIT_CAPACITY,
    UNIT_CAPACITY * 10,
    UNIT_CAPACITY * 100,
    UNIT_CAPACITY * 1000,
    UNIT_CAPACITY * 10_000,
    UNIT_CAPACITY * 100_000,
];

fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index/insert");
    let mut rng = rand::thread_rng();
    for capacity in CAPACITY_RANGE.iter() {
        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(capacity),
            capacity,
            |b, &capacity| {
                b.iter(|| black_box(seed_index(&capacity, &mut rng)));
            },
        );
    }
    group.finish();
}

fn query_range_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index/range_query");
    let mut rng = rand::thread_rng();

    for capacity in CAPACITY_RANGE.iter() {
        let geo_index: SpatialIndex<String> = seed_index(&capacity, &mut rng);

        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, _| {
            b.iter(|| {
                let ((lat, lng), range) = (
                    (
                        rng.gen_range(-90f64..90f64).into(),
                        rng.gen_range(-180f64..180f64).into(),
                    ),
                    rng.gen_range(50..500).into(),
                );
                geo_index.search([lat, lng], &range, None)
            });
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(60));
    targets = insert_benchmark, query_range_benchmark
);
criterion_main!(benches);
