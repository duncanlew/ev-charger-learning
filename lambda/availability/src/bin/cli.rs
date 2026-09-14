use availability::check_availability;

fn main() {
    let result = check_availability();

    // Output with debug formatting
    println!("Output with debug {:?}", result);

    // Output with per field output
    println!(
        "Output with per field output, available: {}",
        result.available
    );
    println!("Output with per field output, message: {}", result.message);

    // Output with json
    let json_output = serde_json::to_string(&result).unwrap();
    println!("Output with json: {}", json_output);

    // Output with json pretty
    let json_pretty = serde_json::to_string_pretty(&result).unwrap();
    println!("Output with json pretty: {}", json_pretty);
}
