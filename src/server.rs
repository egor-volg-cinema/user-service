use std::net::SocketAddr;
use std::sync::Arc;

use tonic::transport::Server;

use crate::grpc::user_service::UserService;
use crate::proto::users::v1::users_service_server::UsersServiceServer;
use crate::repository::db::Database;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(serve())
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let grpc_host = std::env::var("GRPC_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let grpc_port = std::env::var("GRPC_PORT").unwrap_or_else(|_| "50051".into());
    let db_url = std::env::var("POSTGRES_URI")
        .unwrap_or_else(|_| "host=localhost user=postgres dbname=library".into());

    let (db, connection) = Database::new(&db_url).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Ошибка соединения: {}", e);
        }
    });

    db.init().await?;

    let addr: SocketAddr = format!("{grpc_host}:{grpc_port}").parse()?;
    println!("gRPC server listening on {addr}");

    Server::builder()
        .add_service(UsersServiceServer::new(UserService::new(Arc::new(db))))
        .serve(addr)
        .await?;

    Ok(())
}
