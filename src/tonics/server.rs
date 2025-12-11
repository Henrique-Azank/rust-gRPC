// Binary for testing out a tonic hello-world server / service

// Base dependencies
use std::net::SocketAddr;

// Third party dependencies
use tonic::{Request, Response, Status, transport::Server};

// Include the generated protobuf code
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// Import the generated gRPC service and message types
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

// Define the struct that will implement the gRPC service
#[derive(Debug, Default)]
pub struct MyGreeter {}

// Implement the gRPC service defined in the proto file
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // Define an async function to handle the say_hello RPC
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        // Instantiate the reply message
        let reply = HelloReply {
            // We must use .into_inner() as the fields of gRPC requests
            // and responses are private
            message: format!("Hello {}!", request.into_inner().name),
        };

        // Send back our formatted greeting
        Ok(Response::new(reply))
    }
}

// Main script entrypoint
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address and port for the server to listen on
    let addr: SocketAddr = "[::1]:50051".parse()?;

    // Create an instance of our gRPC service
    let greeter: MyGreeter = MyGreeter::default();

    // Buuild and run the gRPC server
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    // End the process
    Ok(())
}
