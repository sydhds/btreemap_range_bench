use std::collections::BTreeMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use serde::{Serialize, Deserialize};

use btreemap_range_bench_1::{find_bounds_range, find_bounds_custom, find_bounds_vec, find_bounds_vec_no_sort};

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
    let mut v_10: Vec<(u32, String)> = bm_10
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_10_s = v_10.clone();
    v_10_s.sort_by(|a, b| a.0.cmp(&b.0));

    let data_25 = std::fs::read_to_string("data_25.json").expect("Unable to read file");
    let rd_25: RandomData = serde_json::from_str(&data_25).expect("Unable to parse");
    let bm_25 = &rd_25.map;
    let indexes_25 = &rd_25.indexes;
    let mut v_25: Vec<(u32, String)> = bm_25
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_25_s = v_25.clone();
    v_25_s.sort_by(|a, b| a.0.cmp(&b.0));

    let data_50 = std::fs::read_to_string("data_50.json").expect("Unable to read file");
    let rd_50: RandomData = serde_json::from_str(&data_50).expect("Unable to parse");
    let bm_50 = &rd_50.map;
    let indexes_50 = &rd_50.indexes;
    let mut v_50: Vec<(u32, String)> = bm_50
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_50_s = v_50.clone();
    v_50_s.sort_by(|a, b| a.0.cmp(&b.0));

    let data_100 = std::fs::read_to_string("data_100.json").expect("Unable to read file");
    let rd_100: RandomData = serde_json::from_str(&data_100).expect("Unable to parse");
    let bm_100 = &rd_100.map;
    let indexes_100 = &rd_100.indexes;
    let mut v_100: Vec<(u32, String)> = bm_100
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_100_s = v_100.clone();
    v_100_s.sort_by(|a, b| a.0.cmp(&b.0));

    let data_250 = std::fs::read_to_string("data_250.json").expect("Unable to read file");
    let rd_250: RandomData = serde_json::from_str(&data_250).expect("Unable to parse");
    let bm_250 = &rd_250.map;
    let indexes_250 = &rd_250.indexes;
    let mut v_250: Vec<(u32, String)> = bm_250
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_250_s = v_250.clone();
    v_250_s.sort_by(|a, b| a.0.cmp(&b.0));

    let data_500 = std::fs::read_to_string("data_500.json").expect("Unable to read file");
    let rd_500: RandomData = serde_json::from_str(&data_500).expect("Unable to parse");
    let bm_500 = &rd_500.map;
    let indexes_500 = &rd_500.indexes;
    let mut v_500: Vec<(u32, String)> = bm_500
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_500_s = v_500.clone();
    v_500_s.sort_by(|a, b| a.0.cmp(&b.0));

    let data_2000 = std::fs::read_to_string("data_2000.json").expect("Unable to read file");
    let rd_2000: RandomData = serde_json::from_str(&data_2000).expect("Unable to parse");
    let bm_2000 = &rd_2000.map;
    let indexes_2000 = &rd_2000.indexes;
    let mut v_2000: Vec<(u32, String)> = bm_2000
        .iter()
        .map(|(version, st)| (*version, st.clone()))
        .collect();
    let mut v_2000_s = v_2000.clone();
    v_2000_s.sort_by(|a, b| a.0.cmp(&b.0));

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

        group.bench_with_input(BenchmarkId::new("bounds_vec", count), count, |b, &count| {
            b.iter(|| {
                let (v, indexes) = match count {
                    10 => (&mut v_10, indexes_10),
                    25 => (&mut v_25, indexes_25),
                    50 => (&mut v_50, indexes_50),
                    100 => (&mut v_100, indexes_100),
                    250 => (&mut v_250, indexes_250),
                    500 => (&mut v_500, indexes_500),
                    2000 => (&mut v_2000, indexes_2000),
                    _ => { unimplemented!() }
                };

                for i in indexes {
                    find_bounds_vec(black_box(v), black_box(*i));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("bounds_vec_no_sort", count), count, |b, &count| {
            b.iter(|| {
                let (v, indexes) = match count {
                    10 => (&v_10_s, indexes_10),
                    25 => (&v_25_s, indexes_25),
                    50 => (&v_50_s, indexes_50),
                    100 => (&v_100_s, indexes_100),
                    250 => (&v_250_s, indexes_250),
                    500 => (&v_500_s, indexes_500),
                    2000 => (&v_2000_s, indexes_2000),
                    _ => { unimplemented!() }
                };

                for i in indexes {
                    find_bounds_vec_no_sort(black_box(v), black_box(*i));
                }
            });
        });


    }

    group.finish();


}

criterion_group!(benches, bench_find_bounds_from_json_files);
criterion_main!(benches);
