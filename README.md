
# Rust gRPC implementation

Some sample code on working with gRPC using Rust as a client / server language

## RCPs - Remote Procedure Calls

A Remote Procedure Call (RPC) is a protocol that allows a program (the client) to cause a procedure (a function or method) to execute in a different address space (the server), ***without the programmer explicitly coding the details for the remote interaction.***

> The fundamental idea of RPC is to make network communication look and feel like a simple, **local function call.**

### Client Side 

When performing an RPC call, the client goes through some steps: 

1. **Procedure call**: Client app calls a **client stub** -> Local placeholder to the remote function, having the same signature as the method / function being called remotely.

2. 
