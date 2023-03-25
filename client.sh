grpcurl -proto grpc/fcbl-core.proto -d '{"name":"foo"}' -plaintext localhost:50051 fcbl_core.UserService.new_core
