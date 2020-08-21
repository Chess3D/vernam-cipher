use rand::distributions::{Distribution, Uniform};


fn main() {
    let otp = generate_otp(100);
    let msg = "Running out of RAM? Go to downloadmoreram.com to download more today!";

    let encoded = encode_str(msg, &otp);
    let decoded = decode_str(&encoded, &otp);


    println!("\n\nOTP: {}", otp);
    println!("MSG: {}", msg);

    println!("\nENCODED: {}", encoded);
    println!("DECODED: {}\n\n", decoded);
}


// Generates a one time pad of a desired length
fn generate_otp(length: usize) -> String {

    // Uses a periodically reseeded CSPRNG to generate random numbers
    let mut rng = rand::thread_rng();

    // Insures all integers within the range [32, 126] are equally likely
    let range = Uniform::from(32..127);
    
    // Creates an empty string with the capacity to store the one time pad
    let mut otp = String::with_capacity(length);

    // Loops until the desired length for the one time pad has been reached
    for _i in 0..length {
        
        // Generates a random integer within the range [32, 126]
        let value: u8 = range.sample(&mut rng);

        // Adds a new character to the end of the one time pad
        otp.push(value as char);
    }

    // Returns the generated one time pad
    otp
}


// Encodes a character using a given key 
fn encode_char(input: char, key: char) -> char {
    
    // Start encoded at 0 to prevent a possible overflow
    let mut encoded: u8 = 0;
    
    // Constrains input values to [0, 94]
    // Adds the constrained input value to the encoded value
    encoded += (input as u8) - 32;

    // Constrains key values to [0, 94]
    // Adds the constrained key value to the encoded value
    encoded += (key as u8) - 32;

    // Constrains possible values to [0, 94]
    // Insures range of the encoded and input values are equivalent
    encoded %= 95;

    // Shifts possible values to [32, 126]
    // Allows characters to represent all possible values
    encoded += 32;

    // Returns the encoded value as a character
    encoded as char
}


// Encodes a string using a given otp
fn encode_str(input: &str, otp: &str) -> String {

    // 
    let mut encoded: String = String::with_capacity( input.len() );

    let itr = input.chars().zip( otp.chars() );

    for pair in itr {
        encoded.push( encode_char(pair.0, pair.1) )
    }

    encoded
}


// Decodes a character using a given key
fn decode_char(input: char, key: char) -> char {
    
    // Start decoded at 95 to prevent a possible underflow
    let mut decoded: u8 = 95;

    // Finds the difference between the input and key values
    decoded += input as u8;
    decoded -= key as u8;

    // Constrains possible values to [0, 94]
    // Removes underflow buffer if needed
    decoded %= 95;

    // Shifts possible values to [32, 126]
    // Allows characters to represent all possible values
    decoded += 32;

    // Returns the decoded value as a character
    decoded as char
}


fn decode_str(input: &str, otp: &str) -> String {

    // 
    let mut decoded: String = String::with_capacity( input.len() );

    let itr = input.chars().zip( otp.chars() );

    for pair in itr {
        decoded.push( decode_char(pair.0, pair.1) )
    }

    decoded
}