
# Rust gRPC implementation

Some sample code on working with gRPC using Rust as a client / server language. The Aim is to understand implementation details of the RPC / gRPC "protocols" for creating more performant microservices or IOT systems that use these remote calls for performant M2M communication,  

## RCPs - Remote Procedure Calls

A Remote Procedure Call (RPC) is a "protocol" that allows a program (the client) to cause a procedure (a function or method) to execute in a different address space (the server), ***without the programmer explicitly coding the details for the remote interaction.***

> The fundamental idea of RPC is to make network communication look and feel like a simple, **local function call.** RPC is not technically a protocol—it is better thought of as a general mechanism for structuring distributed systems. 

> [!NOTE]
> The term RPC refers to a type of protocol rather than a specific standard like TCP, so specific RPC protocols vary in the functions they perform. And, unlike TCP, which is the dominant reliable byte-stream protocol, there is no one dominant RPC protocol. Thus, in this section we will talk more about alternative design choices than previously.


<!-- Add the base RPC image -->
<p align="center">
  <img src="./images/BASE-RPC-DIAGRAM.svg" width="500" alt="Base RPC Image">
</p>

### Client Side 

When performing an RPC call, the client goes through some steps: 

1. **Procedure call**: Client app calls a **client stub** -> Local placeholder to the remote function, having the same signature as the method / function being called remotely.

2. **Marshalling (Serialization)**: The client stub takes the local function parameters and marshals (serializes) them into a format suitable for transmission over the network (e.g., a compact binary format like Protocol Buffers, or a text format like XML/JSON).

3. **Transmission:** The client's RPC runtime system sends the marshaled message, which includes the function ID and the parameters, to the server's machine.


### Server Side

The other half of the procedure call also goes through specifi steps

1. **Reception:** The server's RPC runtime system receives the message.

2. **Dispatch:** It passes the message to a server skeleton (sometimes called the dispatcher). The skeleton is responsible for locating the actual procedure on the server.

3. **Unmarshalling (Deserialization):** The server skeleton unmarshals (deserializes) the network message back into the ***server's native data types*** and formats, reconstructing the original parameters.

4. **Procedure Execution:** The server skeleton calls the actual remote procedure using the reconstructed parameters.

5. **Return and Reverse Flow:** When the remote procedure completes, the result is marshalled and sent back to the client, following the same steps in reverse (unmarshalling by the client stub, and the client application receives the result).

### Interface Definition Language (IDL)
 RPC frameworks require an IDL (like Protocol Buffers) to formally define the contract (the function names, parameters, and return types) between the client and the server.

## gRCP - gRPC Remote Procedure Calls

As in many RPC systems, gRPC is based around the idea of defining a service, specifying the methods that can be called remotely with their parameters and return types.

On the server side, the server implements this interface and runs a gRPC server to handle client calls. On the client side, the client has a stub (referred to as just a client in some languages) that provides the same methods as the server.

> ![TIP]
> The latest Google APIs will have gRPC versions of their interfaces, letting you easily build Google functionality into your applications.

By default, gRPC uses [**protocol buffers**](https://protobuf.dev/overview/), Google’s mature open source mechanism for serializing structured data (although it can be used with other data formats such as JSON).

### Protocol Buffers

> It’s like JSON, except it’s smaller and faster, and it generates native language bindings. You define how you want your data to be structured once, then you can use special generated source code to easily write and read your structured data to and from a variety of data streams and using a variety of languages.

The first step when working with protocol buffers is to define the structure for the data you want to serialize in a proto file. Protocol buffer ***messages and services*** are described by engineer-authored `.proto` files. 

```proto
message Person {
  string name = 1;
  int32 id = 2;
  bool has_car = 3;
}
```

The proto compiler is invoked at build time on `.proto` files to generate code. Each generated class contains simple accessors for each field and methods to serialize and parse the whole structure to and from raw bytes.

## Rust Implementation

Following the previosly exposed theory behind RPC and gRPC, we can proceed with the definition of our RUST-based implementation

We need to:
- Define the application gRPC service 
- Define our application Method
- Define our request and response types using protocol Buffers

The service will provide:
- An RPC method called **GetWeather** that the server will implement
- The **Point** and **Weather** data structs that will be exchanged between the server and the client when **GetWeather** is called
  - The client will provide a **Point** data struct in the request, and the client will respond with a **Weather** data struct. 



