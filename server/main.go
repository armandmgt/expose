package main

import (
	"crypto/tls"
	"fmt"
	"github.com/armandmgt/expose/assets"
	pb "github.com/armandmgt/expose/server/tunnelService"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"log"
	"net"
)

func main() {
	proxyOpts := parseArgs()

	cert, err := tls.LoadX509KeyPair(assets.Path("x509/server.crt"), assets.Path("x509/server.key"))
	if err != nil {
		log.Fatalf("failed to load key pair: %s", err)
	}
	opts := []grpc.ServerOption{
		// The following grpc.ServerOption adds an interceptor for all unary
		// RPCs. To configure an interceptor for streaming RPCs, see:
		// https://godoc.org/google.golang.org/grpc#StreamInterceptor
		grpc.UnaryInterceptor(ensureValidToken),
		// Enable TLS for all incoming connections.
		grpc.Creds(credentials.NewServerTLSFromCert(&cert)),
	}
	s := grpc.NewServer(opts...)
	pb.RegisterTunnelServiceServer(s, NewTunnelServer())
	lis, err := net.Listen("tcp", fmt.Sprintf("proxy.armandmgt.me:%d", uint16(proxyOpts.Port)))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	log.Printf("Listening on port %d\n", proxyOpts.Port)
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
