use tonic::transport::Channel;
use crate::proto::auth::v1::auth_client::AuthClient;

fn main() {
    let grpc_host = std::env::var("GRPC_HOST").unwrap();
    let grpc_port = std::env::var("GRPC_PORT").unwrap();
    let channel = Channel::from_shared(format!("http://{}:{}", grpc_host, grpc_port)).unwrap();
    let client = AuthClient::new(channel);

}
