use fukutsu_cobol::data::data::CobolCore;
use log::info;
use std::collections::HashMap;
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
    name_id_map: Arc<Mutex<HashMap<String, i32>>>,
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn new_core(&self, request: Request<NewCore>) -> Result<Response<Core>, Status> {
        info!("New_core, Got a request: {:?}", request);
        let mut cores = self.cores.lock().unwrap();
        let mut name_id_map = self.name_id_map.lock().unwrap();
        if let Some(id) = name_id_map.get(&request.get_ref().name.to_string()) {
            return Ok(Response::new(Core { id: *id }));
        } else {
            let new_id = cores.len() as i32;
            cores.push(CobolCore::new_by_string(request.get_ref().name.to_string()));
            name_id_map.insert(request.get_ref().name.clone(), new_id);
            Ok(Response::new(fcbl_core::Core { id: new_id }))
        }
    }

    async fn register_field(
        &self,
        request: Request<RegisterField>,
    ) -> Result<Response<Field>, Status> {
        let args = request.get_ref();
        info!("register_field, Got a request: {:?}", request);
        let mut cores = self.cores.lock().unwrap();
        if let Some(core) = cores.get_mut(args.core as usize) {
            let field_id = core.register_field(
                args.start_index,
                args.len,
                args.typ as u8,
                args.digits,
                args.scale,
                args.flags as u8,
                args.pic.clone(),
            );
            return Ok(Response::new(fcbl_core::Field {
                id: field_id as i32,
            }));
        } else {
            return Err(Status::invalid_argument("Core not found"));
        }
    }
}

impl Default for MyUserService {
    fn default() -> Self {
        Self {
            cores: Arc::new(Mutex::new(Vec::new())),
            name_id_map: Arc::new(Mutex::new(HashMap::new())),
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
