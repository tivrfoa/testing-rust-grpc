fn main() {
    tonic_build::compile_protos("proto/hello.proto").unwrap();

    prost_build::compile_protos(&["src/person.proto"], &["src/"]).unwrap();
    // prost_build::compile_protos(&["proto/person.proto"], &["proto/"]).unwrap();
    // prost_build::compile_protos(&["proto/person.proto"], &["proto"]).unwrap();
    //
    // Configure `prost_build`
    // let mut config = prost_build::Config::new();
    // config.out_dir("generated_code"); // Output directory for generated files

    // Compile the `.proto` file
    // config
    //     .compile_protos(&["proto/person.proto"], &["proto/"])
    //     .unwrap();
}
