use fukutsu_cobol::data::data::CobolCore;
use log::info;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

mod fcbl_core {
    tonic::include_proto!("fcbl_core");
}

//use crate::grpc::main::fcbl_core::user_service_server::UserService;
use fcbl_core::{
    user_service_server::{UserService, UserServiceServer},
    Core, Field, NewCore, RegisterField,
};

pub struct MyUserService {
    cores: Arc<Mutex<Vec<CobolCore>>>,
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn new_core(&self, request: Request<NewCore>) -> Result<Response<Core>, Status> {
        info!("New_core, Got a request: {:?}", request);
        let mut cores = self.cores.lock().unwrap();
        let new_id = cores.len() as i32;
        cores.push(CobolCore::new(new_id));
        let reply = fcbl_core::Core { id: new_id };
        Ok(Response::new(reply))
    }

    async fn register_field(
        &self,
        request: Request<RegisterField>,
    ) -> Result<Response<Field>, Status> {
        info!("[dbg] register_field, Got a request: {:?}", request);
        let reply = fcbl_core::Field { id: 0 };
        Ok(Response::new(reply))
    }
}

impl Default for MyUserService {
    fn default() -> Self {
        Self {
            cores: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[tokio::main]
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let allow_cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any);
    let user_service = MyUserService::default();

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(allow_cors)
        .layer(GrpcWebLayer::new())
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
