use std::env;

use mini_postcode::PostcodeRangeLookup;

fn main() {
    // Get the command line argument for the postcode
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a postcode as an argument");
        std::process::exit(1);
    }
    let postcode = &args[1];

    let lookup = PostcodeRangeLookup::from_binary_data();

    // get the value for the given postcode
    let value = lookup.get_values_str(postcode, ',', true);

    println!("{}", value)
}
