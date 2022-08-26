package main

import (
	"crypto/tls"
	"fmt"
	"github.com/armandmgt/expose/assets"
	pb "github.com/armandmgt/expose/server/tunnelService"
	log "github.com/sirupsen/logrus"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"net"
	"os"
	"os/signal"
	"sync"
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
	tunnelServer := NewTunnelServer()
	pb.RegisterTunnelServiceServer(s, tunnelServer)
	lis, err := net.Listen("tcp", fmt.Sprintf("proxy.armandmgt.me:%d", uint16(proxyOpts.Port)))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	log.Infof("Listening on port %d", proxyOpts.Port)
	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		if err := s.Serve(lis); err != nil {
			log.Fatalf("failed to serve: %v", err)
		}
		wg.Done()
	}()
	handleAndClean(s, tunnelServer)
	wg.Wait()
}

func handleAndClean(s *grpc.Server, tunnelServer *server) {
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt)
	<-quit
	tunnelServer.ClientHandler.Stop()
	s.Stop()
	close(quit)
}
