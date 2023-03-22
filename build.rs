extern crate lalrpop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    lalrpop::process_root().unwrap();
    tonic_build::compile_protos("grpc/fcbl-core.proto")?;
    Ok(())
}
