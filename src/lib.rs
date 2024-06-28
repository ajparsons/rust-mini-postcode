use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{WebAssembly,BigUint64Array};



#[wasm_bindgen]
pub fn get_memory() -> wasm_bindgen::JsValue {
    let memory = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
    let buffer = memory.buffer();
    buffer
}


#[wasm_bindgen]
pub fn allocate_vector(len: usize) -> *mut u64 {
    let mut vec: Vec<u64> = Vec::with_capacity(len);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent Rust from freeing the vector
    ptr
}

#[wasm_bindgen]
pub fn deallocate_vector(ptr: *mut u64, len: usize) {
    unsafe {
        let _vec = Vec::from_raw_parts(ptr, len, len); // Let Rust take ownership and deallocate
    }
}


const ZERO_U64: u64 = '0' as u64;
const A_U64: u64 = 'A' as u64;
const A_U64_LOWER: u64 = 'a' as u64;

fn char_to_base36(c: char) -> Option<u64> {
    match c {
        '0'..='9' => Some((c as u64) - ZERO_U64),
        'A'..='Z' => Some(10 + (c as u64) - A_U64),
        'a'..='z' => Some(10 + (c as u64) - A_U64_LOWER),
        _ => None,
    }
}

fn postcode_to_int(postcode: &str) -> u64 {
    // use approach from fulter_and_convert_postcodes for one postcode
    let mut current_number: u64 = 0;
    for c in postcode.chars() {
        if let Some(value) = char_to_base36(c) {
            current_number = current_number * 36 + value;
        }
    }
    current_number
}

fn filter_and_convert_postcodes(postcodes: &str, delimiter: char) -> Vec<u64> {
    let mut result = Vec::new();
    let mut current_number = 0u64;

    for c in postcodes.chars() {
        if c == delimiter {
            result.push(current_number);
            current_number = 0;
        } else if let Some(value) = char_to_base36(c) {
            current_number = current_number * 36 + value;
        }
    }

    // Add the last number if there is no trailing delimiter
    if current_number != 0 {
        result.push(current_number);
    }

    result
}

#[inline]
pub fn valid_postcode_int(base36_num: &u64) -> bool {
    // Function to check if a base36 number representation of a string
    // is a valid UK postcode

    // If more than seven digits - nope
    if *base36_num > 78364164095 {
        return false;
    }

    let u128_base36_num = *base36_num as u128;

    // Making a binary sequence of num or letter using bitwise operations
    let value: i32 = ((*base36_num % 36 >= 10) as i32) << 0
        | ((((u128_base36_num * 512409557603043100) >> 64) % 36 >= 10) as i32) << 1
        | ((((u128_base36_num * 14233598822306752) >> 64) % 36 >= 10) as i32) << 2
        | ((((u128_base36_num * 395377745064076) >> 64) % 36 >= 10) as i32) << 3
        | ((((u128_base36_num * 10982715140668) >> 64) % 36 >= 10) as i32) << 4
        | ((((u128_base36_num * 305075420574) >> 64) % 36 >= 10) as i32) << 5
        | ((((u128_base36_num * 8474317238) >> 64) % 36 >= 10) as i32) << 6;

    // Matches any of these patterns
    // AA9A9AA A9A9AA A99AA A999AA AA99AA AA999AA GL569EB AA999AA
    matches!(value, 19 | 35 | 43 | 51 | 99 | 107)
}

pub fn valid_postcode_int_division_based(base36_num: &u64) -> bool {
    // Function to check if a base36 number representation of a string
    // is a valid UK postcode

    // If more than seven digits - nope
    if *base36_num > 78364164095 {
        return false;
    }

    // Creating binary sequence of if num or letter using bitwise operations
    let value = ((*base36_num % 36 >= 10) as u8) << 0
        | (((*base36_num / 36) % 36 >= 10) as u8) << 1
        | (((*base36_num / 1296) % 36 >= 10) as u8) << 2
        | (((*base36_num / 46656) % 36 >= 10) as u8) << 3
        | (((*base36_num / 1679616) % 36 >= 10) as u8) << 4
        | (((*base36_num / 60466176) % 36 >= 10) as u8) << 5
        | (((*base36_num / 2176782336) % 36 >= 10) as u8) << 6;

    // Matches any of these patterns
    // AA9A9AA A9A9AA A99AA A999AA AA99AA AA999AA GL569EB AA999AA

    matches!(value, 19 | 35 | 43 | 51 | 99 | 107)
}

fn reverse_difference_compression(list_a: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::with_capacity(list_a.len());
    let mut last_value = 0;
    for &value in list_a {
        last_value += value;
        result.push(last_value);
    }
    result
}

fn reverse_drop_minus_one(list_a: &Vec<u16>, values_len: usize) -> Vec<usize> {
    let mut result = list_a[..2].to_vec();
    for &value in &list_a[2..] {
        if value == 0 {
            result.push(result[result.len() - 2]);
        } else {
            result.push(value);
        }
    }
    let converted: Vec<usize> = result.iter().map(|&x| x - 1).map(|x| x as usize).collect();
    // any values greater than the length of the values list are invalid and should be set to 0
    let converted = converted
        .iter()
        .map(|&x| if x >= values_len { 0 } else { x })
        .collect();
    // println!("{:?}", converted);
    converted
}

