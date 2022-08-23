.PHONY: tunnel_service compile_protobuf gen_certs compile

LOCAL_SERVER_NAME := proxy.armandmgt.me
PROTOC_GEN_GO := $(GOPATH)/bin/protoc-gen-go

# If $GOPATH/bin/protoc-gen-go does not exist, we'll run this command to install
# it.
$(PROTOC_GEN_GO):
	go get -u github.com/golang/protobuf/protoc-gen-go

server/tunnelService/tunnelService.pb.go server/tunnelService/tunnelService_grpc.pb.go: server/tunnelService/tunnelService.proto
	protoc --go_out=. --go_opt=paths=source_relative \
           --go-grpc_out=. --go-grpc_opt=paths=source_relative \
           server/tunnelService/tunnelService.proto

tunnel_service: server/tunnelService/tunnelService.pb.go server/tunnelService/tunnelService_grpc.pb.go $(PROTOC_GEN_GO)

compile_protobuf: tunnel_service

assets/x509/server.crt assets/x509/server.key: assets/x509/gen.sh
	cd assets/x509 && ./gen.sh $(LOCAL_SERVER_NAME)

gen_certs: assets/x509/server.crt assets/x509/server.key

compile: gen_certs compile_protobuf
