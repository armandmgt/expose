// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.21.5
// source: server/tunnelService/tunnelService.proto

package tunnelService

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// TunnelServiceClient is the client API for TunnelService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type TunnelServiceClient interface {
	NewClient(ctx context.Context, in *NewClientRequest, opts ...grpc.CallOption) (*NewClientReply, error)
	Alive(ctx context.Context, in *AliveMessage, opts ...grpc.CallOption) (*AliveReply, error)
	NewTunnel(ctx context.Context, in *NewTunnelRequest, opts ...grpc.CallOption) (*NewTunnelReply, error)
}

type tunnelServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewTunnelServiceClient(cc grpc.ClientConnInterface) TunnelServiceClient {
	return &tunnelServiceClient{cc}
}

func (c *tunnelServiceClient) NewClient(ctx context.Context, in *NewClientRequest, opts ...grpc.CallOption) (*NewClientReply, error) {
	out := new(NewClientReply)
	err := c.cc.Invoke(ctx, "/tunnelService.TunnelService/NewClient", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *tunnelServiceClient) Alive(ctx context.Context, in *AliveMessage, opts ...grpc.CallOption) (*AliveReply, error) {
	out := new(AliveReply)
	err := c.cc.Invoke(ctx, "/tunnelService.TunnelService/Alive", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *tunnelServiceClient) NewTunnel(ctx context.Context, in *NewTunnelRequest, opts ...grpc.CallOption) (*NewTunnelReply, error) {
	out := new(NewTunnelReply)
	err := c.cc.Invoke(ctx, "/tunnelService.TunnelService/NewTunnel", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// TunnelServiceServer is the server API for TunnelService service.
// All implementations must embed UnimplementedTunnelServiceServer
// for forward compatibility
type TunnelServiceServer interface {
	NewClient(context.Context, *NewClientRequest) (*NewClientReply, error)
	Alive(context.Context, *AliveMessage) (*AliveReply, error)
	NewTunnel(context.Context, *NewTunnelRequest) (*NewTunnelReply, error)
	mustEmbedUnimplementedTunnelServiceServer()
}

// UnimplementedTunnelServiceServer must be embedded to have forward compatible implementations.
type UnimplementedTunnelServiceServer struct {
}

func (UnimplementedTunnelServiceServer) NewClient(context.Context, *NewClientRequest) (*NewClientReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method NewClient not implemented")
}
func (UnimplementedTunnelServiceServer) Alive(context.Context, *AliveMessage) (*AliveReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Alive not implemented")
}
func (UnimplementedTunnelServiceServer) NewTunnel(context.Context, *NewTunnelRequest) (*NewTunnelReply, error) {
	return nil, status.Errorf(codes.Unimplemented, "method NewTunnel not implemented")
}
func (UnimplementedTunnelServiceServer) mustEmbedUnimplementedTunnelServiceServer() {}

// UnsafeTunnelServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TunnelServiceServer will
// result in compilation errors.
type UnsafeTunnelServiceServer interface {
	mustEmbedUnimplementedTunnelServiceServer()
}

func RegisterTunnelServiceServer(s grpc.ServiceRegistrar, srv TunnelServiceServer) {
	s.RegisterService(&TunnelService_ServiceDesc, srv)
}

func _TunnelService_NewClient_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(NewClientRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TunnelServiceServer).NewClient(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/tunnelService.TunnelService/NewClient",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TunnelServiceServer).NewClient(ctx, req.(*NewClientRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _TunnelService_Alive_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AliveMessage)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TunnelServiceServer).Alive(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/tunnelService.TunnelService/Alive",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TunnelServiceServer).Alive(ctx, req.(*AliveMessage))
	}
	return interceptor(ctx, in, info, handler)
}

func _TunnelService_NewTunnel_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(NewTunnelRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TunnelServiceServer).NewTunnel(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/tunnelService.TunnelService/NewTunnel",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TunnelServiceServer).NewTunnel(ctx, req.(*NewTunnelRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// TunnelService_ServiceDesc is the grpc.ServiceDesc for TunnelService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var TunnelService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "tunnelService.TunnelService",
	HandlerType: (*TunnelServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "NewClient",
			Handler:    _TunnelService_NewClient_Handler,
		},
		{
			MethodName: "Alive",
			Handler:    _TunnelService_Alive_Handler,
		},
		{
			MethodName: "NewTunnel",
			Handler:    _TunnelService_NewTunnel_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "server/tunnelService/tunnelService.proto",
}
