// Generate fake data (in json files) for the benchmark

use std::collections::BTreeMap;
use std::fs::File;
use fake::{Dummy, Fake, Faker};
use rand::rngs::StdRng;
use rand::SeedableRng;
// using `faker` module with locales
use fake::faker::name::raw::*;
use fake::locales::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct RandomData {
    map: BTreeMap<u32, String>,
    indexes: Vec<u32>,
}

fn gen_data(count: usize, query_count: usize) {

    let i = fake::vec![u32; count];
    let names = fake::vec![String as Name(EN); count];

    let it0 = i
        .into_iter()
        // .collect::<Vec<u32>>()
        ;
    let it1 = names
        .iter()
        .map(String::from)
        // .collect::<Vec<String>>()
        ;
    let bmap: BTreeMap<u32, String> = BTreeMap::from_iter(
        it0.zip(it1)
    );

    let indexes = fake::vec![u32; query_count];

    let rd = RandomData {
        map: bmap,
        indexes,
    };

    // Write as json file
    serde_json::to_writer(&File::create(format!("data_{}.json", count)).unwrap(), &rd).unwrap()

}

fn main() {

    let counts = [10, 25, 50, 100, 250, 500, 2000];
    let query_count = 20;
    for c in counts {
        println!("Generating data (with {c} entries), query_count: {query_count}");
        gen_data(c, query_count);
    }

}
