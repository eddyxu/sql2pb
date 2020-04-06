fn main() {
    prost_build::compile_protos(&["proto/ast.proto"], &["proto"]).unwrap();
}
