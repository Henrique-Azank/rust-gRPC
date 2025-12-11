// Binary for testing out a tonic hello-world server / service

// Third party dependencies
use tonic::{Request, Response, transport::Channel};

// Include the generated protobuf code
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// Generated client implementations
use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloReply, HelloRequest};

// Main client entrypoint
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiate the gRPC client
    let mut client: GreeterClient<Channel> = GreeterClient::connect("http://[::1]:50051").await?;

    // Create the request message
    let request = Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    // Make the gRPC call
    let response: Response<HelloReply> = client.say_hello(request).await?;

    // Print out the response
    println!("RESPONSE={:?}", response);

    // End the process
    Ok(())
}
