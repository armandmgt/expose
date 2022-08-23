package main

import (
	"context"
	"errors"
	"fmt"
	"github.com/armandmgt/expose/server/clients"
	"github.com/armandmgt/expose/server/tunnel"
	"github.com/armandmgt/expose/server/tunnelService"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"log"
)

type server struct {
	Clients map[string]*clients.Client
	tunnelService.UnimplementedTunnelServiceServer
}

func NewTunnelServer() *server {
	return &server{Clients: make(map[string]*clients.Client)}
}

type NewTunnelArgs struct {
	ClientUUID string
	Kind       tunnel.Kind
}

type NewTunnelReply struct {
	Address string
}

func (s *server) NewClient(_ context.Context, _ *tunnelService.NewClientRequest) (reply *tunnelService.NewClientReply, err error) {
	clientsReg := s.Clients
	client := clients.NewClient()
	clientsReg[client.UUID] = client

	log.Println("TunnelService.NewClient: new client created ", clientsReg[client.UUID])
	reply = &tunnelService.NewClientReply{UUID: client.UUID}
	return
}

func (s *server) Alive(_ context.Context, args *tunnelService.AliveMessage) (_ *tunnelService.AliveReply, err error) {
	client, err := getClient(s, args.ClientUUID)
	if err != nil {
		return nil, status.Error(codes.InvalidArgument, err.Error())
	}
	if err = client.ExtendLife(); err != nil {
		return nil, status.Error(codes.InvalidArgument, err.Error())
	}
	return
}

func (s *server) NewTunnel(_ context.Context, args *tunnelService.NewTunnelRequest) (reply *tunnelService.NewTunnelReply, err error) {
	reply = &tunnelService.NewTunnelReply{}
	client, err := getClient(s, args.ClientUUID)
	if err != nil {
		return nil, status.Error(codes.InvalidArgument, err.Error())
	}
	switch args.Kind {
	case tunnelService.TunnelKind_HTTP_TUNNEL:
		client.Tunnel = tunnel.NewHTTP()
	case tunnelService.TunnelKind_TCP_TUNNEL:
		client.Tunnel, err = tunnel.NewTCP()
		if err != nil {
			return nil, status.Error(codes.ResourceExhausted, err.Error())
		}
		client.Tunnel.Start()
		reply.Address = client.Tunnel.(*tunnel.TcpTunnel).Address
	}
	log.Println("TunnelService.NewTunnel: new tunnel created ", client)
	return
}

func getClient(s *server, uuid string) (client *clients.Client, err error) {
	client, ok := s.Clients[uuid]
	if !ok {
		return nil, errors.New(fmt.Sprintf("client %s not found", uuid))
	}
	return
}
