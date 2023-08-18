# Veloxide Contracts

In this folder you can find protobuf files that act as contracts between the frontend and backend.

## Local testing

You can test the backend server is functioning by using grpcurl.

```sh
grpcurl -plaintext -import-path . -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:8080' helloworld.Greeter/SayHello 
```

## Generating code

Code can be generated from these protobuf files by running `just gen` in the root directory of the repo.

