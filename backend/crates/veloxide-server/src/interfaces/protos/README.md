# Protocol Buffers

```sh
grpcurl -plaintext -import-path . -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:8080' helloworld.Greeter/SayHello 
```

