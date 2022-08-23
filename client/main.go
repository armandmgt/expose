package main

import (
	"context"
	"fmt"
	"github.com/armandmgt/expose/assets"
	"github.com/armandmgt/expose/server/tunnelService"
	"golang.org/x/oauth2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/oauth"
	"log"
	"time"
)

func main() {
	clientOpts := parseArgs()

	perRPC := oauth.NewOauthAccess(fetchToken())
	creds, err := credentials.NewClientTLSFromFile(assets.Path("x509/server.crt"), "")
	if err != nil {
		log.Fatalf("failed to load credentials: %v", err)
	}
	opts := []grpc.DialOption{
		grpc.WithPerRPCCredentials(perRPC),
		grpc.WithTransportCredentials(creds),
	}

	conn, err := grpc.Dial(fmt.Sprintf("%s:%d", clientOpts.ServerName, clientOpts.ServerPort), opts...)
	if err != nil {
		log.Printf("did not connect: %v", err)
		return
	}
	defer func(conn *grpc.ClientConn) {
		_ = conn.Close()
	}(conn)
	rgc := tunnelService.NewTunnelServiceClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	client, err := rgc.NewClient(ctx, &tunnelService.NewClientRequest{})
	if err != nil {
		log.Printf("rgc.NewClient(_) = _, %v: ", err)
		return
	}
	fmt.Println("NewClient: ", client.UUID)
	//quit := make(chan os.Signal, 1)
	//signal.Notify(quit, os.Interrupt)
	//for {
	//	select {
	//	case <-quit:
	//		close(quit)
	//		return
	//	}
	//	if _, err = rgc.Alive(context.Background(), &tunnelService.AliveMessage{ClientUUID: client.UUID}); err != nil {
	//		return
	//	}
	//}
}

// fetchToken simulates a token lookup and omits the details of proper token
// acquisition.
func fetchToken() *oauth2.Token {
	return &oauth2.Token{
		AccessToken: "some-secret-token",
	}
}
