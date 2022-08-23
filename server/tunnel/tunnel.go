package tunnel

type Tunnel interface {
	Start()
}

type Kind int

const (
	HttpTunnelKind Kind = iota
	TcpTunnelKind
)
