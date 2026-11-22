// src/proto/mod.rs
#![allow(clippy::all)]
 
pub mod auth {
    pub mod v1 {
        tonic::include_proto!("auth.v1");
    }
}

pub mod users {
    pub mod v1 {
        tonic::include_proto!("users.v1");
    }
}
 