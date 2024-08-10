use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use geoprox_core::{cache::SpatialIndex, models::LatLngCoord};
use hashbrown::HashSet;
use rand::prelude::*;

const UNIT_CAPACITY: i32 = 10;
const CAPACITY_RANGE: [i32; 5] = [
    UNIT_CAPACITY,
    UNIT_CAPACITY * 10,
    UNIT_CAPACITY * 100,
    UNIT_CAPACITY * 1000,
    UNIT_CAPACITY * 10_000,
];
const MAX_COUNT: usize = 100;
const DEFAULT_DEPTH: usize = 6;

fn random_geohash(rng: &mut ThreadRng) -> String {
    geohash::encode(
        [rng.gen_range(-180f64..180f64), rng.gen_range(-90f64..90f64)].into(),
        DEFAULT_DEPTH,
    )
    .unwrap()
}

fn random_query(rng: &mut ThreadRng) -> (LatLngCoord, f64) {
    let ((lat, lng), range) = (
        (
            rng.gen_range(-90f64..90f64).into(),
            rng.gen_range(-180f64..180f64).into(),
        ),
        rng.gen_range(50..500).into(),
    );
    ([lat, lng], range)
}

fn seed_index(size: &i32, rng: &mut ThreadRng) -> SpatialIndex {
    let mut geo_index = SpatialIndex::new(*size as usize);
    (0..*size).for_each(|n| {
        geo_index.insert(&n.to_string(), &random_geohash(rng));
    });
    geo_index
}

fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index/insert");
    let mut rng = rand::thread_rng();
    for capacity in CAPACITY_RANGE.iter() {
        let data: Vec<(String, String)> = (0..*capacity)
            .map(|n| (n.to_string(), random_geohash(&mut rng)))
            .collect();

        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(capacity),
            capacity,
            |b, &capacity| {
                b.iter_batched(
                    || (SpatialIndex::new(capacity as usize), data.clone()),
                    |(mut geo_index, data)| {
                        data.iter()
                            .for_each(|(key, ghash)| black_box(geo_index.insert(key, ghash)))
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("many", capacity),
            capacity,
            |b, &capacity| {
                b.iter_batched(
                    || (SpatialIndex::new(capacity as usize), data.clone()),
                    |(mut geo_index, data)| black_box(geo_index.insert_many(data)),
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

fn remove_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index/remove");
    for capacity in CAPACITY_RANGE.iter() {
        let mut rng = rand::thread_rng();

        group.throughput(Throughput::Elements(*capacity as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(capacity),
            capacity,
            |b, &capacity| {
                b.iter_batched(
                    || seed_index(&capacity, &mut rng),
                    |mut geo_index| {
                        (0..capacity).for_each(|n| {
                            let key = n.to_string();
                            black_box(geo_index.remove(&key));
                        });
                    },
                    BatchSize::LargeInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("many", capacity),
            capacity,
            |b, &capacity| {
                b.iter_batched(
                    || {
                        let geo_index = seed_index(&capacity, &mut rng);
                        let data: HashSet<String> = (0..capacity).map(|n| n.to_string()).collect();
                        (geo_index, data)
                    },
                    |(mut geo_index, data)| black_box(geo_index.remove_many(data)),
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

fn query_range_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index/range_query");
    let mut rng = rand::thread_rng();

    for capacity in CAPACITY_RANGE.iter() {
        let geo_index: SpatialIndex = seed_index(capacity, &mut rng);

        group.throughput(Throughput::Elements(*capacity as u64));

        group.bench_with_input(BenchmarkId::from_parameter(capacity), capacity, |b, _| {
            b.iter_batched(
                || random_query(&mut rng),
                |([lat, lng], range)| {
                    black_box(geo_index.search([lat, lng], range, MAX_COUNT, false, DEFAULT_DEPTH))
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("sorted", capacity), capacity, |b, _| {
            b.iter_batched(
                || random_query(&mut rng),
                |([lat, lng], range)| {
                    black_box(geo_index.search([lat, lng], range, MAX_COUNT, true, DEFAULT_DEPTH))
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = insert_benchmark, remove_benchmark, query_range_benchmark
);
criterion_main!(benches);