#[derive(Clone)]
pub struct BinaryStoredData<'a, const N: usize, const M: usize> {
    postcode_keys: [u64; N],
    value_key: [u16; N],
    value_values: [&'a str; M],
}

#[derive(Clone)]
pub struct StoredData {
    postcode_keys: Vec<u64>,
    value_key: Vec<u16>,
    value_values: Vec<String>,
}

impl<'a, const N: usize, const M: usize> From<BinaryStoredData<'a, N, M>> for StoredData {
    fn from(item: BinaryStoredData<'a, N, M>) -> Self {
        StoredData {
            postcode_keys: item.postcode_keys.to_vec(),
            value_key: item.value_key.to_vec(),
            value_values: item.value_values.iter().map(|s| s.to_string()).collect(),
        }
    }
}

// Declare the static data structure - include data.rs generated by build.rs
include! {concat!(env!("OUT_DIR"), "/data.rs")}
#[wasm_bindgen]
pub struct PostcodeRangeLookup {
    postcode_keys: Vec<u64>,
    value_key: Vec<usize>,
    value_values: Vec<String>,

}

impl From<StoredData> for PostcodeRangeLookup {
    fn from(item: StoredData) -> Self {
        PostcodeRangeLookup::new(
            reverse_difference_compression(&item.postcode_keys),
            reverse_drop_minus_one(&item.value_key, item.value_values.len()),
            item.value_values.clone()
        )
    }
}

#[wasm_bindgen]
impl PostcodeRangeLookup {
    fn new(postcode_keys: Vec<u64>, value_key: Vec<usize>, value_values: Vec<String>) -> Self {
        PostcodeRangeLookup {
            postcode_keys,
            value_key,
            value_values,
        }
    }

    pub fn get_value_values(&self) -> Vec<String> {
        self.value_values.clone()
    }

    pub fn from_binary_data() -> Self {
        let data: StoredData = DATA.clone().into();
        data.into()
    }

    pub fn get_values_str(
        &self,
        postcodes: &str,
        delimiter: char,
        check_valid_postcode: bool,
    ) -> String {
        // Filter and process the input string
        let filtered_postcodes = filter_and_convert_postcodes(postcodes, delimiter);

        // Process the parsed values and join the results
        // Create the resulting string directly using write! macro
        let average_length_per_value = 20;
        let mut result = String::with_capacity(filtered_postcodes.len() * average_length_per_value);

        for postcode_value in filtered_postcodes {
            match self.get_value_int(postcode_value, check_valid_postcode) {
                Some(value_index) => unsafe {
                    result.push_str(&self.value_values.get_unchecked(value_index));
                },
                None => {
                    result.push_str("");
                }
            }
            result.push(delimiter);
        }

        // Remove the trailing delimiter
        if !result.is_empty() {
            result.pop();
        }

        result
    }

    pub fn get_values_str_vec(
        &self,
        postcodes: Vec<String>,
        check_valid_postcode: bool,
    ) -> Vec<String> {
        // vector of postcode strings in
        // strings out
        postcodes
            .iter()
            .map(|postcode| {
                self.get_value(postcode, check_valid_postcode)
                    .unwrap_or_else(|| "".to_string())
            })
            .collect()
    }

    pub fn get_values_float_array(&self, postcodes: Vec<f64>) -> Vec<usize> {

        // convert floats to u64
        let postcodes: Vec<u64> = postcodes.iter().map(|x| *x as u64).collect();

        self.get_values_vec(postcodes, true)
    }

    pub fn get_values_vec_array(&self, postcodes: BigUint64Array) -> Vec<usize> {

        // convert floats to u64
        let postcodes = postcodes.to_vec();

        self.get_values_vec(postcodes, true)
    }

    pub fn get_values_vec(&self, postcodes: Vec<u64>, check_valid_postcode: bool) -> Vec<usize> {
        // vector of postcode ints in
        // ids out
        postcodes
            .iter()
            .map(|postcode| {
                self.get_value_int(*postcode, check_valid_postcode)
                    .unwrap_or_else(|| 0)
            })
            .collect()
    }

    pub fn get_value_int(&self, int_postcode: u64, check_valid_postcode: bool) -> Option<usize> {
        if check_valid_postcode && !valid_postcode_int(&int_postcode) {
            return None;
        }
        match self.postcode_keys.binary_search(&int_postcode) {
            // exact match found
            Ok(index) => {
                // get the value index
                Some(self.value_key[index])
            }
            // left place to insert
            Err(pos) => {
                if pos > 0 {
                    unsafe { Some(*self.value_key.get_unchecked(pos - 1)) }
                } else {
                    None
                }
            }
        }
    }

    pub fn get_value(&self, postcode: &str, check_valid_postcode: bool) -> Option<String> {
        // a postcode needs to just be upper case letters and numbers.
        // need to remove spaces and make it uppercase

        match self.get_value_int(postcode_to_int(postcode), check_valid_postcode) {
            None => None,
            Some(value_index) => unsafe {
                Some(self.value_values.get_unchecked(value_index).clone())
            },
        }
    }
}
