use criterion::{criterion_group, criterion_main, Criterion};
use mini_postcode::{valid_postcode_int, valid_postcode_int_division_based, PostcodeRangeLookup};
use std::error::Error;
use std::fs::File;
use csv::Reader;

use std::io::prelude::*;

fn read_postcodes_from_csv(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut postcodes = Vec::new();
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    for result in reader.records() {
        let record = result?;
        let postcode = record.get(0).ok_or("Invalid CSV format")?;
        postcodes.push(postcode.to_string());
    }
    Ok(postcodes)
}

fn read_postcodes_from_csv_as_string(file_path: &str) -> String {
    // want just one big string of the entire contents of the file
    // newlines and all
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents
}

fn execute_batch(func: &dyn Fn(&u64) -> bool, postcodes: &Vec<u64>) -> Vec<bool> {
    postcodes.iter().map(|postcode| func(postcode)).collect()
}

fn execute_batch_str_warm_lookup(lookup: &PostcodeRangeLookup, postcodes: &Vec<String>) -> Vec<Option<String>> {
    postcodes.iter().map(|postcode| lookup.get_value(postcode, true)).collect()
}

fn execute_batch_str(postcodes: &Vec<String>) -> Vec<Option<String>> {
    let lookup: PostcodeRangeLookup = PostcodeRangeLookup::from_binary_data();
    postcodes.iter().map(|postcode| lookup.get_value(postcode, true)).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let postcodes = read_postcodes_from_csv("/workspaces/learn-rust/benches/valid_postcodes.csv")
        .expect("Failed to read postcodes from CSV");

    let str_postcodes = read_postcodes_from_csv_as_string("/workspaces/learn-rust/benches/valid_postcodes.csv");

    let postcode_values: Vec<u64> = postcodes
    .iter()
    .map(|postcode| {
        let postcode = postcode.replace(" ", "").to_uppercase();
        u64::from_str_radix(&postcode, 36).unwrap()
    })
    .collect();

    let mut main_group = c.benchmark_group("postcode-converter");
    main_group.sample_size(50);

    let lookup: PostcodeRangeLookup = PostcodeRangeLookup::from_binary_data();
    main_group.bench_function("postcode_lookup_warm", |b| {
        b.iter(|| execute_batch_str_warm_lookup(&lookup, &postcodes))
    });

    main_group.bench_function("postcode_lookup_just_values", |b| {
        b.iter(||     lookup.get_values_vec(postcode_values.clone(), true)
    )
    });

    main_group.bench_function("postcode_lookup_big_string", |b| {
        b.iter(|| lookup.get_values_str(&str_postcodes, '\n', true))
    });

    main_group.bench_function("postcode_lookup", |b| {
        b.iter(|| execute_batch_str(&postcodes))
    });


    main_group.finish();


    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("postcode-converter");
    group.sample_size(10000);


    group.bench_function("valid_postcode_int", |b| {
        b.iter(|| execute_batch(&valid_postcode_int, &postcode_values))
    });

    group.bench_function("valid_postcode_int_division_based", |b| {
        b.iter(|| execute_batch(&valid_postcode_int_division_based, &postcode_values))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
