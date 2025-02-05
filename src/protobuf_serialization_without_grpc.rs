use prost::Message; // For encoding and decoding
use std::fs::File;
use std::io::{Read, Write};

// Import the generated Protobuf module
mod person {
    include!(concat!(env!("OUT_DIR"), "/person.rs"));
}
use person::Person;

fn main() -> std::io::Result<()> {
    // Create a new Person message
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Serialize the Person message to bytes
    let mut encoded = Vec::new();
    person.encode(&mut encoded).unwrap();

    println!("Serialized Protobuf Message: {:?}", encoded);

    // Save to file
    let mut file = File::create("person.bin")?;
    file.write_all(&encoded)?;

    // Read from file and deserialize
    let mut file = File::open("person.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let decoded_person = Person::decode(&*buffer).unwrap();
    println!("Decoded Person: {:?}", decoded_person);

    Ok(())
}

