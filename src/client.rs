use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

use prost::Message;

use std::fs::File;
use std::io::Write;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let hello_request = HelloRequest {
        name: "Tonic".into(),
    };
    let encoded_message = hello_request.encode_to_vec();
    println!("{:02x?}", encoded_message);

    // gRPC requires a 5-byte prefix: 1 byte for compression flag (0 = none), 4 bytes for message length
    let mut grpc_frame = vec![0u8]; // First byte: compression flag (0 = no compression)
    grpc_frame.extend_from_slice(&(encoded_message.len() as u32).to_be_bytes()); // 4-byte length
    grpc_frame.extend_from_slice(&encoded_message); // Append the actual serialized message

    // Print the full message in hex (useful for debugging)
    println!("gRPC Encoded Message (Hex): {:02X?}", grpc_frame);

    let mut file = File::create("m4")?;
    file.write_all(&grpc_frame)?;

    // Print in base64 (needed for curl)
    let base64_message = base64::encode(&grpc_frame);
    println!("gRPC Encoded Message (Base64): {}", base64_message);

    let request = tonic::Request::new(hello_request);


    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
