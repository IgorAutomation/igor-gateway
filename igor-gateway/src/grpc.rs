use crate::grpc::hello_world::greeter_server::Greeter;
use crate::grpc::hello_world::{HelloReply, HelloRequest};
use sqlx::SqlitePool;
use tonic::{Request, Response, Status};
use tracing::debug;

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}
#[derive(Debug)]
pub struct MyGreeter {
    pool: SqlitePool,
}

impl MyGreeter {
    pub fn new(pool: SqlitePool) -> Self {
        MyGreeter { pool }
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        debug!(
            "Got a request: {:?}, DbClosed?: {}",
            request,
            self.pool.is_closed()
        );

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
