use std::collections::BTreeMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use btreemap_range_bench_1::{find_bounds_range, find_bounds_custom};

/*
fn bench_find_bounds(c: &mut Criterion) {
    let bm = BTreeMap::from([
        (1, "Defined".to_string()),
        (4, "Started".to_string()),
        (6, "LockedIn".to_string()),
        (8, "Active".to_string()),
    ]);

    let index = [0, 5, 9, 6];

    let mut group = c.benchmark_group("find_bounds");

    group.bench_function("using range", |b| b.iter(|| {
        find_bounds_range(black_box(&bm), black_box(index[0]));
        find_bounds_range(black_box(&bm), black_box(index[1]));
        find_bounds_range(black_box(&bm), black_box(index[2]));
        find_bounds_range(black_box(&bm), black_box(index[3]));
    }));

    group.bench_function("using custom impl", |b| b.iter(|| {
        find_bounds_custom(black_box(&bm), black_box(index[0]));
        find_bounds_custom(black_box(&bm), black_box(index[1]));
        find_bounds_custom(black_box(&bm), black_box(index[2]));
        find_bounds_custom(black_box(&bm), black_box(index[3]));
    }));

    group.finish();

}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RandomData {
    map: BTreeMap<u32, String>,
    indexes: Vec<u32>,
}

fn bench_find_bounds_from_json(c: &mut Criterion) {

    let data = std::fs::read_to_string("data.json").expect("Unable to read file");
    let rd: RandomData = serde_json::from_str(&data).expect("Unable to parse");
    let bm = &rd.map;
    let indexes = &rd.indexes;

    let mut group = c.benchmark_group("find_bounds");

    group.bench_function("using range", |b| b.iter(|| {

        for i in indexes {
            find_bounds_range(black_box(&bm), black_box(*i));
        }
    }));

    group.bench_function("using custom impl", |b| b.iter(|| {
        for i in indexes {
            find_bounds_custom(black_box(&bm), black_box(*i));
        }
    }));

    group.finish();

}

// criterion_group!(benches, bench_find_bounds);
criterion_group!(benches, bench_find_bounds_from_json);
criterion_main!(benches);
*/

// criterion_group!(benches, bench_find_bounds);
// criterion_group!(benches, bench_find_bounds_from_json);

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RandomData {
    map: BTreeMap<u32, String>,
    indexes: Vec<u32>,
}

fn bench_find_bounds_from_json_files(c: &mut Criterion) {

    let data_10 = std::fs::read_to_string("data_10.json").expect("Unable to read file");
    let rd_10: RandomData = serde_json::from_str(&data_10).expect("Unable to parse");
    let bm_10 = &rd_10.map;
    let indexes_10 = &rd_10.indexes;

    let data_25 = std::fs::read_to_string("data_25.json").expect("Unable to read file");
    let rd_25: RandomData = serde_json::from_str(&data_25).expect("Unable to parse");
    let bm_25 = &rd_25.map;
    let indexes_25 = &rd_25.indexes;

    let data_50 = std::fs::read_to_string("data_50.json").expect("Unable to read file");
    let rd_50: RandomData = serde_json::from_str(&data_50).expect("Unable to parse");
    let bm_50 = &rd_50.map;
    let indexes_50 = &rd_50.indexes;

    let data_100 = std::fs::read_to_string("data_100.json").expect("Unable to read file");
    let rd_100: RandomData = serde_json::from_str(&data_100).expect("Unable to parse");
    let bm_100 = &rd_100.map;
    let indexes_100 = &rd_100.indexes;

    let data_250 = std::fs::read_to_string("data_250.json").expect("Unable to read file");
    let rd_250: RandomData = serde_json::from_str(&data_250).expect("Unable to parse");
    let bm_250 = &rd_250.map;
    let indexes_250 = &rd_250.indexes;

    let data_500 = std::fs::read_to_string("data_500.json").expect("Unable to read file");
    let rd_500: RandomData = serde_json::from_str(&data_500).expect("Unable to parse");
    let bm_500 = &rd_500.map;
    let indexes_500 = &rd_500.indexes;

    let data_2000 = std::fs::read_to_string("data_2000.json").expect("Unable to read file");
    let rd_2000: RandomData = serde_json::from_str(&data_2000).expect("Unable to parse");
    let bm_2000 = &rd_2000.map;
    let indexes_2000 = &rd_2000.indexes;

    let mut group = c.benchmark_group("find_bounds");

    // Note: Too much diff for > 200 items
    for count in [10, 25, 50, 100].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::new("bounds_range", count), count, |b, &count| {
            b.iter(|| {

                let (bm, indexes) = match count {
                    10 => (&bm_10, indexes_10),
                    25 => (&bm_25, indexes_25),
                    50 => (&bm_50, indexes_50),
                    100 => (&bm_100, indexes_100),
                    250 => (&bm_250, indexes_250),
                    500 => (&bm_500, indexes_500),
                    2000 => (&bm_2000, indexes_2000),
                    _ => { unimplemented!() }
                };

                for i in indexes {
                    find_bounds_range(black_box(bm), black_box(*i));
                }
            });
        });
        group.bench_with_input(BenchmarkId::new("bounds_custom", count), count, |b, &count| {
            b.iter(|| {
                let (bm, indexes) = match count {
                    10 => (&bm_10, indexes_10),
                    25 => (&bm_25, indexes_25),
                    50 => (&bm_50, indexes_50),
                    100 => (&bm_100, indexes_100),
                    250 => (&bm_250, indexes_250),
                    500 => (&bm_500, indexes_500),
                    2000 => (&bm_2000, indexes_2000),
                    _ => { unimplemented!() }
                };

                for i in indexes {
                    find_bounds_custom(black_box(bm), black_box(*i));
                }
            });
        });
    }

    group.finish();


}

criterion_group!(benches, bench_find_bounds_from_json_files);
criterion_main!(benches);
