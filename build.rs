#[allow(dead_code)]

// build.rs
use std::env;
use std::io::Write;
use std::fs;
use std::path::Path;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct StoredData {
    postcode_keys: Vec<u64>,
    value_key: Vec<u16>,
    value_values: Vec<String>,
}

fn main() {
    // Specify the path to your JSON file
    let json_path = Path::new("pcon.json");

    let data = std::fs::read_to_string(json_path).unwrap();
    // print the data
    let data: StoredData = match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to parse JSON data: {}", err);
            std::process::exit(1);
        }
    };

    // Generate the Rust code
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");

    let mut file = fs::File::create(&dest_path)
        .expect("Unable to create data.rs");

    // Serialize the data structure into Rust code
    writeln!(file, "pub static DATA: BinaryStoredData::<{0:?},{1:?}> = BinaryStoredData::<{0:?},{1:?}> {{
        postcode_keys: {2:?},
        value_key: {3:?},
        value_values: {4:?}
            }};", data.postcode_keys.len(),
                  data.value_values.len(),
                  data.postcode_keys,
                  data.value_key,
                  data.value_values)
    .expect("Unable to write to data.rs");
}
