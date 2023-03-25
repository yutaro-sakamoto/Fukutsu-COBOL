use tonic::{transport::Server, Request, Response, Status};

mod fcbl_core {
    tonic::include_proto!("fcbl_core");
}

//use crate::grpc::main::fcbl_core::user_service_server::UserService;
use fcbl_core::{
    user_service_server::{UserService, UserServiceServer},
    Core, Field, NewCore, RegisterField,
};

#[derive(Default)]
pub struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn new_core(&self, request: Request<NewCore>) -> Result<Response<Core>, Status> {
        println!("[dbg] new_core, Got a request: {:?}", request);
        let reply = fcbl_core::Core { id: 0 };
        Ok(Response::new(reply))
    }

    async fn register_field(
        &self,
        request: Request<RegisterField>,
    ) -> Result<Response<Field>, Status> {
        println!("[dbg] register_field, Got a request: {:?}", request);
        let reply = fcbl_core::Field { id: 0 };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let user = MyUserService::default();

    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
