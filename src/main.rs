#[path = "../proto/mod.rs"]
mod proto;
mod grpc;
mod repository;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run()
}
