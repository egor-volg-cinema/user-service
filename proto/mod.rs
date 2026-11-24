// src/proto/mod.rs
#![allow(clippy::all)]
 
pub mod account {
    pub mod v1 {
        tonic::include_proto!("account.v1");
    }
}

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
 